use std::process;
use std::sync::Arc;
use anyhow::Result;
use common::constant::OrderDirection;
use dashmap::DashMap;
use sea_orm::TransactionTrait;
// use chrono::{Datelike, Timelike};
use tokio::sync::{broadcast, mpsc};
use rust_decimal::{prelude::*, Decimal};
use common::{
    constant::OrderStatus,
    constant::AssetChangeType,
    constant::AssetChangeDirection,
    uidservice::UidgenService,
};

use crate::protofiles::{
    Currency,
    OrderReq, CancelReq, OrderResp,
    RouterMsg, OrderType, OrderMsg, MsgType, MsgContent, ExecType, ExecMsg, 
    PhoenixassetscenterRequest, PhoenixassetscenterRequestInfo, PhoenixassetspostionrequestInfo
};
use crate::client::{
    DbClient,
    AkaCenterClient,
    RiskCenterClient,
    AssetsCenterClient,
};

use crate::dataservice::entities::{
    prelude::PhoenixOrdSuborder, 
    prelude::PhoenixOrdStockorder,
    prelude::PhoenixOrdStockdeal,
    prelude::PhoenixSysDictionary,
    prelude::PhoenixOmsFeeset,
    prelude::PhoenixOmsTradeconfig,
};
use crate::common::common::{OrderDetail, CommodityInfo};

pub enum PersistData {
    StockOrder(Box<PhoenixOrdStockorder>),
    SubOrder(Box<Vec<PhoenixOrdSuborder>>),
    StockDeal(Box<PhoenixOrdStockdeal>),
}

#[derive(Clone)]
pub struct OrderCenterController {
    pub tx_persist: mpsc::Sender<PersistData>,
    pub tx_order: broadcast::Sender<RouterMsg>, //发订单消息 -> 报盘
    pub tx_cancel: mpsc::Sender<ExecMsg>,   //未报撤单
    pub db_client: Arc<DbClient>,
    pub riskcenter_client: RiskCenterClient,
    // pub assetscenter_client: AssetsCenterClient,
    // pub akacenter_client: AkaCenterClient,
    pub commodity_info: Arc<DashMap<i64, CommodityInfo>>,
}

impl OrderCenterController {
    // // 初始化证券基础信息
    // pub async fn init(&self) -> Result<()>{
    //     let  ret = self.akacenter_client.query_stock_info(0).await.unwrap_or_else(|err| {
    //         log::error!("{:?}", err);
    //         process::exit(1);
    //     });
    //     let _ = ret.data.iter().map(|data| {
    //         let commodity_info = CommodityInfo {
    //             stock_id: data.stock_id,
    //             stock_type: data.stock_type,
    //             exchange_id: data.market_id as i32,
    //             stock_code: data.stock_code.to_owned(),
    //             exchange_code: data.market_code.to_owned(),
    //             currency_no: {
    //                 if data.trade_currency() == Currency::Cny {
    //                     "CNY".to_owned()
    //                 } else if data.trade_currency() == Currency::Usd {
    //                     "USD".to_owned()
    //                 } else {
    //                     "HKD".to_owned()
    //                 }
    //             },
    //         };
    //         self.commodity_info.insert(data.stock_id, commodity_info);
    //     });
    //     Ok(())
    // }

    pub async fn persist_data(&self, persist_data: &PersistData) -> Result<()> {
        match persist_data {
            PersistData::StockOrder(data) => {
                if data.id > 0 {
                    if let Err(err) = PhoenixOrdStockorder::update(&data, &self.db_client).await {
                        log::error!("{:?}", err);
                    }
                    log::info!("订单更新成功: {}", &data.order_no);
                } else {
                    if let Err(err) = PhoenixOrdStockorder::insert(&data, &self.db_client).await {
                        log::error!("{:?}", err);
                    }
                    log::info!("订单落库成功: {}", &data.order_no);
                }
            },
            PersistData::SubOrder(data) => {
                if let Err(err) = PhoenixOrdSuborder::insert_many(&data, &self.db_client).await {
                    log::error!("{:?}", err);
                }
                log::info!("{}个子订单落库成功", data.len());
            },
            PersistData::StockDeal(data) => {
                if data.id != 0 {
                    //更新
                } else {
                    if let Err(err) = PhoenixOrdStockdeal::insert(&data, &self.db_client).await {
                        //后期加消息中心
                        log::error!("{:?}", err);
                    }
                    log::info!("成交落库成功 order_no: {}, deal_no: {}", &data.order_no, data.deal_no);
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
                        match exec_msg.exec_type() {
                            ExecType::ExecUndef => {},
                            ExecType::Confirm => {
                                log::info!("委托确认: {:#?}", exec_msg);
                                if let Err(err) = self.confirm_order_receipt(&exec_msg).await {
                                    log::error!("{:?}", err);
                                } 
                            },
                            ExecType::Filled => {
                                log::info!("成交: {:#?}", exec_msg);
                                if let Err(err) = self.filled_order_receipt(&exec_msg).await {
                                    log::error!("{:?}", err);
                                }
                            },
                            ExecType::Canceled => {
                                log::info!("撤单: {:#?}", exec_msg);
                                if let Err(err) = self.cancel_order_receipt(&exec_msg).await {
                                    log::error!("{:?}", err);
                                }
                            },
                            ExecType::Rejected => {
                                log::info!("废单: {:#?}", exec_msg);
                                if let Err(err) = self.rejected_order_receipt(&exec_msg).await{
                                    log::error!("{:?}", err);
                                }
                            },
                        }
                    }
                }
            },
            MsgType::Response => {},
        }
        Ok(())
    }

    pub async fn deal_update(&self, detail: &mut OrderDetail, order: &PhoenixOrdStockorder, sub_order: &PhoenixOrdSuborder) -> Result<()>{
        let deal_amount = detail.deal_amount;//备份
        let deal_value = detail.deal_value;
        if order.deal_amount + deal_amount == order.order_amount {
            log::info!("订单已成: {}", order.order_no);
            detail.order_status = OrderStatus::DEALED as i32;//已成
        } else {
            log::info!("订单部成: {}", order.order_no);
            detail.order_status = OrderStatus::PARTIALDEALED as i32;//部成
        }
        detail.deal_amount += order.deal_amount;
        detail.deal_value += order.deal_value.to_f64().unwrap_or_default();
        if let Err(err) = PhoenixOrdStockorder::deal_update(detail, &self.db_client).await {
            log::error!("{:?}", err);
            return Err(anyhow!("{:?}", err)); 
        }
        if sub_order.deal_amount + deal_amount == sub_order.order_amount {
            log::info!("子订单已成 order_no: {} channel_id: {}, channel_type: {}", sub_order.order_no, detail.channel_id, detail.channel_type);
            detail.order_status =  OrderStatus::DEALED as i32;//已成
        } else {
            log::info!("子订单部成 order_no: {} channel_id: {}, channel_type: {}", sub_order.order_no, detail.channel_id, detail.channel_type);
            detail.order_status = OrderStatus::PARTIALDEALED as i32;//部成
        }
        detail.deal_amount = sub_order.deal_amount + deal_amount;
        detail.deal_value = sub_order.deal_value.to_f64().unwrap_or_default() + deal_value;
        if let Err(err) = PhoenixOrdSuborder::deal_update(detail, &self.db_client).await {
            log::error!("{:?}", err);
            return Err(anyhow!("{:?}", err)); 
        }
        Ok(())
    }

    pub async fn cancel_update(&self, detail: &mut OrderDetail, order: &PhoenixOrdStockorder) -> Result<()>{
        //主委托更新
        if order.cancel_amount + detail.cancel_amount == order.order_amount {
            detail.order_status = OrderStatus::CANCELED as i32;//已撤
        } else {
            detail.order_status = OrderStatus::PARTIALCANCELED as i32;//部撤
        }
        detail.cancel_amount += order.cancel_amount;
        if let Err(err) = PhoenixOrdStockorder::cancel_update(&detail, &self.db_client).await {
            log::error!("{:?}", err);
            return Err(anyhow!("{:?}", err));
        }
        // 子委托更新  
        if let Ok(sub_order) = PhoenixOrdSuborder::query_sub_order(&detail, &self.db_client).await {
            if sub_order.deal_amount == 0 {
                detail.order_status = OrderStatus::CANCELED as i32;
            } else {
                detail.order_status = OrderStatus::PARTIALCANCELED as i32;//部撤
            }
            if let Err(err) = PhoenixOrdSuborder::cancel_update(&detail, &self.db_client).await {
                log::error!("{:?}", err);
                return Err(anyhow!("{:?}", err));
            }
        } else {
            log::info!("没有子订单: order_id: {}, channel_id: {}, channel_type: {}", detail.order_id, detail.channel_id, detail.channel_type);
        }
        Ok(())       
    }

    pub async fn place_order(&self, order: &OrderReq) -> Result<OrderResp>{
        log::info!("收到订单: {:#?}", order);
        let mut resp = OrderResp::default();
        let mut order_detail = OrderDetail::place_order(&order).await;
        resp.msg_id = order_detail.msg_id;
        resp.order_id = order_detail.order_id; //订单id
        resp.error_code = 0;
        let ret = self.get_stock_info(&mut order_detail).await;
        if ret.as_ref().is_err() {
            log::error!("memo: {:?}", ret.as_ref().err().unwrap().to_string());
            resp.error_msg = format!("下单失败: {}", ret.as_ref().err().unwrap().to_string());
            return Ok(resp)
        }

        // if order_detail.price_type == 1 {//现价
        //     // 取最新价
        //     // CalcTotalFee
        // }

        // 计算费用
        if let Err(err) = self.calc_fare_info(&mut order_detail).await {
            log::error!("{:?}", err);
            //消息中心
            return Err(anyhow!("{:?}", err));
        }
        
        // 获取币种,汇率计算费用,计算预卖预买金额
        order_detail.pre_fee = order_detail.fee_total;
        // 冻结金额和买入金额=委托数量*委托价格 + 费用
        order_detail.pre_capital = order_detail.order_amount as f64 * order_detail.order_price + order_detail.fee_total;
        log::info!("订单明细: {:#?}", &order_detail);





        //请求风控
        let ret = self.riskcenter_client.phoenix_risk_check(&order_detail).await;//返回子订单, 可能多个
        if ret.as_ref().is_err() {
            log::error!("memo: {:?}", ret.as_ref().err().unwrap().to_string());
            resp.error_msg = format!("{}", ret.as_ref().err().unwrap().to_string());
            return Ok(resp)
        } 
        let risk_check = ret.unwrap();
        if risk_check.retinfo.is_empty() {
            //风控返回错误, 数据库,订单状态为废单, 结束本次操作
            log::error!("废单原因: {:?}", risk_check.ret_msg);
            // 订单生成落库 
            order_detail.order_status = OrderStatus::INVALID as i32;
            order_detail.memo = risk_check.ret_msg.to_owned();
            let generate_order = PhoenixOrdStockorder::convert_to_stockorder(&order_detail).await;
            let _ = self.tx_persist.send(PersistData::StockOrder(Box::new(generate_order))).await;

            resp.error_code = risk_check.ret_code.parse().unwrap_or_default();
            resp.error_msg = format!("{}", risk_check.ret_msg);
            return Ok(resp)
        }
        log::info!("拆单结果: {:#?}", &risk_check);

        // 订单生成落库
        let generate_order = PhoenixOrdStockorder::convert_to_stockorder(&order_detail).await;
        let _ = self.tx_persist.send(PersistData::StockOrder(Box::new(generate_order))).await;

        //生成子订单落库
        let mut sub_order = PhoenixOrdSuborder::convert_to_suborder(&order_detail).await;
        let sub_orders: Vec<PhoenixOrdSuborder> = risk_check.retinfo.iter().map(|v| {
            let mut id = UidgenService::new(1,1);
            sub_order.sub_id = id.get_uid() % 100000000;
            sub_order.channel_id = v.order_channel as i32;
            sub_order.channel_type = v.channel_type;
            sub_order.order_amount = v.order_amount;
            log::info!("子订单: {:?}", &sub_order);
            sub_order.clone()
        }).collect();
        let _ = self.tx_persist.send(PersistData::SubOrder(Box::new(sub_orders.clone()))).await;




        //请求报盘
        for sub_order in sub_orders.iter() {
            let router_msg = OrderCenterController::convert_to_orderrouterinfo(sub_order, &order_detail).await;
            if let Err(err) = self.tx_order.send(router_msg) {
                log::error!("{:#?}", err);
                //发送失败,需要处理为废单,同时调整回滚资产
            }
        }

        //处理资产
        // let assets_change = AssetsCenterClient::convert_to_assets(&order_detail).await;
        // let ret = self.assetscenter_client.phoenix_assets_change(&order_detail).await;
        // if ret.as_ref().is_err() {
        //     return Err(anyhow!("{:?}", ret.as_ref().err().unwrap().to_string()));
        // } 

        // 下单成功返回
        resp.error_msg = format!("委托成功");
        log::info!("委托结束");
        Ok(resp)
    }

    pub async fn cancel_order(&self, cancel_order: &CancelReq) -> Result<OrderResp>{
        let mut cancel_order_detail = OrderDetail::cancel_order(&cancel_order).await;
        log::info!("撤单明细: {:#?}", &cancel_order_detail);
        let mut resp = OrderResp {
            msg_id: cancel_order_detail.msg_id,
            order_id: cancel_order_detail.order_id,
            error_code: 0,
            error_msg: "撤单成功".to_owned(),
        };
        // 撤单: 未报撤单, 已报撤单 ?
        if let Err(err) = self.do_order_cancel(&mut cancel_order_detail).await {
            log::error!("{:#?}", err);
            resp.error_msg = format!("{}", err);
            return Ok(resp);
        }
        //目前主订单状态为a
        let sub_orders = PhoenixOrdSuborder::query_all_sub_order(cancel_order_detail.unit_id, cancel_order_detail.order_id, &self.db_client).await;
        // let mut total_amount: i32 = 0;//用于更新主委托
        for sub_order in sub_orders.iter() {
            log::info!("{:#?}", sub_order);
            cancel_order_detail.cancel_amount = sub_order.order_amount - sub_order.deal_amount; //取消数量
            //检查子订单状态(5：废单, 7：已成 8：部撤 9：已撤)  5789 表示最终状态, 无法撤单,跳过
            let order_status = vec![OrderStatus::INITED as i32, 2, 3, OrderStatus::SUBMITTED as i32, OrderStatus::PARTIALDEALED as i32];
            if order_status.iter().find(|&&v| v == sub_order.order_status).is_some() {
                log::info!("订单{}状态{}不允许执行撤单", sub_order.sub_id, sub_order.order_status);
                continue;
            } 
            // 订单状态1(未报) ===>直接撤   channel 处理还是直接处理?
            if sub_order.order_status == OrderStatus::INITED as i32 {
                log::info!("订单未报,直接撤 order_status, {}", sub_order.order_status);
                let mut exec_msg = ExecMsg::default();
                    exec_msg.order_id = cancel_order_detail.order_id;
                    exec_msg.channel_id = sub_order.channel_id as i64;
                    exec_msg.channel_type = sub_order.channel_type;
                    // exec_type: ExecType::Canceled as i32,
                    // exec_qty: 0,
                    // exec_price: todo!(),
                    // exec_id: ,
                    // exec_time: todo!(),
                    // order_direction: sub_order.,
                    // brk_order_id: todo!(),

                    // memo: todo!(),
                let _ = self.tx_cancel.send(exec_msg).await;
                continue;
            }
            // 其他 ===> 报盘
            cancel_order_detail.cancel_amount = sub_order.order_amount - sub_order.deal_amount; //取消数量
            let router_msg = OrderCenterController::convert_to_orderrouterinfo(sub_order, &cancel_order_detail).await;
            if let Err(err) = self.tx_order.send(router_msg) {
                log::error!("{:#?}", err);
                resp.error_msg = format!("{}", err);
                return Ok(resp);
            }
            // total_amount += cancel_order_detail.cancel_amount;
        }
        // 更新主委托取消数量,撤单标志 =>待撤
        // log::info!("订单共取消数量为: {}", total_amount);
        // 更新子订单cancel_flag为1
        log::info!("cancel order end...");
        Ok(resp)
    }

    pub async fn do_order_cancel(&self, order_detail: &mut OrderDetail) -> Result<()> {
        //主委托
        let ret = PhoenixOrdStockorder::query_order(order_detail.order_id, &self.db_client).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("{:?}", ret.as_ref().err().unwrap().to_string()));
        }
        let mut order = ret.unwrap();
        log::info!("待测订单: {:#?}", &order);
        order_detail.stock_code = order.stock_code.to_owned();    // 证券代码
        order_detail.exchange_id = order.exchange_id.to_owned();  // 市场id
        order_detail.order_status = order.order_status;
        //判断主委托是否可以撤单[委托记录处于(1,2,3,4,6,a 未报、正报、 待报、正报、已报、部成、待撤)允许撤单]
        let order_status = vec![1,2,3,4,6];
        if order_status.iter().find(|&&v| v == order.order_status).is_none() {
            return Err(anyhow!("订单{}状态{}不允许执行撤单", order.order_no, order.order_status));
        } 

        //设置子订单为待撤 cancel_flag = 1 ====> 子订单有成交的如何处理(解决: 增加条件: 订单状态为 1,4,6的设置)
        match PhoenixOrdSuborder::cancel_flag_update(order_detail.order_id, &self.db_client).await {
            Ok(row) => {
                if row == 0 {
                    log::info!("没有待撤订单: {}", order_detail.order_id);
                    return Ok(());
                }
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!("{:?}", err));
            }
        };
        //订单状态 主订单
        // 1,2,3,4,6 => 未报、正报、 待报、正报、已报、部成
        // 状态改为 待测order_status = a

        //生成撤单委托

        //a =>待测订单
        //生成撤单委托
        order.id = 0;
        order.order_no = order_detail.cancel_id;
        order.order_memo = Some(format!("{}", order.order_no));//数据库怎么提现出该撤单委托对应那个订单?
        log::info!("原委托:{}, 对应撤单委托:{}", order_detail.order_id, order.order_no);
        let _ = self.tx_persist.send(PersistData::StockOrder(Box::new(order))).await;
        Ok(())
    }

    pub async fn invalid_order_persist(&self, order_detail: &mut OrderDetail, memo: &String) {
        log::error!("废单原因: {:?}", memo);
        // 无效订单落库 
        order_detail.order_status = OrderStatus::INVALID as i32;
        order_detail.memo = format!("{}", memo);
        let generate_order = PhoenixOrdStockorder::convert_to_stockorder(&order_detail).await;
        let _ = self.tx_persist.send(PersistData::StockOrder(Box::new(generate_order))).await;
    }

    pub async fn query_stock_info(&self, stock_id: i64) -> Result<CommodityInfo> {
        if let Some(commodity_info) = self.commodity_info.get(&stock_id) {
            return Ok(commodity_info.value().to_owned())
        } else {
            //未找到,从接口查
            log::info!("基础数据查询");
            // let ret = self.akacenter_client.query_stock_info(stock_id).await;
            // if ret.as_ref().is_err() {
            //     log::error!("基础数据查询失败: {:?}", ret.as_ref().err().unwrap().to_string());
            //     return Err(anyhow!("基础数据查询失败: {:?}", ret.as_ref().err().unwrap().to_string()));
            // }
            // let val = ret.unwrap();
            // for data in val.data {
            //     let commodity_info = CommodityInfo {
            //         stock_id: data.stock_id,
            //         stock_code: data.stock_code.to_owned(),
            //         exchange_id: data.market_id as i32,
            //         currency_no: "data.stock_code".to_owned(),
            //         exchange_code: todo!(),
            //     };

                let commodity_info = CommodityInfo {
                    stock_id: 10773,
                    exchange_id: 101,
                    stock_type: 1,
                    stock_code: "600000".to_owned(),
                    exchange_code: "XSHG".to_owned(),
                    currency_no: "CNY".to_owned(),
                };
                self.commodity_info.insert(commodity_info.stock_id, commodity_info);

                let commodity_info = CommodityInfo {
                    stock_id: 10312,
                    exchange_id: 102,
                    stock_type: 1,
                    stock_code: "000001".to_owned(),
                    exchange_code: "XSHE".to_owned(),
                    currency_no: "CNY".to_owned(),
                };
                self.commodity_info.insert(commodity_info.stock_id, commodity_info);

                let commodity_info = CommodityInfo {
                    stock_id: 40585,
                    exchange_id: 101,
                    stock_type: 1,
                    stock_code: "600941".to_owned(),
                    exchange_code: "XSHG".to_owned(),
                    currency_no: "CNY".to_owned(),
                };
                self.commodity_info.insert(commodity_info.stock_id, commodity_info);
            // }
            if let Some(v) = self.commodity_info.get(&stock_id) {
                return Ok(v.value().to_owned())
            } else {
                log::error!("未找到证券{}的信息", stock_id);
                return Err(anyhow!("未找到证券{}的信息", stock_id));
            }
        }
    }

    //报盘消息
    pub async fn convert_to_orderrouterinfo(sub_order: &PhoenixOrdSuborder, order_detail: &OrderDetail) -> RouterMsg{
        let mut order_msg = OrderMsg::default(); 
        if order_detail.order_flag == OrderType::Place {
            // 下单
            order_msg.order_type       = OrderType::Place as i32;         // 订单类型:见OrderType
            order_msg.order_qty        = sub_order.order_amount;         // 订单数量或撤单数量
            // order_msg.order_id         = sub_order.sub_id;             // 委托id
            order_msg.channel_type     = sub_order.channel_type;         // 通道类型 1 外盘, 2 内盘
            order_msg.channel_id       = sub_order.channel_id as i64;    // 通道id

            order_msg.order_id         = order_detail.order_id;             // 委托id
            order_msg.currency_no      = order_detail.currency_no.to_owned();// 币种
            order_msg.order_price      = order_detail.order_price;         // 委托价格
            order_msg.stock_code       = order_detail.stock_code.to_owned();// 证券代码
            order_msg.exchange_id      = order_detail.exchange_id as i64;    // 市场id
            order_msg.order_direction  = order_detail.order_direction;      // 委托方向  1=buy  2=sell
            order_msg.price_type       = order_detail.price_type;           // 价格类型(市价限价)
        } else {
            // 撤单
            order_msg.order_type       = OrderType::Cancel as i32;         // 订单类型:见OrderType
            order_msg.brk_order_id     = sub_order.confirm_no.to_owned();// 委托确认号
            // order_msg.order_id         = sub_order.sub_id;             // 委托id
            order_msg.channel_type     = sub_order.channel_type;         // 通道类型 1 外盘, 2 内盘
            order_msg.channel_id       = sub_order.channel_id as i64;    // 通道id
            order_msg.price_type       = sub_order.price_type;           // 价格类型(市价限价)

            order_msg.order_id         = order_detail.order_id;             // 委托id
            order_msg.cancel_id        = order_detail.cancel_id;        // 撤单id(新生成的撤单委托序号)
            order_msg.order_qty        = order_detail.cancel_amount;         // 撤单数量(需要计算)
            order_msg.stock_code       = order_detail.stock_code.to_owned();// 证券代码
            order_msg.exchange_id      = order_detail.exchange_id as i64;    // 市场id
            order_msg.order_direction  = order_detail.order_direction;      // 委托方向  1=buy  2=sell
        }

        let mut router_msg = RouterMsg::default();
        router_msg.msg_type = MsgType::Order as i32;
        let mut msg_content = MsgContent::default();
        msg_content.order_msg = Some(order_msg);
        router_msg.msg_content = Some(msg_content);
        router_msg.msg_id   = order_detail.msg_id;  //是否使用同一个?
        router_msg.msg_time = order_detail.msg_time;
        log::info!("转报盘消息: {:#?}", &router_msg);
        router_msg
    }

    pub async fn confirm_order_receipt(&self, exec_msg: &ExecMsg) -> Result<()> {
        log::info!("开始处理委托确认: {:?}", exec_msg);
        let confirm_detail = OrderDetail::receipt_msg(&exec_msg).await;

        if let Err(err) = PhoenixOrdSuborder::confirm_update(&confirm_detail, &self.db_client).await {
            log::error!("{:?}", err);
            return Err(anyhow!("{:?}", err));
        }
        if let Err(err) = PhoenixOrdStockorder::confirm_update(confirm_detail.order_id, &self.db_client).await {
            log::error!("{:?}", err);
            return Err(anyhow!("{:?}", err));
        }
        match PhoenixOrdSuborder::query_sub_order(&confirm_detail, &self.db_client).await {
            Ok(model) => {
                log::info!("修改后: {:?}", model);
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!("{:?}", err));
            }
        }
        log::info!("委托确认处理成功: {}", confirm_detail.order_id);
        Ok(())
    }

    pub async fn filled_order_receipt(&self, exec_msg: &ExecMsg) -> Result<()>{
        log::info!("开始处理成交: {:?}", exec_msg);
        let mut deal_detail = OrderDetail::receipt_msg(&exec_msg).await;
        if let Err(err) = self.order_deal(&mut deal_detail).await {
            log::error!("{:?}", err);
        }
        Ok(())
    }

    pub async fn cancel_order_receipt(&self, exec_msg: &ExecMsg) -> Result<()>{
        log::info!("开始处理撤单: {:?}", exec_msg);
        if exec_msg.channel_id == 0 {
            log::error!("撤单回报有误: {:?}", exec_msg)
        }
        let mut cancel_detail = OrderDetail::receipt_msg(&exec_msg).await;
        // 报盘返回订单号为0,查询  (条件确认号 ?)
        // 报盘返回数量为0时，取子委托中未成交数量
        if cancel_detail.cancel_amount == 0 {
            if let Ok(sub_order) = PhoenixOrdSuborder::query_sub_order(&cancel_detail, &self.db_client).await {
                cancel_detail.cancel_amount = sub_order.order_amount - sub_order.deal_amount;
                log::info!("子订单: order_id: {}, channel_id: {}, channel_type: {}, order_amount: {}, deal_amount: {}, cancel_amount: {}", cancel_detail.order_id, cancel_detail.channel_id, cancel_detail.channel_type, sub_order.order_amount, sub_order.deal_amount, cancel_detail.cancel_amount);
            } else {
                log::info!("没有子订单: order_id: {}, channel_id: {}, channel_type: {}", cancel_detail.order_id, cancel_detail.channel_id, cancel_detail.channel_type);
            }
        }
        if cancel_detail.channel_type == 2 {//内盘待测数量
            if let Ok(sub_order) = PhoenixOrdSuborder::query_sub_order(&cancel_detail, &self.db_client).await {
                cancel_detail.cancel_amount += sub_order.order_amount - sub_order.deal_amount;
                log::info!("子订单: order_id: {}, channel_id: {}, channel_type: {}, order_amount: {}, deal_amount: {}, cancel_amount: {}", cancel_detail.order_id, cancel_detail.channel_id, cancel_detail.channel_type, sub_order.order_amount, sub_order.deal_amount, cancel_detail.cancel_amount);
            } else {
                log::info!("没有子订单: order_id: {}, channel_id: {}, channel_type: {}", cancel_detail.order_id, cancel_detail.channel_id, cancel_detail.channel_type);
            }
        }
        if cancel_detail.cancel_amount <= 0 {
            log::info!("没有可撤数量: {}", cancel_detail.order_id);
            return Ok(());
        }

        // 获取撤单包含的信息
        let ret = PhoenixOrdStockorder::query_cancel_order(cancel_detail.order_id, &self.db_client).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("{:?}", ret.as_ref().err().unwrap().to_string()));
        }
        let cancel_order = ret.unwrap();
        log::info!("待撤订单: {:#?}", &cancel_order);
        cancel_detail.unit_id = cancel_order.unit_id;
        cancel_detail.stock_id = cancel_order.stock_id;
        cancel_detail.order_direction = cancel_order.order_direction;
        cancel_detail.order_amount = cancel_order.order_amount;
        cancel_detail.un_deal_amount = cancel_order.order_amount - cancel_order.deal_amount;
        if cancel_detail.order_amount == cancel_order.cancel_amount {
            log::error!("订单已撤: {}", cancel_detail.order_id);
            return Err(anyhow!("订单已撤: {}", cancel_detail.order_id));
        }
        log::info!("委托状态: {}", cancel_order.order_status);
        // if cancel_order.order_memo.is_some() {
        //     cancel_detail.memo = cancel_order.order_memo.unwrap().to_owned();
        // }

        if cancel_order.order_status == OrderStatus::CANCELED as i32 {//9：已撤
            log::error!("委托{}已撤单，不能撤单", cancel_order.order_no);
            return Ok(());
        } else if cancel_order.order_status == OrderStatus::INVALID as i32{//5：废单
            log::error!("委托{}废单，不能撤单", cancel_order.order_no);
            return Ok(());
        }

        // 判断是否为最后一笔，会影响费用的计算方式
        if cancel_detail.un_deal_amount <= cancel_detail.cancel_amount {
            if cancel_detail.un_deal_amount < cancel_detail.cancel_amount {
                cancel_detail.cancel_amount = cancel_detail.un_deal_amount;
                log::info!("撤单数量: {}", cancel_detail.cancel_amount)
            }
        } else {

        }
        // 计算撤单金额
        let cancel_capital = -(cancel_order.order_price.to_f64().unwrap_or_default() * cancel_detail.cancel_amount as f64);
        log::info!("撤单金额: {}", cancel_capital);
        // 进行证券或资金冻结(通知资产调整)doTradeFrozen
        let _ = self.trade_frozen(&mut cancel_detail).await;

        if let Err(err) = self.cancel_update(&mut cancel_detail, &cancel_order).await {
            log::error!("{:?}", err);
            return Err(anyhow!("{:?}", err));
        }
        log::info!("撤单处理成功: {}", cancel_detail.order_id);
        Ok(())
    }

    pub async fn rejected_order_receipt(&self, exec_msg: &ExecMsg) -> Result<()>{
        log::info!("开始处理废单: {:?}", exec_msg);
        Ok(())
    }


    pub async fn order_deal(&self, detail: &mut OrderDetail) -> Result<()>{
        // 检查成交表是否存在这条记录,存在则已处理(条件?)
        let ret =  PhoenixOrdSuborder::query_sub_order(&detail, &self.db_client).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("{:?}", ret.as_ref().err().unwrap().to_string()));
        }
        let sub_order = ret.unwrap();

        let ret = PhoenixOrdStockorder::query_order(detail.order_id, &self.db_client).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("{:?}", ret.as_ref().err().unwrap().to_string()));
        }
        let order = ret.unwrap();

        // let inited: i32 = format!("{}", OrderStatus::INITED).parse().unwrap_or_default();//未报1
        let invalid: i32 = OrderStatus::INVALID as i32;//废单
        let partialcanceled: i32 =  OrderStatus::PARTIALCANCELED as i32;//部撤
        let canceled: i32 = OrderStatus::CANCELED as i32;//已撤
        //处理买卖
        if order.order_direction == OrderDirection::BUY as i32 || 
            order.order_direction == OrderDirection::SELL as i32 {
            detail.sys_date = order.sys_date;
            detail.order_id = order.order_no;
            detail.unit_id = order.unit_id;
            // detail.deal_id = 0;
            detail.order_direction = order.order_direction;
            detail.pre_fee = order.pre_fee.to_f64().unwrap_or_default();
            detail.order_type = order.order_type;
            detail.stock_id = order.stock_id;
            detail.stock_code = order.stock_code.to_owned();
            detail.exchange_id = order.exchange_id;
            detail.un_deal_amount = order.order_amount - order.deal_amount - order.cancel_amount;

            detail.order_amount = sub_order.order_amount;
            detail.order_price = sub_order.order_price.to_f64().unwrap_or_default();
            detail.channel_id = sub_order.channel_id;
            detail.channel_type = sub_order.channel_type;

            detail.deal_value = detail.deal_amount as f64 * detail.deal_price;
            detail.face_value = 1.0;

            if sub_order.channel_type == 2 && (sub_order.order_status == invalid || sub_order.order_status == partialcanceled || sub_order.order_status == canceled) {
                log::info!("子委托已撤单: order_id: {}, channel_id: {}, channel_type: {}, order_status: {}", 
                sub_order.order_no, sub_order.channel_id, sub_order.channel_type, sub_order.order_status);
                return Err(anyhow!("子委托已撤单: order_id: {}, channel_id: {}, channel_type: {}, order_status: {}", 
                sub_order.order_no,sub_order.channel_id, sub_order.channel_type, sub_order.order_status));
            }

            //计算费用
            if let Err(err) = self.calc_fare_info(detail).await {
                log::error!("计算费用出错: {:?}", err);
                return Err(anyhow!("计算费用出错: {:?}", err));
            }
        }
        //关联委托
        if order.relate_order.is_some() {
            //  取关联委托号
        }
        log::info!("{:?}", &detail);
        
        // 计算卖收益?
        if detail.order_direction == OrderDirection::SELL as i32 {
            // 取持仓表均价
            // detail.refer_profit = (detail.deal_price - dAvgPrice) * detail.deal_amount;
        }
        // 成交落库
        let stockdeal = PhoenixOrdStockdeal::convert_to_stockdeal(&detail).await;
        let _ = self.tx_persist.send(PersistData::StockDeal(Box::new(stockdeal))).await;

        //资产调整
        if let Err(err) = self.stock_deal_process(detail).await {
            // 处理失败, 发送到消息中心
            log::error!("{}", err);
            return Err(anyhow!("{:?}", err));  
        }

        if let Err(err) = self.deal_update(detail, &order, &sub_order).await {
            log::error!("{}", err);
            return Err(anyhow!("{:?}", err));  
        }
        log::info!("成交处理成功");
        Ok(())
    }

    pub async fn calc_fare_info(&self, detail: &mut OrderDetail) -> Result<()>{
        let ret = PhoenixSysDictionary::query_dictionary(&self.db_client).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("{:?}", ret.as_ref().err().unwrap().to_string()));
        }
        let model = ret.unwrap();
        log::info!("字典表: {:#?}", model);

        for val in model {
            // 取得某一分项的费用设置
            // [查费用配置表]
            detail.fee_type = val.lemma_item.to_owned();
            match PhoenixOmsFeeset::query_feeset(&detail, &self.db_client).await {
                Ok(fee) => {
                    log::info!("deal_amount: {}, deal_price: {}, capital_ratio: {}, face_value_ratio: {}, amount_ratio: {}", 
                    detail.deal_amount, detail.deal_price, fee.capital_ratio, fee.face_value_ratio, fee.amount_ratio);

                    detail.item_fee = fee.capital_ratio.to_f64().unwrap_or_default() * detail.deal_amount as f64 * detail.deal_price + 
                    fee.face_value_ratio.to_f64().unwrap_or_default() * detail.deal_amount as f64 * detail.face_value + 
                    fee.amount_ratio.to_f64().unwrap_or_default() * detail.deal_amount as f64;
                    log::info!("fee: {}", detail.item_fee);

                    if fee.minimum_fee.to_f64().unwrap_or_default() > 0.001 || fee.maximum_fee.to_f64().unwrap_or_default() > 0.001 {
                        // 当前证券的交易币种 和 当前费用设置的币种 不一致, 需要将费用设置的最高/低费用转成当前交易币种.
                    }

                    // 成交回报中保存费用，在生成待交割数据时使用
                    match &val.lemma_item as &str {
                        "1" => {
                            if fee.serfee_type.eq(&1) { detail.fee_jy = detail.item_fee; }//交易费
                            // detail.fee_jy = detail.item_fee; //交易费
                        },
                        "2" => {
                            if fee.serfee_type.eq(&1) { detail.fee_yh = detail.item_fee; }//印花税
                        },
                        "3" => {
                            if fee.serfee_type.eq(&1) { detail.fee_gh = detail.item_fee; }//过户费
                        },
                        "4" => {
                            if fee.serfee_type.eq(&1) { detail.fee_yj = detail.item_fee; }//佣金
                        },
                        "5" => {
                            if fee.serfee_type.eq(&1) { detail.fee_js = detail.item_fee; }//经手费
                        },
                        "6" => {
                            if fee.serfee_type.eq(&1) { detail.fee_zg = detail.item_fee; }//证管费
                        },
                        "7" => {
                            if fee.serfee_type.eq(&1) { detail.fee_qt = detail.item_fee; }//其他费用
                        },
                        // "9" => {
                        //     if fee.serfee_type.eq(&1) { detail.fee_fx = detail.item_fee; }//风险金
                        // },
                        "a" => {
                            if fee.serfee_type.eq(&1) { detail.fee_js2 = detail.item_fee; }//结算费
                        },
                        "b" => {
                            if fee.serfee_type.eq(&1) { detail.fee_jg = detail.item_fee; }//交割费
                        },
                        // "c" => {
                        //     detail.fee_jy = detail.item_fee; //实际佣金
                        // },
                        // "d" => {
                        //     detail.fee_jy = detail.item_fee; //券商佣金
                        // },
                        _ => {},
                    }

                },
                Err(err) => {
                    log::error!("{}", err);
                }
            }
        }
        detail.fee_total = detail.fee_jy + detail.fee_yh + detail.fee_gh + detail.fee_yj + detail.fee_js + 
        detail.fee_zg + detail.fee_qt + detail.fee_js2 + detail.fee_jg;
        log::info!("fee_total: {}", detail.fee_total);
        Ok(())
    }

    pub async fn stock_deal_process(&self, detail: &mut OrderDetail) -> Result<()>{
        let ret = self.get_stock_info(detail).await;
        if ret.as_ref().is_err() {
            log::error!("{}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!("{:?}", ret.as_ref().err().unwrap().to_string()));
        }

        // 结算数量 = detail.deal_amount;
        // 资金
        detail.deal_value = detail.deal_amount as f64 * detail.deal_price;
        
        if detail.order_direction == OrderDirection::SELL as i32 {
            // 卖时需多扣费用
            detail.capital = detail.deal_value - detail.fee_total;
        } else {
            // 买时需多收费用
            detail.capital = detail.deal_value + detail.fee_total;
        }

        // 准备结算需要的东西 InitPendSettleInfo
        if let Err(err) = self.init_pend_settle_info(detail).await {
            log::error!("{}", err);
            return Err(anyhow!("{:?}", err));
        }

        detail.un_deal_amount = detail.un_deal_amount - detail.deal_amount;
        log::info!("订单未成交部分: {}", detail.un_deal_amount);
        if detail.un_deal_amount <= 0 {// 完全成交
            // 最后一笔将剩余的委托时的冻结都逆掉 ===>资产
        }
        log::info!("股票债券买卖成交处理 un_deal_amount: {}", detail.un_deal_amount);
        // 持仓信息 InitStockPosition
        // pSP->iDate = ptDealData->iDate;
        // pSP->iExchangeId = ptDealData->iExchangeId;
        // pSP->iStockId = ptDealData->iStockId;
        // pSP->iUnitId = ptDealData->iUnitId;
        // pSP->iChannelId = ptDealData->iChannelId;
        // 资产信息 InitUnitAsset
        // pUA->iDate = iDate;
        // pUA->iAccountId = iAccountId;
        // pUA->iUnitId = iUnitId;
        // 持仓处理
        // 资金处理
        let mut assets_req = PhoenixassetscenterRequest::default();
        assets_req.message_id = detail.msg_id;
        assets_req.operator_id = detail.operator_no;
        assets_req.business_flag = AssetChangeType::TradeOrderAsset as i32;//交易产生的资产变动
        assets_req.unit_id = detail.unit_id;
        let mut postion = PhoenixassetspostionrequestInfo::default();
        postion.stock_id = detail.stock_id;
        let mut asset = PhoenixassetscenterRequestInfo::default();
        if detail.order_direction == OrderDirection::BUY as i32{
            // 买成交：  预买减少，资金临时解冻，资金减少（费用），持仓增加，持仓冻结（A股冻结，港股不冻结）
            log::info!("持仓增加, 资金减少");
            postion.op_type = AssetChangeDirection::AddPosition as i32; //持仓增加
            postion.deal_amount = detail.deal_amount;
            postion.fee_value = detail.fee_total;
            postion.deal_price = detail.deal_price;
            // postion.qfii_state = 
            // postion.margin_rate = 
            postion.frozen_amount = detail.deal_amount;
            postion.temp_frozen_amount = detail.deal_amount;
            postion.prebuy_amount =  -detail.deal_amount;
            // postion.presale_amount = 
            assets_req.postions.push(postion.clone());
            // postion.op_type = AssetChangeDirection::FrozenPosition as i32; //持仓冻结
            // postion.deal_amount = 0;
            asset.change_amount = detail.capital;
            asset.op_type = AssetChangeDirection::ReduceCapital as i32;//资金减少
            assets_req.assets.push(asset.to_owned());
            asset.change_amount = detail.capital;
            asset.op_type = AssetChangeDirection::UnFrozenCapital as i32;//资金解冻
            assets_req.assets.push(asset.to_owned());
        } else {
            //卖成交： 持仓预卖减少，持仓减少，持仓临时冻结减少，资金增加（盈利部分-费用），当日卖增加，
            log::info!("持仓减少, 资金增加");
            postion.op_type = AssetChangeDirection::ReducePosition as i32; //持仓减少
            postion.deal_amount = detail.deal_amount;
            postion.fee_value = detail.fee_total;
            postion.deal_price = detail.deal_price;
            // postion.qfii_state = 
            // postion.margin_rate = 
            postion.frozen_amount = -detail.deal_amount;
            postion.temp_frozen_amount = -detail.deal_amount;
            postion.presale_amount = -detail.deal_amount;
            assets_req.postions.push(postion.clone());
            // postion.op_type = AssetChangeDirection::UnFrozenPosition as i32; //持仓解冻
            // postion.deal_amount = 0;
            asset.change_amount = detail.capital;
            asset.op_type = AssetChangeDirection::AddCapital as i32;//资金增加
            assets_req.assets.push(asset.to_owned());
            asset.change_amount = detail.capital;
            asset.op_type = AssetChangeDirection::FrozenCapital as i32;//资金冻结
            assets_req.assets.push(asset.to_owned());
        }

        Ok(())
    }


    //进行证券或资金冻结
    pub async fn trade_frozen(&self, detail: &mut OrderDetail) -> Result<()>{
        let ret = self.get_stock_info(detail).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!("{:?}", ret.as_ref().err().unwrap().to_string()));
        }
        //更新持仓
        //更新资产
        if detail.order_direction == OrderDirection::SELL as i32 {
            // 委托卖撤单： 持仓预卖减少，资金临时冻结减少
   
            log::info!("{:#?}", &detail);
        } else {
            // 委托买撤单： 持仓预买减少，资金临时冻结减少
            log::info!("{:#?}", &detail);
        }
        Ok(())
    }


    pub async fn init_pend_settle_info(&self, detail: &mut OrderDetail) ->Result<()>{
        log::info!("根据成交回报数据{}生成对应的待交割处理数据", detail.exec_id);
        let ret = PhoenixOmsTradeconfig::query_trade_config(detail, &self.db_client).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!("{:?}", ret.as_ref().err().unwrap().to_string()));
        }
        let trade_config = ret.unwrap();
        if let Err(err) = OrderCenterController::isvalid_trade_config(&trade_config).await{
            log::error!("{:?}", err);
            return Err(anyhow!("{:?}", err)); 
        }

        //计算日期信息redis?
        // 资产处理日期标识计算资产处理日期
        if trade_config.asset_settle_date.eq(&0) {//交易日处理资产

        }  else if trade_config.asset_settle_date.eq(&1) {//交割日处理资产

        }
        Ok(())

    }

    // 判断是否为有效的业务配置数据
    pub async fn isvalid_trade_config(trade_config: &PhoenixOmsTradeconfig) -> Result<()>{
        if trade_config.clear_speed.ne("0") && trade_config.clear_speed.ne("1") && trade_config.clear_speed.ne("2") && trade_config.clear_speed.ne("N") {
            log::error!("清算配置有误[0, T+1, T+2]: {}", trade_config.clear_speed);
        }
        if trade_config.asset_settle_date.ne(&0) && trade_config.asset_settle_date.ne(&1) {
            log::error!("无效的资产处理日期: {}", trade_config.asset_settle_date);
            return Err(anyhow!("无效的资产处理日期: {}", trade_config.asset_settle_date));
        }
        if trade_config.asset_settle_point.ne(&0) && trade_config.asset_settle_point.ne(&1) && trade_config.asset_settle_point.ne(&2) {
            log::error!("无效的资产处理时点: {}", trade_config.asset_settle_point);
            return Err(anyhow!("无效的资产处理时点: {}", trade_config.asset_settle_point));
        }
        if trade_config.market_cash_add_type.ne(&0) && trade_config.market_cash_add_type.ne(&1) && trade_config.market_cash_add_type.ne(&2) {
            log::error!("无效的本市场头寸增加方式: {}", trade_config.market_cash_add_type);
            return Err(anyhow!("无效的本市场头寸增加方式: {}", trade_config.market_cash_add_type));
        }
        //判断是否没有取到对应的配置
        if trade_config.clear_speed.eq("0") && trade_config.asset_settle_point.eq(&0) && trade_config.market_cash_add_type.eq(&0) {
            log::error!("找不到对应的业务配置: {:?}", trade_config);
            return Err(anyhow!("找不到对应的业务配置: {:?}", trade_config));
        }
        Ok(())
    }

    pub async fn get_stock_info(&self, detail: &mut OrderDetail) -> Result<()>{
        let ret = self.query_stock_info(detail.stock_id).await;
        if ret.as_ref().is_err() {
            log::error!("{}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!("{:?}", ret.as_ref().err().unwrap().to_string()));
        }
        let commodity_info = ret.unwrap();
        detail.exchange_id = commodity_info.exchange_id;
        detail.exchange_code =  commodity_info.exchange_code.to_owned();
        detail.stock_code = commodity_info.stock_code.to_owned();
        detail.currency_no = commodity_info.currency_no.to_owned();

        let mut currency = Currency::Hkd as i32;
        if detail.currency_no.contains("CNY") {
            currency = Currency::Cny as i32;
        }else if detail.currency_no.contains("USD") {
            currency = Currency::Usd as i32;
        }
        //查询汇率
        // let ret = self.akacenter_client.query_exchange_rate(currency).await;
        // if ret.as_ref().is_err() {
        //     return Err(anyhow!("{:?}", ret.as_ref().err().unwrap().to_string()));
        // }
        // let exchange_rate = ret.unwrap(); 
        // if detail.order_direction == OrderDirection::BUY as i32 {
        //     detail.rate = exchange_rate.buy_rate;
        // } else {
        //     detail.rate = exchange_rate.sell_rate;
        // }
        Ok(())
    }







































    //委托落库
    // async fn convert_to_stockorder(order_detail: &OrderDetail) -> PhoenixOrdStockorder {
    //     let mut stock_order = PhoenixOrdStockorder::new();
    //     // stock_order.id
    //     stock_order.order_no = order_detail.order_id;
    //     stock_order.sys_date = order_detail.sys_date;
    //     stock_order.unit_id = order_detail.unit_id;
    //     stock_order.stock_id = order_detail.stock_id;
    //     stock_order.stock_code = order_detail.stock_code.to_owned();                    // 接口查询,缓存读取
    //     stock_order.exchange_id = order_detail.exchange_id;                            // 接口查询,缓存读取
    //     stock_order.order_direction = order_detail.order_direction;
    //     stock_order.order_price = Decimal::from_f64(order_detail.order_price).unwrap_or_default();
    //     stock_order.order_amount = order_detail.order_amount;
    //     stock_order.order_type = order_detail.order_type;
    //     stock_order.pre_fee = Decimal::from_f64(order_detail.pre_fee).unwrap_or_default();       //需要计算
    //     stock_order.pre_capital = Decimal::from_f64(order_detail.pre_capital).unwrap_or_default();   //需要计算
    //     stock_order.price_type = order_detail.price_type;
    //     stock_order.deal_amount = order_detail.deal_amount; //0
    //     stock_order.deal_value = Decimal::from_f64(order_detail.deal_value).unwrap_or_default();//0
    //     stock_order.deal_fee = Decimal::from_f64(order_detail.deal_fee).unwrap_or_default();//0
    //     stock_order.cancel_amount = 0;
    //     stock_order.order_status = order_detail.order_status; //下单 1:未报
    //     stock_order.first_deal_time = 0;
    //     // stock_order.relate_order = 0;
    //     stock_order.trade_type = 1;
    //     stock_order.order_memo = Some(order_detail.memo.to_owned());
    //     stock_order.create_time = order_detail.create_time;
    //     stock_order.modify_time = 0;
    //     log::info!("委托单: {:#?}", &stock_order);
    //     stock_order
    // }

    //子订单落库
    // async fn convert_to_suborder(order_detail: &OrderDetail) -> PhoenixOrdSuborder {
    //     let mut sub_order = PhoenixOrdSuborder::default();
    //     // sub_order.id = NotSet;
    //     sub_order.sub_id = order_detail.sub_id;
    //     sub_order.sys_date = order_detail.sys_date;
    //     sub_order.order_no = order_detail.order_id;
    //     sub_order.unit_id = order_detail.unit_id;
    //     sub_order.channel_id = order_detail.channel_id;   //需要根据实际赋值
    //     sub_order.channel_type = order_detail.channel_type; //需要根据实际赋值
    //     sub_order.order_amount = order_detail.order_amount;
    //     sub_order.order_price = Decimal::from_f64(order_detail.order_price).unwrap_or_default();
    //     sub_order.price_type = order_detail.price_type;
    //     sub_order.confirm_no = order_detail.confirm_no.to_owned();
    //     sub_order.order_status = order_detail.order_status; // 未报
    //     sub_order.deal_value = Decimal::from_f64(order_detail.deal_value).unwrap_or_default();
    //     sub_order.deal_amount = order_detail.deal_amount; //下单时成交量为0
    //     sub_order.modify_time = 0;
    //     sub_order.create_time = order_detail.create_time;
    //     sub_order.cancel_flag = 0;
    //     // sub_order.remark = "".to_owned();
    //     sub_order
    // }

    // pub async fn convert_to_assets(order_detail: &OrderDetail) ->PhoenixassetscenterRequest{
    //     let mut assets_req = PhoenixassetscenterRequest::default();
    //     let mut change_capital = PhoenixassetscenterRequestInfo::default();
    //     let mut change_postion = PhoenixassetspostionrequestInfo::default();
    //     assets_req.message_id = order_detail.msg_id;
    //     assets_req.operator_id = order_detail.operator_no;
    //     assets_req.business_flag = 3;
    //     assets_req.unit_id = order_detail.unit_id;

    //     if order_detail.order_direction == common::constant::OrderDirection::BUY as i32 {
    //         //资金
    //         change_capital.change_amount = order_detail.pre_capital;
    //         change_capital.op_type = 105;
    //         change_capital.memo = "买单交易冻结".to_string();

    //         //持仓
    //         change_postion.op_type = 201;
    //         change_postion.prebuy_amount = order_detail.order_amount;
    //     } else {
    //         //资金
    //         change_capital.change_amount = order_detail.pre_capital;
    //         change_capital.op_type = 101;
    //         change_capital.memo = "卖单".to_string();

    //         //持仓
    //         change_postion.op_type = 202;
    //         change_postion.deal_amount = order_detail.order_amount;
    //         change_postion.presale_amount = order_detail.order_amount;
    //         change_postion.frozen_amount = order_detail.order_amount;
    //         change_postion.temp_frozen_amount = order_detail.order_amount;

    //     }
    //     change_postion.stock_id = order_detail.stock_id;
    //     // change_postion.position_flag =   ;
    //     // change_postion.fee_value =   ;
    //     // change_postion.deal_price =   ;
    //     // change_postion.qfii_state =   ;
    //     // change_postion.margin_rate =   ;
    //     assets_req.assets.push(change_capital);
    //     assets_req.postions.push(change_postion);
    //     assets_req

    // }

}