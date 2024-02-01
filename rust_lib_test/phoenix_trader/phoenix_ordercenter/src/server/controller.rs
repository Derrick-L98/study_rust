use std::{sync::Arc, ops::AddAssign};
use anyhow::Result;
// use chrono::{Datelike, Timelike};
use tokio::sync::{broadcast, mpsc, RwLock};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use common::{
    calcfee::FeeDetail,
    akaclient::AkaClient,
    uidservice::UidgenService,
    redisclient::redisclient::RedisClient,
    constant::{OrderStatus, ChannelType, OrderDirection},
    logclient::LogClient,
};
use messagecenter::notificationclient::NotificationClient;
use crate::{
    server::service::ordercache::CacheKey, 
    config::settings::Settings, 
};
use crate::server::service::notificationmsg;
use crate::server::service::convertdata;
use crate::server::service::assetchange;
use crate::server::service::queryinfo;

use crate::protofiles::{
    OrderReq, CancelReq, OrderResp,
    RouterMsg, MsgType, ExecType, ExecMsg,
    PhoenixRiskCheckInfo, 
    ReplenishOrderReq, 
    // ReplenishOrderResp
};
use crate::client::{
    DbClient,
    // AkaCenterClient,
    RiskCenterClient,
    AssetsCenterClient,
    AccountRiskClient,
};

use crate::dataservice::entities::{
    prelude::PhoenixOrdSuborder, 
    prelude::PhoenixOrdStockorder,
    prelude::PhoenixOrdStockdeal,
    prelude::PhoenixOmsTradeconfig,
    prelude::PhoenixOrdPendSettle,
};
use crate::common::common::OrderDetail;

pub enum PersistData {
    // StockDeal(Box<PhoenixOrdStockdeal>),
    // PendSettle(Box<PhoenixOrdPendSettle>),
    StockOrder(Box<PhoenixOrdStockorder>),
    SubOrder(Box<Vec<PhoenixOrdSuborder>>),
}

#[derive(Clone)]
pub struct OrderCenterController {
    pub settings: Arc<Settings>,
    pub tx_persist: mpsc::Sender<PersistData>,
    pub tx_order: broadcast::Sender<RouterMsg>, //发订单消息 -> 报盘
    pub tx_cancel: mpsc::Sender<ExecMsg>,   //未报撤单
    pub tx_opponent: mpsc::Sender<OrderDetail>,
    pub db_client: Arc<DbClient>,
    pub riskcenter_client: Arc<RiskCenterClient>,
    pub aka_client: Arc<AkaClient>,
    pub assetscenter_client: Arc<AssetsCenterClient>,
    pub account_risk_client: Arc<AccountRiskClient>,
    pub redis_client: Arc<RedisClient>,
    pub mq_client: Arc<NotificationClient>,
    pub uidgen: Arc<RwLock<UidgenService>>,
}
//增加订单redis缓存

impl OrderCenterController {
    pub async fn place_order(&self, order: &OrderReq) -> Result<OrderResp>{
        log::info!("收到订单: {:#?}", order);
        let mut id = self.uidgen.write().await;
        let mut order_detail = OrderDetail::place_order(&order).await;
        if order.order_id != 0 {
            order_detail.order_id = order.order_id;
        } else {
            order_detail.order_id = id.get_uid();
        }
        order_detail.msg_id = id.get_uid();
        drop(id);

        let mut resp = OrderResp {
            msg_id: order_detail.msg_id,
            order_id: order_detail.order_id,
            error_code: 0,
            error_msg: format!("委托成功"),
        };

        //取信息
        if let Err(err) = queryinfo::get_stock_info(&mut order_detail, &self.aka_client, &self.account_risk_client).await {
            log::error!("memo: {:?}", err);
            resp.error_code = 1;
            resp.error_msg = format!("query stock info failed: {:?}", err);
            return Ok(resp)
        }
        //查询交易日
        let ret = queryinfo::query_trade_date(order_detail.exchange_id as i64,0,1,0, &self.aka_client).await;
        if ret.as_ref().is_err() {
            log::error!("memo: {:?}", ret.as_ref().err().unwrap().to_string());
            resp.error_code = 1;
            resp.error_msg = format!("query trade date failed");
            return Ok(resp)
        }
        order_detail.sys_date = ret.unwrap().target_date;

        // 计算费用
        if let Err(err) = self.calc_fee_info(&mut order_detail).await {
            log::error!("{:?}", err);
            resp.error_code = 1;
            resp.error_msg = format!("{}", err);
            return Ok(resp)
        }

        order_detail.pre_fee = order_detail.fee_total;
        order_detail.capital = (Decimal::from(order_detail.order_amount) * order_detail.order_price * order_detail.margin_rate + order_detail.fee_total) * order_detail.rate;
        log::info!("订单明细: {:?}", &order_detail);

        //风控拆单
        let ret = self.risk_check(&mut order_detail).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
            resp.error_code = 1;
            resp.error_msg = format!("{:?}", ret.as_ref().err().unwrap().to_string());
            return Ok(resp);
        }
        let risk_check = ret.unwrap();

        //生成订单
        let ret = self.generate_order(&order_detail, &risk_check).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
            resp.error_code = 1;
            resp.error_msg = format!("{:?}", ret.as_ref().err().unwrap().to_string());
            return Ok(resp);
        }
        let (generate_order, sub_orders) = ret.unwrap();
        
        //自撮合通道
        if let Some(opponent_order)= sub_orders.iter().find(|x| x.channel_type == ChannelType::INTERNAL as i32 && x.channel_id != 0) {
            let data = OrderDetail::opponent_order(&mut order_detail, opponent_order).await;
            let _ = self.tx_opponent.send(data).await;
        }
        
        //请求报盘
        if self.settings.grpcserver.orderrouter.is_some() && order.order_id == 0{
            if let Err(err) = self.place_order_to_bp(&order_detail, &sub_orders).await {
                log::error!("{:?}", err);
                resp.error_code = 1;
                resp.error_msg = format!("{:?}", err);
                return Ok(resp);
            }
        }

        // 处理资产
        if let Err(err) = assetchange::place_order_asset_change(&order_detail, &self.assetscenter_client).await {
            log::error!("{:?}", err);
            LogClient::get().push_error(format!("{:?}", err).as_str()).await;
        }
        
        let _ = notificationmsg::notificationmsg(&order_detail, &generate_order, &self.mq_client).await;
        Ok(resp)
    }

    pub async fn cancel_order(&self, cancel_order: &CancelReq) -> Result<OrderResp>{
        log::info!("收到撤单请求: {:#?}", cancel_order);
        let mut id = self.uidgen.write().await;
        let mut cancel_order_detail = OrderDetail::cancel_order(&cancel_order).await;
        cancel_order_detail.msg_id         = id.get_uid();         // 消息ID
        cancel_order_detail.cancel_id      = id.get_uid();
        drop(id);

        let mut resp = OrderResp {
            msg_id: cancel_order_detail.msg_id,
            order_id: cancel_order_detail.order_id,
            error_code: 0,
            error_msg: "撤单委托成功".to_owned(),
        };

        let ret = self.check_cancel_order(&mut cancel_order_detail).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
            resp.error_code = 1;
            resp.error_msg = format!("{:?}", ret.as_ref().err().unwrap().to_string());
            return Ok(resp);
        }
        let (_order, mut sub_orders) = ret.unwrap();

        //设置子订单为待撤
        let ret = self.cancel_flag_update(cancel_order_detail.order_id, &mut sub_orders).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
            resp.error_code = 1;
            resp.error_msg = format!("{:?}", ret.as_ref().err().unwrap().to_string());
            return Ok(resp);
        }
        log::info!("撤单明细: {:#?}", &cancel_order_detail);
        // order.id = 0;
        // order.order_memo = format!("{}", order.order_no);//数据库怎么提现出该撤单委托对应那个订单?
        // order.order_no = cancel_order_detail.cancel_id;
        // // order.order_status = 
        // log::info!("原委托:{}, 对应撤单委托:{}", cancel_order_detail.order_id, order.order_no);
        // let _ = self.tx_persist.send(PersistData::StockOrder(Box::new(order))).await;

        if let Err(err) = self.cancel_order_to_bp(&mut cancel_order_detail, &sub_orders).await {
            log::error!("{:?}", err);
            resp.error_code = 1;
            resp.error_msg = format!("{:?}", err);
            return Ok(resp);
        }
        log::info!("cancel order end...");
        Ok(resp)
    }

    pub async fn replenishment_order(&self, req: &ReplenishOrderReq) -> Result<()> {
        if req.order.is_none() {
            return Err(anyhow!(format!("补单信息不全")));
        }
        let order = req.to_owned().order.unwrap();

        log::info!("收到补订单: {:#?}", order);
        let mut id = self.uidgen.write().await;
        let mut order_detail = OrderDetail::place_order(&order).await;
        if order.order_id != 0 {
            order_detail.order_id = order.order_id;
        } else {
            order_detail.order_id = id.get_uid();
        }
        order_detail.msg_id = id.get_uid();
        drop(id);

        //取信息
        if let Err(err) = queryinfo::get_stock_info(&mut order_detail, &self.aka_client, &self.account_risk_client).await {
            log::error!("memo: {:?}", err);
            return Err(anyhow!(format!("query stock info failed: {:?}", err)));
        }
        //查询交易日
        let ret = queryinfo::query_trade_date(order_detail.exchange_id as i64,0,1,0, &self.aka_client).await;
        if ret.as_ref().is_err() {
            log::error!("memo: {:?}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!(format!("query trade date failed")));
        }
        order_detail.sys_date = ret.unwrap().target_date;

        // 计算费用
        if let Err(err) = self.calc_fee_info(&mut order_detail).await {
            log::error!("{:?}", err);
            return Err(anyhow!(format!("{}", err)));
        }

        order_detail.pre_fee = order_detail.fee_total;
        order_detail.capital = (Decimal::from(order_detail.order_amount) * order_detail.order_price * order_detail.margin_rate + order_detail.fee_total) * order_detail.rate;
        log::info!("订单明细: {:?}", &order_detail);

        //风控拆单
        let mut risk_check: Vec<PhoenixRiskCheckInfo> = Vec::new();
        if req.riskinfo.is_empty() {
            let ret = self.risk_check(&mut order_detail).await;
            if ret.as_ref().is_err() {
                log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
                return Err(anyhow!(format!("{}", ret.as_ref().err().unwrap().to_string())));
            }
            risk_check = ret.unwrap();
        }
        let mut risk = PhoenixRiskCheckInfo::default();
        for x in req.riskinfo.iter() {
            risk.order_amount = x.order_amount;
            risk.channel_type = x.channel_type;
            risk.order_channel = x.channel_id;
            risk_check.push(risk.clone())
        };

        //生成订单
        let ret = self.generate_order(&order_detail, &risk_check).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!(format!("{}", ret.as_ref().err().unwrap().to_string())));
        }
        let (generate_order, sub_orders) = ret.unwrap();
        
        //自撮合通道
        if let Some(opponent_order)= sub_orders.iter().find(|x| x.channel_type == ChannelType::INTERNAL as i32 && x.channel_id != 0) {
            let data = OrderDetail::opponent_order(&mut order_detail, opponent_order).await;
            let _ = self.tx_opponent.send(data).await;
        }
        
        // 处理资产
        if let Err(err) = assetchange::place_order_asset_change(&order_detail, &self.assetscenter_client).await {
            log::error!("{:?}", err);
            LogClient::get().push_error(format!("{:?}", err).as_str()).await;
        }
        
        let _ = notificationmsg::notificationmsg(&order_detail, &generate_order, &self.mq_client).await;

        Ok(())
    }

    pub async fn generate_opponent_order(&self, detail: &mut OrderDetail) ->Result<()>{
        // detail.unit_id = 200844;
        detail.unit_id = self.settings.system.oppaccount;
        let mut id = self.uidgen.write().await;
        detail.order_id = id.get_uid();
        drop(id);
        log::info!("反向单: {:#?}", detail);

        // 计算费用
        if let Err(err) = self.calc_fee_info(detail).await {
            log::error!("{:?}", err);
            //消息中心
        }
        // 订单生成落库
        let generate_order = convertdata::convert_to_stockorder(&detail).await;
        // let _ = self.tx_persist.send(PersistData::StockOrder(Box::new(generate_order.clone()))).await;
        if let Err(err) = PhoenixOrdStockorder::insert(&generate_order, &self.db_client).await {
            log::error!("{}", err);
            return Ok(())
        } 

        let sub_order = convertdata::convert_to_suborder(&detail).await;
        // let _ = self.tx_persist.send(PersistData::SubOrder(Box::new(vec![sub_order]))).await;
        if let Err(err) = PhoenixOrdSuborder::insert_many(&vec![sub_order], &self.db_client).await {
            log::error!("{}", err);
            return Ok(())
        }

        // 处理资产
        if let Err(err) = assetchange::place_order_asset_change(&detail, &self.assetscenter_client).await {
            log::error!("{:?}", err);
            LogClient::get().push_error(format!("{:?}", err).as_str()).await;
        }
        detail.exec_type = ExecType::Confirm;
        let _ = notificationmsg::notificationmsg(detail, &generate_order, &self.mq_client).await;
        Ok(())
    }

    async fn risk_check(&self, order_detail: &mut OrderDetail) -> Result<Vec<PhoenixRiskCheckInfo>>{
        //风控拆单
        let ret = self.riskcenter_client.phoenix_risk_check(&order_detail).await;
        if ret.as_ref().is_err() {
            log::error!("memo: {:?}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        } 
        let risk_check = ret.unwrap();
        //废单
        if risk_check.retinfo.is_empty() {
            log::error!("memo: {:?}", risk_check.ret_msg);
            order_detail.memo = risk_check.ret_msg.to_owned();
            order_detail.order_status = OrderStatus::INVALID as i32;
            order_detail.capital = dec!(0.0);
            let generate_order = convertdata::convert_to_stockorder(&order_detail).await;
            let _ = self.tx_persist.send(PersistData::StockOrder(Box::new(generate_order))).await;
            return Err(anyhow!(risk_check.ret_msg));
        }
        log::info!("拆单结果: {:#?}", &risk_check);
        Ok(risk_check.retinfo)
    }

    async fn generate_order(&self, order_detail: &OrderDetail, risk_check: &Vec<PhoenixRiskCheckInfo>) -> Result<(PhoenixOrdStockorder, Vec<PhoenixOrdSuborder>)>{
        // 订单生成
        let mut order = convertdata::convert_to_stockorder(&order_detail).await;
        // 生成子订单
        let mut id = self.uidgen.write().await;
        let mut sub_order = convertdata::convert_to_suborder(&order_detail).await;
        let sub_orders: Vec<PhoenixOrdSuborder> = risk_check.iter().map(|v| {
            sub_order.sub_id = id.get_uid();
            log::info!("=============={}", sub_order.sub_id);
            sub_order.channel_id = v.order_channel as i32;
            sub_order.channel_type = v.channel_type;
            sub_order.order_amount = v.order_amount;
            if sub_order.channel_id == 0 {
                sub_order.order_status = OrderStatus::INVALID as i32;
                sub_order.remark = format!("channel未知");
                order.order_status = OrderStatus::INVALID as i32;
                order.order_memo = format!("channel未知");
            }
            if sub_order.channel_type == ChannelType::INTERNAL as i32 { 
                sub_order.relate_order = id.get_uid();
            }
            sub_order.clone()
        }).collect();
        drop(id);

        if let Err(err) = PhoenixOrdStockorder::insert(&order, &self.db_client).await {
            log::error!("{}", err);
            return Err(anyhow!(format!("订单处理失败,请稍后再试!")));
        } 
        if let Err(err) = PhoenixOrdSuborder::insert_many(&sub_orders, &self.db_client).await {
            log::error!("{}", err);
            return Err(anyhow!(format!("订单处理失败,请稍后再试!")));
        }
        Ok((order, sub_orders))
    }

    pub async fn place_order_to_bp(&self, order_detail: &OrderDetail, sub_orders: &Vec<PhoenixOrdSuborder>) -> Result<()>{
        log::info!("sub_orders: {:?}", sub_orders);
        if sub_orders.iter().find(|x| x.channel_id == 0).is_some() {
            return Err(anyhow!(format!("channel未知")))
        }
        for sub_order in sub_orders.iter() {
            if sub_order.channel_id != 0 {
                let router_msg = convertdata::convert_to_orderrouterinfo(sub_order, &order_detail).await;
                if let Err(err) = self.tx_order.send(router_msg) {
                    log::error!("{:?}", err);
                }
            }
        }
        Ok(())
    }

    pub async fn order_receipt(&self, router_msg: &RouterMsg) -> Result<()>{
        match router_msg.msg_type() {
            MsgType::Register => {},
            MsgType::Order => {},
            MsgType::Exec => {
                if let Some(msg_content) = router_msg.msg_content.clone() {
                    if let Some(exec_msg) = msg_content.exec_msg {
                        //分布式锁 redis
                        let lockkey= format!("{}{}", super::service::ordercache::LOCK_ORDER, exec_msg.order_id);
                        let mut count=0;
                        //获取锁，超过16次取锁失败，不继续获取锁,锁1秒
                        while self.redis_client.lock(&lockkey, 3).await!=1 && count<16 {
                            std::thread::sleep(std::time::Duration::from_millis(1));
                            count+=1;
                        }
                        match exec_msg.exec_type() {
                            ExecType::ExecUndef => {},
                            ExecType::Confirm => {
                                log::info!("委托确认: {:?}", exec_msg);
                                if let Err(err) = self.confirm_order_receipt(&exec_msg).await {
                                    log::error!("{:?}", err);
                                } 
                            },
                            ExecType::Filled => {
                                log::info!("成交: {:?}", exec_msg);
                                if let Err(err) = self.filled_order_receipt(&exec_msg).await {
                                    log::error!("{:?}", err);
                                }
                            },
                            ExecType::Canceled => {
                                log::info!("撤单: {:?}", exec_msg);
                                if let Err(err) = self.cancel_order_receipt(&exec_msg).await {
                                    log::error!("{:?}", err);
                                }
                            },
                            ExecType::Rejected => {
                                log::info!("废单: {:?}", exec_msg);
                                if let Err(err) = self.rejected_order_receipt(&exec_msg).await{
                                    log::error!("{:?}", err);
                                }
                            },
                        }
                        //释放锁
                        let _=self.redis_client.delele(&lockkey).await;
                    }
                }
            },
            MsgType::Response => {},
        }
        Ok(())
    }

    pub async fn confirm_order_receipt(&self, exec_msg: &ExecMsg) -> Result<()> {
        log::info!("开始处理委托确认: {:?}", exec_msg);
        let mut id = self.uidgen.write().await;
        let mut detail = OrderDetail::receipt_msg(&exec_msg).await;
        detail.msg_id = id.get_uid();
        // detail.deal_id = id.get_uid();
        // detail.settle_id = id.get_uid();
        drop(id);

        let cache_key = CacheKey {order_id: detail.order_id, ..Default::default() };
        let ret = cache_key.query_order_info(&self.redis_client, &self.db_client).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }
        let (mut sub_orders, mut order) = ret.unwrap();
        if sub_orders.is_empty(){
            log::error!("{}确认失败,order_status: {}, sub_orders len: {}",order.order_no, order.order_status, sub_orders.len());
            return Ok(());
        }

        for sub_order in sub_orders.iter_mut() {
            if sub_order.order_no == detail.order_id && 
                sub_order.channel_id == detail.channel_id && 
                sub_order.channel_type == detail.channel_type && 
                sub_order.order_status == OrderStatus::INITED as i32 //先成交, 后确认
            {
                sub_order.order_status = OrderStatus::SUBMITTED as i32;//已报
                sub_order.confirm_no = detail.confirm_no.to_owned();
                sub_order.modify_time = utility::timeutil::current_timestamp();
            } else {
                log::error!("找不到子订单: order_id: {}, channel_id: {}, channel_type: {}", 
                detail.order_id, detail.channel_id, detail.channel_type);
                return Ok(());
            }
        }

        if order.order_no == detail.order_id && order.order_status == OrderStatus::INITED as i32 {
            order.order_status = OrderStatus::SUBMITTED as i32;
            order.modify_time = utility::timeutil::current_timestamp();
        }

        let _ = self.persist_and_update_cache(&sub_orders, &order).await;
        log::info!("订单 {} 更新后: {:?}", detail.order_id, order);
        log::info!("委托确认处理成功: {}", detail.order_id);
        let _ = notificationmsg::notificationmsg(&detail, &order, &self.mq_client).await;
        Ok(())
    }

    pub async fn filled_order_receipt(&self, exec_msg: &ExecMsg) -> Result<()>{
        log::info!("开始处理成交: {:?}", exec_msg);
        let mut id = self.uidgen.write().await;
        let mut detail = OrderDetail::receipt_msg(&exec_msg).await;
        detail.msg_id = id.get_uid();
        detail.deal_id = id.get_uid();
        detail.settle_id = id.get_uid();
        drop(id);

        log::info!("成交明细: {:?}", &detail);
        if let Err(err) = self.begin_order_deal(&mut detail).await {
            log::error!("{}", err);
            return Err(err); 
        }

        //反向单模拟报盘消息回执
        //send -> order_receipt ->
        // let mut exec_msg = ExecMsg::default();
        // exec_msg.exec_type = ExecType::Filled as i32;
        // exec_msg.order_id = detail.relate_id;
        // exec_msg.exec_qty = detail.deal_amount;
        // exec_msg.exec_price = detail.deal_price.to_f64().unwrap_or_default();
        // exec_msg.exec_id = detail.exec_id.to_owned();
        // exec_msg.exec_time = detail.deal_time.to_owned();
        // exec_msg.order_direction = detail.order_direction;
        // exec_msg.brk_order_id = detail.confirm_no.to_owned();
        // exec_msg.channel_id = detail.channel_id as i64;
        // exec_msg.channel_type = detail.channel_type ;
        // exec_msg.memo = detail.memo.to_owned();

        // 反向单处理
        if detail.relate_id > 0 && detail.channel_type == 2 {
            // let mut id = UidgenService::new(self.settings.application.machineid, self.settings.application.nodeid);
            let mut id = self.uidgen.write().await;
            detail.order_id = detail.relate_id;
            detail.relate_id = 0;
            detail.deal_id = id.get_uid();
            detail.settle_id = id.get_uid();
            detail.exec_id = id.get_uid().to_string();
            drop(id);
            log::info!("反向单成交:{}, deal_id: {}, settle_id: {}, exec_id: {}", detail.order_id, detail.deal_id, detail.settle_id, detail.exec_id);
            if let Err(err) = self.begin_order_deal(&mut detail).await {
                log::error!("{}", err);
                return Err(err); 
            }
        }
        Ok(())
    }

    pub async fn begin_order_deal(&self, detail: &mut OrderDetail) -> Result<()> {
        if let Err(err) = PhoenixOrdStockdeal::cheak_order_deal(&detail.exec_id, &self.db_client).await {
            log::error!("{}", err);
            return Err(err); 
        }

        let cache_key = CacheKey {order_id: detail.order_id, ..Default::default() };
        let ret = cache_key.query_order_info(&self.redis_client, &self.db_client).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }
        let (mut sub_orders, mut order) = ret.unwrap();

        detail.sys_date = order.sys_date;
        detail.order_id = order.order_no;
        detail.unit_id = order.unit_id;
        detail.order_direction = order.order_direction;
        detail.pre_fee = order.pre_fee;
        detail.order_type = order.order_type;
        detail.stock_id = order.stock_id;
        detail.stock_code = order.stock_code.to_owned();
        detail.exchange_id = order.exchange_id;
        detail.un_deal_amount = order.order_amount - order.deal_amount - order.cancel_amount;
        detail.deal_value = Decimal::from(detail.deal_amount) * detail.deal_price;
        // detail.face_value = dec!(1.0);

        let ret = sub_orders.iter()
        .find(|sub_order| 
            sub_order.order_no == detail.order_id && 
            sub_order.channel_id == detail.channel_id && 
            sub_order.channel_type == detail.channel_type
        ); //.map(|x| x);
        if ret.is_none() {
            return Err(anyhow!("找不到子委托: order_id: {}, channel_id: {}, channel_type: {}", 
            detail.order_id, detail.channel_id, detail.channel_type));
        }
        let sub_order = ret.unwrap();
        detail.relate_id = sub_order.relate_order;

        if order.order_direction == OrderDirection::BUY as i32 || 
            order.order_direction == OrderDirection::SELL as i32 {
            detail.channel_id = sub_order.channel_id;
            detail.channel_type = sub_order.channel_type;
            if sub_order.channel_type == 2 && 
                (sub_order.order_status == OrderStatus::INVALID as i32 || 
                sub_order.order_status == OrderStatus::PARTIALCANCELED as i32 || 
                sub_order.order_status == OrderStatus::CANCELED as i32) 
            {
                log::info!("子委托已撤单: order_id: {}, channel_id: {}, channel_type: {}, order_status: {}", 
                sub_order.order_no, sub_order.channel_id, sub_order.channel_type, sub_order.order_status);
                return Err(anyhow!("子委托已撤单: order_id: {}, channel_id: {}, channel_type: {}, order_status: {}", 
                sub_order.order_no,sub_order.channel_id, sub_order.channel_type, sub_order.order_status));
            }

            if let Err(err) = queryinfo::get_stock_info(detail, &self.aka_client, &self.account_risk_client).await {
                log::error!("查询证券信息出错: {:?}", err);
                return Err(anyhow!("查询证券信息出错: {:?}", err));
            }

            detail.order_amount = detail.deal_amount;
            detail.order_price = detail.deal_price;
            if let Err(err) = self.calc_fee_info(detail).await {
                log::error!("计算费用出错: {:?}", err);
                return Err(anyhow!("计算费用出错: {:?}", err));
            }
        }
        detail.order_amount = order.order_amount;

        if detail.relate_id > 0 {
            //  取关联委托号
            let ret = PhoenixOrdSuborder::query_sub_order_by_subid(sub_order.relate_order, &self.db_client).await;
            if ret.as_ref().is_err() {
                log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
                return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
            }
            detail.relate_id = ret.unwrap().order_no;
            log::info!("订单: {}, 关联订单: {}", detail.order_id, detail.relate_id);
        }

        // // 计算卖收益================> 由资产算
        // if detail.order_direction == OrderDirection::SELL as i32 {
        //     if let Err(err) = self.cal_refer_profit(detail).await {
        //         log::error!("{:?}", err);
        //         return Err(err); 
        //     }
        // }

        // 成交落库
        let stockdeal = convertdata::convert_to_stockdeal(&detail).await;
        // let _ = self.tx_persist.send(PersistData::StockDeal(Box::new(stockdeal))).await;
        if let Err(err) = PhoenixOrdStockdeal::insert(&stockdeal, &self.db_client).await {
            log::info!("{:?}", err);
            return Err(err);
        }

        detail.capital = (detail.deal_value * detail.margin_rate + detail.fee_total) * detail.rate;
        detail.un_deal_amount = detail.un_deal_amount - detail.deal_amount;
        log::info!("订单未成交部分: {}", detail.un_deal_amount);
        if detail.un_deal_amount <= 0 {// 完全成交
            // 最后一笔将剩余的委托时的冻结都逆掉 ===>资产
            log::info!("订单{}委托最后一笔进入", detail.order_id);
            detail.capital = order.pre_capital;
        }

        //交收数据处理
        if let Err(err) = self.pendsettle_process(detail).await {
            log::error!("{}", err);
            return Err(err); 
        }

        //资产调整
        if let Err(err) = assetchange::deal_order_asset_change(&detail, &self.assetscenter_client).await {
            log::error!("{}", err);
            LogClient::get().push_error(format!("{:?}", err).as_str()).await;
            // 上手已成交, 调资产调整接口错误, 如何处理?
            // return Err(anyhow!("{:?}", err)); 
        }

        if let Err(err) = self.deal_update(&detail, &mut order, &mut sub_orders).await {
            log::error!("{:?}", err);
            return Err(err);
        }

        let _ = notificationmsg::notificationmsg(&detail, &order, &self.mq_client).await;
        log::info!("成交处理成功: {}", order.order_no);
        Ok(())
    }
    
    pub async fn cancel_order_receipt(&self, exec_msg: &ExecMsg) -> Result<()>{
        log::info!("开始处理撤单: {:?}", exec_msg);
        if exec_msg.channel_id == 0 {
            log::error!("撤单回报有误: {:?}", exec_msg);
            return Err(anyhow!("撤单回报有误: {:?}", exec_msg))
        }

        let mut id = self.uidgen.write().await;
        let mut cancel_detail = OrderDetail::receipt_msg(&exec_msg).await;
        cancel_detail.msg_id = id.get_uid();
        // cancel_detail.deal_id = id.get_uid();
        // cancel_detail.settle_id = id.get_uid();
        drop(id);

        if let Err(err) = self.begin_cancel_order(&mut cancel_detail).await {
            log::error!("{}", err);
            return Err(err); 
        }

        if cancel_detail.relate_id > 0 {
            cancel_detail.order_id = cancel_detail.relate_id;
            cancel_detail.relate_id = 0;
            log::info!("撤反向单: {}", cancel_detail.order_id);
            if let Err(err) = self.begin_cancel_order(&mut cancel_detail).await {
                log::error!("{}", err);
                return Err(err); 
            }
        }
        Ok(())
    }

    pub async fn begin_cancel_order(&self, cancel_detail: &mut OrderDetail) -> Result<()>{
        let cache_key = CacheKey {order_id: cancel_detail.order_id, ..Default::default() };
        let ret = cache_key.query_order_info(&self.redis_client, &self.db_client).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }
        let (mut sub_orders, mut cancel_order) = ret.unwrap();

        if cancel_detail.cancel_amount == 0 {
            let ret = sub_orders.iter()
            .find(|sub_order| 
                sub_order.order_no == cancel_detail.order_id && 
                sub_order.channel_id == cancel_detail.channel_id && 
                sub_order.channel_type == cancel_detail.channel_type
            );
            // .map(|x| x);
            if ret.is_some() {
                let sub_order = ret.unwrap();
                cancel_detail.cancel_amount = sub_order.order_amount - sub_order.deal_amount;
                cancel_detail.relate_id = sub_order.relate_order;
                cancel_detail.order_amount = sub_order.order_amount;
                log::info!("子订单: order_id: {}, channel_id: {}, channel_type: {}, order_amount: {}, deal_amount: {}, cancel_amount: {}", 
                    cancel_detail.order_id, cancel_detail.channel_id, cancel_detail.channel_type, 
                    sub_order.order_amount, sub_order.deal_amount, cancel_detail.cancel_amount
                );
            } else {
                log::error!("找不到子订单: order_id: {}, channel_id: {}, channel_type: {}", 
                cancel_detail.order_id, cancel_detail.channel_id, cancel_detail.channel_type);
            }  
        }

        if cancel_detail.channel_type == 1 {// 内盘待测数量
            let ret = sub_orders.iter()
            .find(|sub_order| 
                sub_order.order_no == cancel_detail.order_id && 
                sub_order.channel_id == cancel_detail.channel_id && 
                sub_order.channel_type == ChannelType::INTERNAL as i32
            );
            // .map(|x| x);
            if ret.is_some() {
                let sub_order = ret.unwrap();
                cancel_detail.cancel_amount += sub_order.order_amount - sub_order.deal_amount;
                log::info!("子订单: order_id: {}, channel_id: {}, channel_type: {}, order_amount: {}, deal_amount: {}, cancel_amount: {}", 
                    cancel_detail.order_id, cancel_detail.channel_id, cancel_detail.channel_type, 
                    sub_order.order_amount, sub_order.deal_amount, cancel_detail.cancel_amount
                );
            } else {
                log::error!("找不到子订单: order_id: {}, channel_id: {}, channel_type: {}", 
                cancel_detail.order_id, cancel_detail.channel_id, 2);
            }
        }

        if cancel_detail.cancel_amount <= 0 {
            log::info!("没有可撤数量: {}", cancel_detail.order_id);
            return Ok(());
        }

        //判断是否可以撤单
        let order_status = vec![OrderStatus::INVALID as i32, OrderStatus::DEALED as i32, OrderStatus::CANCELED as i32];
        if order_status.iter().find(|&&v| v == cancel_order.order_status).is_some() {//5：废单 7：已成 9：已撤
            log::info!("订单{}状态{}[5：废单 7：已成 9：已撤]不允许执行撤单", cancel_order.order_no, cancel_order.order_status);
            return Err(anyhow!("订单{}状态{}[5：废单 7：已成 9：已撤]不允许执行撤单", cancel_order.order_no, cancel_order.order_status));
        } 
        log::info!("待撤订单: {:#?}", &cancel_order);
        cancel_detail.unit_id = cancel_order.unit_id;
        cancel_detail.stock_id = cancel_order.stock_id;
        cancel_detail.order_direction = cancel_order.order_direction;
        // cancel_detail.stock_code = cancel_order.stock_code.to_owned();
        // cancel_detail.exchange_id = cancel_order.exchange_id;
        cancel_detail.order_amount = cancel_order.order_amount;
        cancel_detail.un_deal_amount = cancel_order.order_amount - cancel_order.deal_amount;
        if cancel_detail.order_amount == cancel_order.cancel_amount {
            log::error!("订单已撤: {}", cancel_detail.order_id);
            return Err(anyhow!("订单已撤: {}", cancel_detail.order_id));
        }
        log::info!("委托状态: {}", cancel_order.order_status);

        if let Err(err) = queryinfo::get_stock_info(cancel_detail, &self.aka_client, &self.account_risk_client).await {
            log::error!("查询证券信息出错: {:?}", err);
            return Err(anyhow!("查询证券信息出错: {:?}", err));
        }

        cancel_detail.order_amount = cancel_detail.cancel_amount;
        cancel_detail.order_price = cancel_order.order_price;
        if let Err(err) = self.calc_fee_info(cancel_detail).await {
            log::error!("计算费用出错: {:?}", err);
            return Err(anyhow!("计算费用出错: {:?}", err));
        }

        if cancel_detail.relate_id > 0 {
            //  取关联委托号
            let ret = PhoenixOrdSuborder::query_sub_order_by_subid(cancel_detail.relate_id, &self.db_client).await;
            if ret.as_ref().is_err() {
                log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
                return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
            }
            cancel_detail.relate_id = ret.unwrap().order_no;
            log::info!("订单: {}, 关联订单: {}", cancel_detail.order_id, cancel_detail.relate_id);
        }

        // 计算撤单金额
        // cancel_detail.capital = (cancel_order.order_price * Decimal::from(cancel_detail.cancel_amount) + cancel_detail.fee_total) * cancel_detail.rate;
        cancel_detail.capital = (cancel_order.order_price * Decimal::from(cancel_detail.cancel_amount) * cancel_detail.margin_rate + cancel_detail.fee_total) * cancel_detail.rate;
        log::info!("撤单金额: {}", cancel_detail.capital);
        log::info!("委托数量: {}, 委托金额: {}", cancel_detail.cancel_amount, cancel_detail.capital);

        // 判断是否为最后一笔
        if cancel_detail.un_deal_amount <= cancel_detail.cancel_amount {
            cancel_detail.cancel_amount = cancel_detail.un_deal_amount;
            cancel_detail.capital = cancel_order.pre_capital;
            log::info!("撤单数量: {}, 撤单金额: {}", cancel_detail.cancel_amount, cancel_detail.capital);
        }

        log::info!("cancel_detail: {:?}", &cancel_detail);
        // 进行证券或资金冻结(通知资产调整)
        if let Err(err) = assetchange::cancel_order_asset_change(cancel_detail, &self.assetscenter_client).await {
            log::error!("{:?}", err);
            LogClient::get().push_error(format!("{:?}", err).as_str()).await;
            // return Err(anyhow!(err));
        }

        if let Err(err) = self.cancel_update(cancel_detail, &mut cancel_order, &mut sub_orders).await {
            log::error!("{:?}", err);
            return Err(err);
        }
        let _ = notificationmsg::notificationmsg(&cancel_detail, &cancel_order, &self.mq_client).await;
        log::info!("撤单处理成功: {}", cancel_order.order_no);
        Ok(())
    }

    pub async fn rejected_order_receipt(&self, exec_msg: &ExecMsg) -> Result<()>{
        log::info!("开始处理废单: {:?}", exec_msg);
        if let Err(err) = self.cancel_order_receipt(exec_msg).await {
            return Err(err);
        }
        Ok(())
    }

    pub async fn pendsettle_process(&self, detail: &mut OrderDetail) -> Result<()>{
        //生成交收数据
        if let Err(err) = self.init_pend_settle_info(detail).await {
            // 处理失败, 发送到消息中心
            log::error!("{}", err);
            return Err(err);  
        }

        if let Err(err) = self.pendsettle_update_or_insert(&detail).await {
            // 处理失败, 发送到消息中心
            log::error!("{}", err);
            return Err(err);  
        }
        Ok(())
    }

    pub async fn init_pend_settle_info(&self, detail: &mut OrderDetail) -> Result<()>{
        if detail.order_direction == OrderDirection::SELL as i32 {
            // 卖时需多扣费用[盈亏+费用]
            detail.settle_capital = detail.deal_value - detail.fee_total;
        } else {
            // 买时需多收费用[费用]
            // 买时只会扣费用
            detail.settle_capital = detail.deal_value + detail.fee_total;
        }

        // 准备结算需要的数据
        log::info!("根据成交回报数据{}生成对应的待交割处理数据", detail.exec_id);
        let ret = PhoenixOmsTradeconfig::query_trade_config(detail, &self.db_client).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }
        let trade_config = ret.unwrap();
        if let Err(err) = PhoenixOmsTradeconfig::isvalid_trade_config(&trade_config).await{
            log::error!("{:?}", err);
            return Err(err); 
        }
        detail.clear_speed = trade_config.clear_speed.parse().unwrap_or_default();

        let _ = queryinfo::get_settle_date(detail, &self.redis_client).await;
        //计算日期信息redis?
        // 资产处理日期标识计算资产处理日期
        if trade_config.asset_settle_date.eq(&0) {//交易日处理资产
            detail.asset_settle_date = detail.sys_date;
        }  else if trade_config.asset_settle_date.eq(&1) {//交割日处理资产
            detail.asset_settle_date = detail.settle_date;
        }
        log::info!("交易日: {}, 交割日: {}, 资产处理: {}", detail.sys_date, detail.settle_date, detail.asset_settle_date);
        Ok(())
    }

    pub async fn pendsettle_update_or_insert(&self, detail: &OrderDetail) -> Result<()> {
        log::info!("order_id:{}, channel_id:{}, unit_id:{}, stock_id:{}", detail.order_id, detail.channel_id, detail.unit_id, detail.stock_id);
        let ret = PhoenixOrdPendSettle::select(detail, &self.db_client).await;
        if ret.as_ref().is_err() {
            log::error!("查询失败...");
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }
        let ret_ok = ret.unwrap();
        if ret_ok.is_none() {
            log::info!("未找订单 {} 交收信息, 将生成新的交收数据", detail.order_id);
            let model = convertdata::convert_to_pendsettle(detail).await;
            log::info!("insert PhoenixOrdPendSettle data: {:?}", &model);
            // let _ = self.tx_persist.send(PersistData::PendSettle(Box::new(model))).await;
            if let Err(err) = PhoenixOrdPendSettle::insert(&model, &self.db_client).await {
                log::error!("{:?}", err);
            }
            return Ok(());
        }
        let mut model = ret_ok.unwrap();
        log::info!("结算数量: {}, 成交数量: {}", model.settle_amount, detail.deal_amount);
        model.settle_amount += detail.deal_amount;
        model.settle_internal_amount += detail.deal_amount;
        model.settle_balance += detail.settle_capital;
        model.net_balance += detail.deal_value;
        if !Decimal::from(model.settle_amount).is_zero() {
            model.deal_avg_price = model.net_balance / Decimal::from(model.settle_amount);  //注意结算数量为0
        }
        // model.fee_total.add_assign(detail.fee_total);
        model.fee_total += detail.fee_total;
        model.fee_jy += detail.fee_jy;
        model.fee_yh += detail.fee_yh;
        model.fee_gh += detail.fee_gh;
        model.fee_yj += detail.fee_yj;
        model.fee_js += detail.fee_js;
        model.fee_zg += detail.fee_zg;
        // model.fee_other += detail.;
        if model.fee_js2.is_none() {
            model.fee_js2 = Some(detail.fee_js2);
        } else {
            let fee_js2 = model.fee_js2.unwrap();
            model.fee_js2 = Some(fee_js2 + detail.fee_js2);
        }
        if model.fee_jg.is_none() {
            model.fee_jg = Some(detail.fee_jg);
        } else {
            let fee_js2 = model.fee_jg.unwrap();
            model.fee_jg = Some(fee_js2 + detail.fee_jg);
        }
        log::info!("update PhoenixOrdPendSettle data: {:?}", &model);
        // let _ = self.tx_persist.send(PersistData::PendSettle(Box::new(model))).await;
        if let Err(err) = PhoenixOrdPendSettle::update(&model, &self.db_client).await {
            log::error!("{:?}", err);
        }

        Ok(())
    }

    pub async fn calc_fee_info(&self, detail: &mut OrderDetail) -> Result<()>{
        let ret = self.aka_client
        .query_fee_setting(detail.fee_type.clone(), detail.exchange_id as i64, detail.order_direction, detail.unit_id, detail.channel_id as i64, detail.stock_type).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }

        let fee_setting = ret.unwrap();
        log::info!("fee_setting: {:?}", fee_setting);
        let mut fee_detail = FeeDetail::new().await;
        fee_detail.amount = detail.order_amount;
        fee_detail.price = detail.order_price;
        fee_detail.order_direction = detail.order_direction;
        fee_detail.currency_no = detail.currency_no.to_owned();
        fee_detail.rate = detail.rate;

        if let Err(err) = FeeDetail::calc_fee_info(&mut fee_detail, &fee_setting, &self.redis_client).await {
            return Err(err);
        }
        detail.fee_jy = fee_detail.fee_jy; //交易费
        detail.fee_yh = fee_detail.fee_yh; //印花税
        detail.fee_gh = fee_detail.fee_gh; //过户费
        detail.fee_yj = fee_detail.fee_yj; //佣金
        detail.fee_js = fee_detail.fee_js; //经手费
        detail.fee_zg = fee_detail.fee_zg; //证管费
        detail.fee_qt = fee_detail.fee_qt; //其他费用
        detail.fee_js2 = fee_detail.fee_js2; //结算费
        detail.fee_jg = fee_detail.fee_jg; //交割费
        // detail.fee_fx = fee_detail.fee_fx; //风险金
        detail.fee_total = fee_detail.fee_total;
        Ok(())
    }

    pub async fn check_cancel_order(&self, detail: &mut OrderDetail) -> Result<(PhoenixOrdStockorder, Vec<PhoenixOrdSuborder>)> {
        let cache_key = CacheKey {order_id: detail.order_id, ..Default::default() };
        let ret = cache_key.query_order_info(&self.redis_client, &self.db_client).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }
        let (sub_orders, order) = ret.unwrap();

        //判断主委托是否可以撤单[委托记录处于(1,2,3,4,6,a 未报、正报、 待报、正报、已报、部成、待撤)允许撤单]
        let order_status = vec![OrderStatus::INITED as i32, 2, 3, OrderStatus::SUBMITTED as i32, OrderStatus::PARTIALDEALED as i32];
        if order_status.iter().find(|&&v| v == order.order_status).is_none() {
            log::info!("订单{}状态{}不允许执行撤单", order.order_no, order.order_status);
            return Err(anyhow!("订单{}状态{}不允许执行撤单", order.order_no, order.order_status));
        }
        // for sub_order in sub_orders.iter() {
        //     if sub_order.cancel_flag == 1 {
        //         return Err(anyhow!("撤单中!"));
        //     }
        // }
        detail.stock_code = order.stock_code.to_owned();    // 证券代码
        detail.exchange_id = order.exchange_id.to_owned();  // 市场id
        detail.order_status = order.order_status;
        detail.order_direction = order.order_direction;
        return Ok((order, sub_orders));
    }

    pub async fn cancel_flag_update(&self, order_id: i64, sub_orders: &mut Vec<PhoenixOrdSuborder>) -> Result<()> { 
        let cache_key = CacheKey {order_id, ..Default::default() };

        let order_status = vec![OrderStatus::INITED as i32, OrderStatus::SUBMITTED as i32, OrderStatus::PARTIALDEALED as i32];
        for sub_order in sub_orders.iter_mut() {
            if order_status.iter().find(|&&x| x == sub_order.order_status).is_some() {
                sub_order.cancel_flag = 1;
            } 
        }

        let _ = self.tx_persist.send(PersistData::SubOrder(Box::new(sub_orders.to_owned()))).await;
        if let Err(err) = cache_key.update_sub_order_cache(&sub_orders, &self.redis_client).await {
            log::error!("{}", err);
            return Err(anyhow!(err));
        }
        Ok(()) 
    }

    pub async fn cancel_order_to_bp(&self, detail: &mut OrderDetail, sub_orders: &Vec<PhoenixOrdSuborder>) -> Result<()> {
        let order_status = vec![OrderStatus::INVALID as i32, OrderStatus::DEALED as i32, OrderStatus::PARTIALCANCELED as i32, OrderStatus::CANCELED as i32];
        for sub_order in sub_orders.iter() {
            log::info!("{:?}", sub_order);
            if sub_order.cancel_flag == 1 {
                //检查子订单状态(5：废单, 7：已成 8：部撤 9：已撤)  5789 表示最终状态, 无法撤单
                if order_status.iter().find(|&&v| v == sub_order.order_status).is_some() {
                    log::info!("订单{}状态{}不允许执行撤单", sub_order.sub_id, sub_order.order_status);
                    continue;
                } 
                if sub_order.order_status == OrderStatus::INITED as i32 {
                    log::info!("子订单{}未报 order_status, {}", sub_order.sub_id, sub_order.order_status);
                    let mut exec_msg = ExecMsg::default();
                    exec_msg.order_id = detail.order_id;
                    exec_msg.channel_id = sub_order.channel_id as i64;
                    exec_msg.channel_type = sub_order.channel_type;
                    exec_msg.exec_qty =  sub_order.order_amount - sub_order.deal_amount; //取消数量
                    exec_msg.memo = format!("INITED cancel order");
                    let _ = self.tx_cancel.send(exec_msg).await;
                    continue;
                }
                detail.cancel_amount = sub_order.order_amount - sub_order.deal_amount; //取消数量
                let router_msg = convertdata::convert_to_orderrouterinfo(sub_order, &detail).await;
                if let Err(err) = self.tx_order.send(router_msg) {
                    log::error!("{:#?}", err);
                    return Err(anyhow!(err));
                }
            }
        }
        Ok(())
    }

    pub async fn deal_update(&self, detail: &OrderDetail, order: &mut PhoenixOrdStockorder, sub_orders: &mut Vec<PhoenixOrdSuborder>) -> Result<()>{
        let ret = sub_orders.iter_mut()
        .find(|sub_order| 
            sub_order.order_no == detail.order_id && 
            sub_order.channel_id == detail.channel_id && 
            sub_order.channel_type == detail.channel_type
        );
        // .map(|x| x);
        if ret.is_none() {
            return Err(anyhow!("找不到子订单: order_id: {}, channel_id: {}, channel_type: {}", 
            detail.order_id, detail.channel_id, detail.channel_type));
        }
        let sub_order = ret.unwrap();

        //主
        if order.deal_amount + detail.deal_amount == order.order_amount {
            log::info!("订单已成: {}", order.order_no);
            order.order_status = OrderStatus::DEALED as i32;//已成
        } else {
            log::info!("订单部成: {}", order.order_no);
            order.order_status = OrderStatus::PARTIALDEALED as i32;//部成
        }
        order.deal_amount += detail.deal_amount;
        order.deal_value += detail.deal_value;
        order.deal_fee += detail.deal_fee;
        // order.pre_capital.sub_assign(detail.capital);
        order.pre_capital -= detail.capital;
        order.last_deal_time = detail.last_deal_time;
        order.deal_fee += detail.fee_total;
        order.modify_time = utility::timeutil::current_timestamp();

        //子
        if sub_order.deal_amount + detail.deal_amount == sub_order.order_amount {
            log::info!("子订单已成 order_no: {} channel_id: {}, channel_type: {}", 
                sub_order.order_no, detail.channel_id, detail.channel_type
            );
            sub_order.order_status =  OrderStatus::DEALED as i32;//已成
        } else {
            log::info!("子订单部成 order_no: {} channel_id: {}, channel_type: {}", 
                sub_order.order_no, detail.channel_id, detail.channel_type
            );
            sub_order.order_status = OrderStatus::PARTIALDEALED as i32;//部成
        }
        sub_order.deal_amount += detail.deal_amount;
        sub_order.deal_value += detail.deal_value;
        if sub_order.confirm_no.is_empty() {
            sub_order.confirm_no = detail.confirm_no.to_owned();
        }
        sub_order.modify_time = utility::timeutil::current_timestamp();

        let _ = self.persist_and_update_cache(&sub_orders, &order).await;
        log::info!("订单 {} 成交, 更新后: {:?}", detail.order_id, order);
        Ok(())
    }

    pub async fn cancel_update(&self, detail: &OrderDetail, order: &mut PhoenixOrdStockorder, sub_orders: &mut Vec<PhoenixOrdSuborder>) -> Result<()>{
        let ret = sub_orders.iter_mut()
        .find(|sub_order| 
            sub_order.order_no == detail.order_id && 
            sub_order.channel_id == detail.channel_id && 
            sub_order.channel_type == detail.channel_type
        );
        // .map(|x| x);
        if ret.is_none() {
            return Err(anyhow!("没有子订单: order_id: {}, channel_id: {}, channel_type: {}", 
                detail.order_id, detail.channel_id, detail.channel_type
            ));
        }
        let sub_order = ret.unwrap();
        //主
        if order.cancel_amount + detail.cancel_amount == order.order_amount {
            order.order_status = OrderStatus::CANCELED as i32;//已撤
        } else {
            order.order_status = OrderStatus::PARTIALCANCELED as i32;//部撤
        }
        order.cancel_amount += detail.cancel_amount;
        order.order_memo = detail.memo.to_owned();
        order.pre_capital -= detail.capital;
        order.modify_time = utility::timeutil::current_timestamp();

        //子
        if sub_order.deal_amount == 0 {
            sub_order.order_status = OrderStatus::CANCELED as i32;
        } else {
            sub_order.order_status = OrderStatus::PARTIALCANCELED as i32;//部撤
        }
        // if sub_order.deal_amount + detail.cancel_amount == sub_order.order_amount {
        //     sub_order.cancel_flag = 0;
        // }
        if detail.exec_type == ExecType::Rejected {
            if order.order_amount == sub_order.order_amount {
                order.order_status = OrderStatus::INVALID as i32;//废单
            }
            sub_order.order_status = OrderStatus::INVALID as i32;//废单
        }
        if sub_order.confirm_no.is_empty() {
            sub_order.confirm_no = detail.confirm_no.to_owned();
        }
        sub_order.modify_time = utility::timeutil::current_timestamp();
        sub_order.remark = detail.memo.to_owned();

        let _ = self.persist_and_update_cache(&sub_orders, &order).await;
        log::info!("订单 {} 撤单/废单, 更新后: {:?}", detail.order_id, order);

        // 撤单委托如何更新?
        // PhoenixOrdStockorder::cancel_entrust_update()
        Ok(())       
    }

    pub async fn persist_data(&self, persist_data: &PersistData) -> Result<()> {
        let now = std::time::Instant::now();
        match persist_data {
            PersistData::StockOrder(data) => {
                log::info!("persist_data 订单: {:?}", &data);
                if let Err(err) = PhoenixOrdStockorder::save(&data, &self.db_client).await {
                    log::error!("{:?}", err);
                }
            },
            PersistData::SubOrder(data) => {
                log::info!("persist_data 子订单: {:?}", &data);
                if let Err(err) = PhoenixOrdSuborder::save_many(&data, &self.db_client).await {
                    log::error!("{:?}", err);
                }
            },
            // PersistData::StockDeal(data) => {
            //     log::info!("persist_data 成交: {:?}", &data);
            //     if let Err(err) = PhoenixOrdStockdeal::save(&data, &self.db_client).await {
            //         log::error!("{:?}", err);
            //     }
            // },
            // PersistData::PendSettle(data) => {
            //     log::info!("persist_data 交收: {:?}", &data);
            //     if let Err(err) = PhoenixOrdPendSettle::save(&data, &self.db_client).await {
            //         log::error!("{:?}", err);
            //     }
            // },
        }
        log::info!("persist completed, elapsed: {:?}", now.elapsed());
        Ok(())
    }

    pub async fn persist_and_update_cache(&self, sub_orders: &Vec<PhoenixOrdSuborder>, order: &PhoenixOrdStockorder) {
        let cache_key = CacheKey {order_id: order.order_no, ..Default::default() };
        let _ = self.tx_persist.send(PersistData::SubOrder(Box::new(sub_orders.to_owned()))).await;
        if let Err(err) = cache_key.update_sub_order_cache(&sub_orders, &self.redis_client).await {
            log::error!("{}", err);
        }
        let _ = self.tx_persist.send(PersistData::StockOrder(Box::new(order.to_owned()))).await;
        if let Err(err) = cache_key.update_order_cache(&order, &self.redis_client).await {
            log::error!("{}", err);
        }
    }

    pub async fn cal_refer_profit(&self, detail: &mut OrderDetail) -> Result<()>{
        // let ret = PhoenixAstStockposition::query_position(&detail, &self.db_client).await;
        // if ret.as_ref().is_err() {
        //     log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
        //     return Err(anyhow!(ret.as_ref().err().unwrap().to_string())); 
        // }
        // let position = ret.unwrap();

        let ret = assetchange::query_postion_info(detail.unit_id, detail.stock_id, &self.assetscenter_client).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string())); 
        }
        let position = ret.unwrap();
        let mut avg_price = dec!(0.0);
        
        if !position.current_amount.is_zero() {
            avg_price = Decimal::from_f64(position.total_value / position.current_amount as f64).unwrap_or_default();
        }
        // (成交价 - 均价) * 数量
        detail.profit = Decimal::from(detail.deal_amount) * (detail.deal_price - avg_price);
        log::info!("盈亏: {}", detail.profit); 
        Ok(())
    }
}