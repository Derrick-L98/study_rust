use super::service::accountassetsservice::PhoenixAccountAssetsService;
use super::service::accountstockpositionservice::PhoenixAccountStockPositionService;
use super::service::basiccacheservice::BasicCacheService;
use super::service::commonservice::CommonService;
use super::service::userassetsservice::UserAssetsService;
use super::service::userpositionservice::UserPositionService;
use super::service::{assetsservice::AssetsDataServie, basedataservice::BaseDataService};
use crate::config::settings::Settings;
use crate::dataview::userassets::UserAssetsData;
use crate::dataview::userposition::UserPositionData;
use crate::protofiles::assetscenter::phoenixassetscenter_client::PhoenixassetscenterClient;
use crate::protofiles::assetscenter::{PhoenixassetsResultInfo, PhoenixassetscenterQueryRequest, Phoenixassetspostioninfo};
// use crate::protofiles::assetscenter::phoenixassetscenter_client::PhoenixassetscenterClient;
use crate::protofiles::phoenixaccountriskcenter::{
  MarginRatioReq, MarginRatioResp, PhoenixAccountAssetsInfo, PhoenixAccountQueryRequest, PhoenixAccountResetRequest, PhoenixAssetsResponse, PhoenixStockPositionRequest, PhoenixStockPositions, PhoenixTransferRequest,
  UserAssetsReq, UserAssetsResp, UserPositionReq, UserPositionResp,
};
use crate::{
  app::constdata,
  dataservice::{dbsetup::DbConnection, entities::prelude::*},
};
use anyhow::{anyhow, Result};
use common::akaclient::AkaClient;
use common::constant::{self, OrderDirection, TradeType};
use messagecenter::protofiles::hqmsg::YsHqInfo;
use messagecenter::protofiles::phoenixnotification::{NotificationAsset, NotificationOrderExec, NotificationPosition, OrderExecType};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::time::SystemTime;
use utility::errors;
use utility::timeutil::current_timestamp;
// use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tonic::Status;

#[derive(Debug)]
pub enum PersistData {
  AccountAssets(Box<Vec<PhoenixAccountAssets>>),
  AccountAssetsHis(Box<Vec<PhoenixAccountAssetsHis>>),
  AccountPosition(Box<Vec<PhoenixStockPositionChannel>>),
  AccountPositionHis(Box<Vec<PhoenixStockPositionChannelHis>>),
  UserAssets(Box<Vec<PhoenixUserAssets>>),
  UserAssetsHis(Box<Vec<PhoenixUserAssetsHis>>),
  TransDetail(Box<PhoenixTransDetail>),
  ResetDetail(Box<PhoenixAccountResetDetail>),
}

#[derive(Clone)]
pub struct PhoenixController {
  pub dbconn: Arc<DbConnection>, //数据库连接
  pub assets_svc: Arc<AssetsDataServie>,
  pub basedata_svc: Arc<BaseDataService>,
  pub setting: Arc<RwLock<Settings>>,
  pub account_position_svc: Arc<PhoenixAccountStockPositionService>,
  pub account_assets_svc: Arc<PhoenixAccountAssetsService>,
  pub aka_svc: Arc<AkaClient>, //基础数据服务
  pub sys_info: PhoenixSysSystem,
  pub tx_persist: mpsc::Sender<PersistData>,
  pub ignore_accounts: Vec<i64>,
  pub ignore_fee_accounts: Vec<i64>,
  pub user_assets_svc: Arc<UserAssetsService>,
  pub user_position_svc: Arc<UserPositionService>,
  pub basic_cache_svc: Arc<BasicCacheService>,
  pub tx_assets: tokio::sync::mpsc::Sender<i64>,
}

impl PhoenixController {
  pub async fn init(&self) -> Result<()> {
    let setting_rd = self.setting.read().await;
    // let mut assets_client_wr = self.assets_client.write().await;
    //初始化用户资产和持仓
    //self.assets_svc.init(&self.basedata_svc, &setting_rd, &self.dbconn).await.expect("init base data error");

    //-----------------新--------------------
    //连接资产服务，获取资产信息
    let asset_server_url = setting_rd.system.assets_server.clone();
    let mut assets_client = PhoenixassetscenterClient::connect(asset_server_url).await.expect("资产中心无法连上");
    let unitid_vec = Vec::new();
    let param = PhoenixassetscenterQueryRequest {
      unit_id: unitid_vec,
      query_type: common::constant::QueryAssetsType::Both as i32,
    };
    let ret_result = assets_client.phoenix_assets_query(param).await;
    if ret_result.as_ref().is_err() {
      log::error!("资产初始化失败，查询资产服务失败{:?}", &ret_result);
      return Err(anyhow!("Error:{:?}", ret_result.as_ref().err().unwrap().to_string()));
    }
    let ret_resp = ret_result.unwrap().into_inner();
    if ret_resp.ret_code != 0 {
      log::error!("资产初始化失败，查询资产服务返回错误{:?}", &ret_resp);
      return Err(anyhow!("Error:{:?}", ret_resp));
    }
    let assets_results = ret_resp.assetsinfo.to_owned();
    let mut assets_vec: Vec<PhoenixassetsResultInfo> = Vec::new();
    let mut positions_vec: Vec<Phoenixassetspostioninfo> = Vec::new();
    for val in assets_results.iter() {
      if val.assets.is_some() {
        let ast = val.assets.as_ref().unwrap().to_owned();
        assets_vec.push(ast);
      }
      positions_vec.extend(val.positionsinfo.to_owned());
    }
    self.user_assets_svc.init(&assets_vec, &self.aka_svc).await.expect("init user assets error");
    self.user_position_svc.init(&positions_vec, &self.aka_svc).await.expect("init user position error");
    //重算资产所有人
    _ = self.tx_assets.send(0).await;
    //-----------------END OF 新--------------------

    //初始化分帐户资产
    self.account_assets_svc.init(&self.dbconn).await.expect("init account assets error");
    //初始化分帐户持仓
    self.account_position_svc.init(&self.dbconn).await.expect("init account stock position error");

    Ok(())
  }

  pub async fn get_stock_positions_quotation_key(&self) -> HashMap<String, i32> {
    let mut qkeys: HashMap<String, i32> = HashMap::new();
    let pos_stocks = self.account_position_svc.get_account_stock_positions(constant::VALUE_ALL);

    for val in pos_stocks {
      if let Ok(mkt) = self.aka_svc.query_market_info(val.p_exchange_id).await {
        let key = format!("stock.{}.{}_{}", mkt.market_type, val.p_stock_code, mkt.market_code);
        *qkeys.entry(key).or_insert(0) += 1;
      }
    }

    qkeys
  }

  pub async fn handle_assets_by_dealinfo(&self, order: &NotificationOrderExec) -> Result<()> {
    if self.ignore_accounts.iter().any(|&x| x == order.unit_id) {
      log::info!("当前账号:{},属于忽略账号,不处理", order.unit_id);
      return Err(anyhow!("当前账号:{},属于忽略账号,不处理", order.unit_id));
    }

    if order.order_id <= 0 {
      return Err(anyhow!("order_id error:{:?}", &order.order_id));
    }

    //股票信息
    let ret = self.aka_svc.query_stock_info(order.stock_id).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!("queyr stock info error:{}", ret.as_ref().err().unwrap().to_string()));
    }
    let stockinfo = ret.unwrap();
    log::info!("股票信息:{:?}", &stockinfo);
    let stocktype: i32 = stockinfo.stock_type;
    //汇率信息
    let ret = self.aka_svc.query_exchange_rate(&stockinfo.trade_currency().as_str_name().to_string()).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!("queyr stock info error:{}", ret.as_ref().err().unwrap().to_string()));
    }
    let rateinfo = ret.unwrap();
    log::info!("汇率信息:{:?}", &rateinfo);

    //产品通道保证金比率
    let ret = self.aka_svc.query_stock_channel_margin(order.stock_id, order.channel_id as i64).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!("queyr stock channel leverage info error:{}", ret.as_ref().err().unwrap().to_string()));
    }
    let leverage = Decimal::from_f64(ret.unwrap()).unwrap_or_default();
    log::info!("产品ID:{},通道id:{}.保证金比率：{}", order.stock_id, order.channel_id, leverage);

    //通道信息
    let ret = self.aka_svc.query_channel_info(order.channel_id as i64).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!("queyr channel info error:{}", ret.as_ref().err().unwrap().to_string()));
    }
    let channelinfo = ret.unwrap();
    log::info!("channel info:{:?}", &channelinfo);
    let accountid = channelinfo.unit_id;

    //参考汇率用买方向
    let currency_rate: Decimal = Decimal::from_f64(rateinfo.buy_rate).unwrap_or_default();
    //成交汇率根据实际交易方向
    let mut currency_rate_cj: Decimal = Decimal::from_f64(rateinfo.buy_rate).unwrap_or_default();
    if order.order_direction == constant::OrderDirection::SELL as i32 {
      currency_rate_cj = Decimal::from_f64(rateinfo.sell_rate).unwrap_or_default();
    }

    let mut dealdetail = CommonService::convert_dealinfo_to_dealdetail(&order, leverage, accountid, currency_rate, currency_rate_cj, stocktype, channelinfo.qfii_state);

    match order.exec_type() {
      OrderExecType::NewOrder => match order.order_direction {
        x if x == OrderDirection::BUY as i32 => {
          log::info!("新订单买单 处理预买量");
          let ret = self.update_positions_prebuy_amount(&dealdetail).await;
          if ret.as_ref().is_err() {
            return Err(anyhow!("更新预买量错误:{}", ret.as_ref().err().unwrap().to_string()));
          }
        }
        x if x == OrderDirection::SELL as i32 => {
          log::info!("新订单卖单，冻结可用（临时冻结）");
          let ret = self.account_position_svc.update_account_positions_frozen_amount(&dealdetail, 0, dealdetail.p_deal_amount).await;
          if ret.as_ref().is_err() {
            return Err(anyhow!("更新分帐户冻结错误:{}", ret.as_ref().err().unwrap().to_string()));
          }
        }
        _ => {
          log::error!("unsupported order direction");
        }
      },
      OrderExecType::OrderFill => {
        let ret = PhoenixDealDetail::insert(&dealdetail, &self.dbconn).await;
        if ret.as_ref().is_err() {
          return Err(anyhow!("真实成交信息，写入数据库错误:{}", ret.as_ref().err().unwrap().to_string()));
        }
        match order.order_direction {
          x if x == OrderDirection::BUY as i32 => {
            log::info!("成交买单，处理预买量，计算资产，增加持仓，冻结可用");
            //减少预买量
            dealdetail.p_prebuy_amount = -dealdetail.p_prebuy_amount;
            let ret = self.update_positions_prebuy_amount(&dealdetail).await;
            if ret.as_ref().is_err() {
              return Err(anyhow!("更新预买量错误:{}", ret.as_ref().err().unwrap().to_string()));
            }
            //买单成交，释放临时冻结，增加冻结
            let ret = self
              .account_position_svc
              .update_account_positions_frozen_amount(&dealdetail, dealdetail.p_deal_amount, -dealdetail.p_deal_amount)
              .await;
            if ret.as_ref().is_err() {
              return Err(anyhow!("更新分帐户冻结错误:{}", ret.as_ref().err().unwrap().to_string()));
            }
          }
          x if x == OrderDirection::SELL as i32 => {
            // log::info!("成交卖单，处理预卖量，计算盈亏");
          }
          _ => {
            log::error!("unsupported order direction");
          }
        }

        let ret = self.update_assets_and_position_by_deal(&dealdetail).await;
        if ret.as_ref().is_err() {
          return Err(anyhow!("更新分帐户冻结错误:{}", ret.as_ref().err().unwrap().to_string()));
        }
      }
      OrderExecType::OrderCancelled => match order.order_direction {
        x if x == OrderDirection::BUY as i32 => {
          log::info!("处理撤单买单 处理预买量，更新可用量");
          dealdetail.p_prebuy_amount = -dealdetail.p_prebuy_amount;
          let ret = self.update_positions_prebuy_amount(&dealdetail).await;
          if ret.as_ref().is_err() {
            return Err(anyhow!("更新预买量错误:{}", ret.as_ref().err().unwrap().to_string()));
          }
        }
        x if x == OrderDirection::SELL as i32 => {
          log::info!("处理撤单 卖单，释放临时冻结");
          let ret = self.account_position_svc.update_account_positions_frozen_amount(&dealdetail, 0, -dealdetail.p_deal_amount).await;
          if ret.as_ref().is_err() {
            return Err(anyhow!("更新分帐户冻结错误:{}", ret.as_ref().err().unwrap().to_string()));
          }
        }
        _ => {
          log::error!("unsupported order direction");
        }
      },
      _ => {
        log::error!("不支持的订单执行回报类型:{}", order.exec_type);
      }
    }
    Ok(())
  }

  pub async fn update_stock_positions_lastprice_by_quotation(&self, stockid: i64, lastprice: &Decimal) -> Result<()> {
    //更新用户持仓的最新价
    let price = lastprice.to_f64().unwrap_or_default();
    let _ = self.user_position_svc.update_positions_last_price(price, stockid).await;

    //更新分帐户持仓的最新价
    let _ret2 = self.account_position_svc.update_account_positions_last_price(lastprice, stockid).await;

    Ok(())
  }

  pub async fn query_account_assets(&self, req: &PhoenixAccountQueryRequest) -> Result<Vec<PhoenixAccountAssetsInfo>> {
    let mut ret: Vec<PhoenixAccountAssetsInfo> = Vec::new();

    let account_assets = self.account_assets_svc.get_account_assets(req.account_id);

    for val in account_assets {
      let ret_info = CommonService::convert_accountassets_to_assetsinfo(&val);
      ret.push(ret_info);
    }

    Ok(ret)
  }

  pub async fn update_assets_by_position(&self) -> Result<()> {
    //更新品种在通道的保证金比率

    //更新用户账户得资产
    let positions = self.user_position_svc.query_user_positions(constant::VALUE_ALL, constant::VALUE_ALL).await;
    let _ret = self.user_assets_svc.update_user_assets_by_positions(constant::VALUE_ALL, &positions).await;

    //获取所有得持仓信息
    let account_positions = self.account_position_svc.get_account_stock_positions(constant::VALUE_ALL);
    self
      .account_assets_svc
      .summarize_account_assets_from_account_positions(constant::VALUE_ALL, &account_positions, self.sys_info.init_date)
      .await;
    //更新总账户数据
    // let total_account = user_assets_svc.get_user_total_assets(self.total_account).await;
    // let _ = account_assets_svc.update_total_account_assets(&total_account).await;
    Ok(())
  }

  //把所有的分帐户持仓数据，用最新价替换均价
  //把对应分帐户的浮动盈亏加到实际盈亏
  //重新计算分帐户的资产数据
  pub async fn reset_profit(&self, accountid: i64, req: &PhoenixAccountResetRequest) -> Result<()> {
    log::info!("开始处理reset......");
    //1)获取交收汇率
    //2)重新计算当前的浮动盈亏,并把浮动盈亏增加到实际盈亏(浮动盈亏=0，实际盈亏+=浮动盈亏)
    //3)更新分帐户持仓的均价和持仓成本(用最新价代替均价,然后用均价*数量代替total_value)
    //4)保存reset历史记录到数据库
    // let currency_rate = self.redis_ctl.get_rate_by_key("RATE_CNY_HKD_JS").await;
    // log::info!("当前的交收汇率:{}", currency_rate);

    // log::info!("start to reset account:{}", accountid);
    // let mut account_position_svc = self.account_position_ctl.write().await;
    // let mut account_asset_svc = self.account_assets_ctl.write().await;
    // let mut tradedate_svc = self.tradedate_ctl.write().await;

    //before reset, re-summarize all assets
    // let account_positions = account_position_svc.get_account_stock_positions(accountid);
    // log::info!("account positions before reset:{:#?}", &account_positions);
    // if let Ok(()) = account_asset_svc
    //     .summarize_account_assets_from_account_positions(accountid, &account_positions, &tradedate_svc)
    //     .await
    // {
    //reset account positions
    let new_financial_borrowed = self.account_position_svc.reset_profit(accountid).await;
    log::info!("分帐户资产reset后得已借:{}", new_financial_borrowed);
    //reset account assets
    let assetprofit = self.account_assets_svc.reset_profit(accountid, new_financial_borrowed).await;
    log::info!("持仓reset后得结果:{:?}", assetprofit);
    //generate reset detail
    let reset_detail = PhoenixAccountResetDetail {
      id: 0,
      p_account_unit_id: accountid,
      p_float_profit_before: assetprofit.1,
      p_current_profit_before: assetprofit.0,
      p_float_profit_after: Decimal::from(0),
      p_current_profit_after: assetprofit.1 + assetprofit.0,
      p_current_principal: assetprofit.2,
      p_account_no: req.operator_id,
      p_datetime: current_timestamp(),
      p_remark: "".to_string(),
    };
    // reset_detail.id = 0;
    // reset_detail.p_account_unit_id = accountid;
    // reset_detail.p_current_profit_before = assetprofit.0;
    // reset_detail.p_float_profit_before = assetprofit.1;
    // reset_detail.p_float_profit_after = Decimal::from(0);
    // reset_detail.p_current_profit_after = assetprofit.1 + assetprofit.0;
    // reset_detail.p_current_principal = assetprofit.2;
    // reset_detail.p_datetime = current_timestamp() as u64;
    // reset_detail.p_account_no = req.operator_id;

    //send reset detail to persist command
    log::info!("reset detail is:{:#?}", &reset_detail);
    let _ = self.tx_persist.send(PersistData::ResetDetail(Box::new(reset_detail))).await;

    //persist to database: account assets, account positions
    let account_assets = self.account_assets_svc.get_account_assets(accountid);
    let account_assets_his: Vec<PhoenixAccountAssetsHis> = account_assets.iter().map(|x| CommonService::convert_accountassets_to_accountassetshis(x, self.sys_info.init_date)).collect();

    // log::info!("开始保存资产数据......");
    let _ = self.tx_persist.send(PersistData::AccountAssets(Box::new(account_assets))).await;
    let _ = self.tx_persist.send(PersistData::AccountAssetsHis(Box::new(account_assets_his))).await;

    //send account position to persist command
    let account_positions = self.account_position_svc.get_account_stock_positions(accountid);
    let current_his_account_positions: Vec<PhoenixStockPositionChannelHis> = account_positions.iter().map(|x| CommonService::convert_stockpositions_to_stockpositionhis(x, self.sys_info.init_date)).collect();
    // log::info!("开始保存持仓数据......");
    let _ = self.tx_persist.send(PersistData::AccountPosition(Box::new(account_positions))).await;
    let _ = self.tx_persist.send(PersistData::AccountPositionHis(Box::new(current_his_account_positions))).await;

    Ok(())
  }

  pub async fn persist_data_interval(&self) -> Result<()> {
    //定时保存数据，主要需要保存的数据包括：1)用户资产数据;2)分帐户资产数据;3)分帐户持仓数据;
    // let user_assets = user_assets_svc.get_user_assets(constant::VALUE_ALL);
    // let _ = self.tx_persist.send(PersistData::UserAssets(Box::new(user_assets))).await;
    let account_assets = self.account_assets_svc.get_account_assets(constant::VALUE_ALL);
    let _ = self.tx_persist.send(PersistData::AccountAssets(Box::new(account_assets))).await;
    let account_positions = self.account_position_svc.get_account_stock_positions(constant::VALUE_ALL);
    let _ = self.tx_persist.send(PersistData::AccountPosition(Box::new(account_positions))).await;
    Ok(())
  }

  pub async fn persist_data(&self, persist_data: &PersistData) -> Result<()> {
    // log::info!("start to persist data ......{:?}", &persist_data);
    let now = SystemTime::now();
    match persist_data {
      PersistData::AccountAssets(data) => {
        let mut mut_data = data.to_owned();
        for val in mut_data.iter_mut() {
          val.p_updatetime = current_timestamp();
        }
        let ret = PhoenixAccountAssets::save_many(&mut_data, &self.dbconn).await;
        if ret.as_ref().is_err() {
          log::error!("update account assets error: {:?}", ret.as_ref().err());
        }
      }
      PersistData::AccountAssetsHis(datas) => {
        let mut mut_data = datas.to_owned();
        for val in mut_data.iter_mut() {
          val.p_updatetime = current_timestamp();
        }
        let ret = PhoenixAccountAssetsHis::save_many(&mut_data, &self.dbconn).await;
        if ret.as_ref().is_err() {
          log::error!("save account assets into history error: {:?}", ret.as_ref().err());
        }
      }
      PersistData::AccountPosition(data) => {
        let ret = PhoenixStockPositionChannel::save_many(&data, &self.dbconn).await;
        if ret.as_ref().is_err() {
          log::error!("update account stock position error: {:?}", ret.as_ref().err());
        }
      }
      PersistData::AccountPositionHis(datas) => {
        let ret = PhoenixStockPositionChannelHis::save_many(&datas, &self.dbconn).await;
        if ret.as_ref().is_err() {
          log::error!("save account position into history error: {:?}", ret.as_ref().err());
        }
      }
      PersistData::UserAssets(data) => {
        // log::info!("saveing user assets: {}", &data.len());
        // let mut mut_data = data.to_owned();
        // for val in mut_data.iter_mut() {
        //   val.p_updatetime = current_timestamp();
        // }
        // let ret = PhoenixUserAssets::update_many(&data, &self.dbconn).await;
        // if ret.as_ref().is_err() {
        //   log::error!("insert user assets error: {:?}", ret.as_ref().err());
        // }
      }
      PersistData::UserAssetsHis(data) => {
        // log::info!("saveing user assets history: {}", &data.len());
        // let mut mut_data = data.to_owned();
        // for val in mut_data.iter_mut() {
        //   val.p_updatetime = current_timestamp();
        // }
        // let ret = PhoenixUserAssetsHis::save_many(&data, &self.dbconn).await;
        // if ret.as_ref().is_err() {
        //   log::error!("insert user assets error: {:?}", ret.as_ref().err());
        // }
      }
      PersistData::ResetDetail(data) => {
        let ret = PhoenixAccountResetDetail::insert(&data, &self.dbconn).await;
        if ret.as_ref().is_err() {
          log::error!("save reset detail error:{:?}", ret.as_ref().err());
        }
      }
      PersistData::TransDetail(data) => {
        // log::info!("save fund transfer detail: {:?}", &data);
        let ret = PhoenixTransDetail::insert(&data, &self.dbconn).await;
        if ret.as_ref().is_err() {
          log::error!("save trans detail error:{:?}", ret.as_ref().err());
        }
      }
    }
    log::info!("persist completed, elapsed: {}", now.elapsed().unwrap().as_secs_f32());
    Ok(())
  }

  async fn update_positions_prebuy_amount(&self, dealdetail: &PhoenixDealDetail) -> Result<()> {
    let ret = self.account_position_svc.update_account_positions_prebuy_amount(&dealdetail, &self.sys_info, &self.dbconn).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!("更新分帐户预买量错误:{}", ret.as_ref().err().unwrap().to_string()));
    }
    let (account_position, is_new) = ret.unwrap();
    if is_new {
      //insert new into his table
      let his_account_position = CommonService::convert_stockpositions_to_stockpositionhis(&account_position, self.sys_info.preinit_date);
      log::info!("需要写入的分帐户持仓信息:{:#?}", his_account_position);
      let his_account_positions = vec![his_account_position];
      let _ = self.tx_persist.send(PersistData::AccountPositionHis(Box::new(his_account_positions))).await;
    }
    let account_positions = vec![account_position];
    let _ = self.tx_persist.send(PersistData::AccountPosition(Box::new(account_positions))).await;

    Ok(())
  }

  async fn update_assets_and_position_by_deal(&self, dealinfo: &PhoenixDealDetail) -> Result<()> {
    //根据订单信息计算资产数据
    //如果是内撮合的订单,则不处理分账户资产,但需要处理用户资产。
    //T0交易,则不需要处理用户资产,但要处理分帐户资产
    //还要判断通道类型，是否

    let ap_ret = self.account_position_svc.update_account_stock_positions_by_dealinfo(&dealinfo, &self.sys_info, &self.dbconn).await;
    if ap_ret.is_err() {
      log::error!("update account stock position error");
      return Err(anyhow!("update account stock position error"));
    }
    let ap_ret = ap_ret.unwrap();

    let aa_ret = self.account_assets_svc.update_account_assets_by_dealinfo(&dealinfo, &ap_ret, &self.sys_info).await;
    if aa_ret.is_err() {
      log::error!("update account assets error");
      return Err(anyhow!("update account assets error"));
    }

    Ok(())
  }
  //运营账户，分帐户和分帐户之间不能直接划转
  pub async fn transfer_fund(&self, req: &PhoenixTransferRequest) -> Result<PhoenixAssetsResponse> {
    let mut res = PhoenixAssetsResponse {
      ret_code: errors::get_error_code(errors::ErrorCode::CodeOk).0,
      ret_msg: errors::get_error_code(errors::ErrorCode::CodeOk).1,
      data: vec![],
    };

    if req.target_account == constant::VALUE_ALL {
      log::error!("target account shoule not be 0");
      res.ret_code = errors::get_error_code(errors::ErrorCode::CodeSystemNotPermitted).0;
      res.ret_msg = errors::get_error_code(errors::ErrorCode::CodeSystemNotPermitted).1;
      res.data = vec![];
      return Ok(res);
    }

    // let account_asset = account_assets[0];
    let transfer_detail = PhoenixTransDetail {
      id: 0,
      p_account_target: req.target_account,
      p_account_source: req.source_account,
      p_op_flag: req.transfer_type,
      p_capital_type: req.capital_type,
      p_account_no: req.operator_id,
      p_remark: req.memo.to_owned(),
      p_trans_value: Decimal::from_f64(req.transfer_value).unwrap_or_default(),
      p_datetime: current_timestamp(),
    };

    match self.account_assets_svc.transfer_fund(&transfer_detail).await {
      Ok(_) => {
        //send data to persist command
        let mut account_target = self.account_assets_svc.get_account_assets(transfer_detail.p_account_target);
        let source_target = self.account_assets_svc.get_account_assets(transfer_detail.p_account_source);
        account_target.extend(source_target);
        let _ = self.tx_persist.send(PersistData::AccountAssets(Box::new(account_target))).await;
        let _ = self.tx_persist.send(PersistData::TransDetail(Box::new(transfer_detail))).await;
        Ok(res)
      }
      Err(e) => {
        log::error!("trans fund error:{}", &e);
        res.ret_code = errors::get_error_code(errors::ErrorCode::CodeSystemNotPermitted).0;
        res.ret_msg = format!("trans fund error:{}", &e);
        res.data = vec![];
        return Ok(res);
      }
    }
    // Ok(res)
  }

  pub async fn query_stock_positions(&self, req: &PhoenixStockPositionRequest) -> Result<Vec<PhoenixStockPositions>> {
    // let mut ret: Vec<PhoenixStockPositions> = Vec::new();

    let account_assets = self.account_position_svc.query_stock_positions(req.stock_id, req.channel_id).await;

    let ret: Vec<PhoenixStockPositions> = account_assets
      .iter()
      .map(|x| PhoenixStockPositions {
        stock_id: x.p_stock_id,
        channel_id: x.p_channel_id,
        current_amount: x.p_current_amount as i64,
        prebuy_amount: x.p_prebuy_amount as i64, //预买数量
        frozen_amount: x.p_frozen_amount as i64,
        frozen_amount_temp: x.p_frozen_amount_temp as i64,
        total_value: x.p_total_value.to_f64().unwrap_or_default(),
        total_value_hkd: x.p_total_value_hkd.to_f64().unwrap_or_default(),
        stock_type: x.p_stock_type,
        is_qfii: x.p_qfii_state,
      })
      .collect();

    Ok(ret)
  }

  //--------------------（新）用户账户相关-------------------------
  pub async fn check_margin_rate(&self, unitid: i64, stockid: i64, marginrate: f64) -> Result<()> {
    //根据用户品种保证金变化，品种保证金变化，停牌，融资杠杆变化，确定新的保证金比率。
    //如果unitid为0，则表示跟品种相关（品种保证金变化，停牌）
    //如果stockid为0，则表示跟用户相关(融资杠杆变化)
    //如果两者都不为0，则表示是用户品种保证金变化
    //返回结果：unitid, stockid, marginrate(最终)
    let ret = self.aka_svc.query_account_info(unitid).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!("query account info error:{}", ret.as_ref().err().unwrap().to_string()));
    }
    let creditmultiple = ret.unwrap().level_rate.parse::<f64>().unwrap_or_default();
    let ret = self.aka_svc.query_unit_stock_margin(unitid, stockid).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!("query account info error:{}", ret.as_ref().err().unwrap().to_string()));
    }
    let unitmargininfo = ret.unwrap();

    //确定保证金后，更新用户账户资产和持仓的保证金比率数据
    //.................
    //发送通知，重算用户资产
    // let send_ret = self.tx_assets.send(()).await;
    // if send_ret.as_ref().is_err() {
    //   return Err(anyhow!("send error:{}", send_ret.as_ref().err().unwrap().to_string()));
    // }
    Ok(())
  }
  //收到资产变更消息
  pub async fn update_user_assets_by_notification(&self, nassets: &NotificationAsset) {
    // let setting_rd = self.setting.read().await;
    // let data = AssetsDataServie::convert_notificationasset_to_data(&req).await;
    let data = UserAssetsData::convert_notificationasset_to_assetsdata(&nassets);
    let ret = self.user_assets_svc.update_user_assets(&data, &self.aka_svc).await;
    if ret.as_ref().is_err() {
      log::error!("update user assets error");
    }
    //重算资产
    let send_ret = self.tx_assets.send(data.unit_id).await;
    if send_ret.as_ref().is_err() {
      log::error!("send error:{}", send_ret.as_ref().err().unwrap().to_string());
    }
  }
  //收到持仓变更消息
  pub async fn update_user_position_by_notification(&self, npositions: &NotificationPosition) {
    let user_posinfo = UserPositionData::convert_notificationposition_to_userpositioninfo(npositions, &self.aka_svc).await;
    let re_calc = self.user_position_svc.update_user_positions(&user_posinfo).await;
    if re_calc {
      //需要重算资产......
      let send_ret = self.tx_assets.send(user_posinfo.unit_id).await;
      if send_ret.as_ref().is_err() {
        log::error!("send error:{}", send_ret.as_ref().err().unwrap().to_string());
      }
    }
  }

  pub async fn re_calculate_user_assets(&self, unit_id: i64) -> Result<()> {
    //重算用户的资产信息,包括风险率
    // 1)从用户持仓模块获取所有的持仓数据（根据unitid)
    // 2)计算用户资产和风险率 update_user_assets_by_positions
    let mut vec_unit = Vec::new();
    if unit_id > 0 {
      vec_unit.push(unit_id);
    } else {
      vec_unit = self.user_assets_svc.get_all_unit_id();
    }
    for i in vec_unit {
      let p_vec = self.user_position_svc.query_user_positions(i, 0).await;
      self.user_assets_svc.compute_user_assets(i, &p_vec);
    }

    Ok(())
  }
  //------------------------END of (新)-------------------------

  //----------------------用户账户相关----------------------------
  //接收推送设置品种的保证金，
  pub async fn set_stock_margin_rate(&self, stock_id: i64, rate: f64) {
    let setting_rd = self.setting.read().await;
    let flag = self.basedata_svc.set_stock_margin_rate(stock_id, rate).await;
    //若设置成功需要重新计算持仓的品种保证金
    if flag {
      self.assets_svc.calculation_position_margin_rate(0, stock_id, &self.basedata_svc).await;
      //重算资产
      self.assets_svc.calculation_unit_assets(0, stock_id, &self.basedata_svc, &setting_rd, &self.dbconn).await;
    }
  }

  //接受推送设置用户品种的保证金比例
  pub async fn set_unit_stock_margin_rate(&self, unit_id: i64, stock_id: i64, rate: f64) {
    let setting_rd = self.setting.read().await;
    let flag = self.basedata_svc.set_user_stock_margin_rate(unit_id, stock_id, rate).await;
    if flag {
      self.assets_svc.calculation_position_margin_rate(unit_id, 0, &self.basedata_svc).await;
      //重算资产
      self.assets_svc.calculation_unit_assets(unit_id, 0, &self.basedata_svc, &setting_rd, &self.dbconn).await;
    }
  }

  //接受推送设置用户的杠杠比例
  pub async fn set_unit_credit_multiple(&self, unit_id: i64, rate: f64) {
    let setting_rd = self.setting.read().await;
    self.assets_svc.set_unit_credit_multiple(unit_id, rate, &self.basedata_svc, &setting_rd, &self.dbconn).await;
    self.assets_svc.calculation_unit_assets(unit_id, 0, &self.basedata_svc, &setting_rd, &self.dbconn).await;
  }

  //查询用户品种保证金比例
  pub async fn query_margin_ratio(&self, _req: &MarginRatioReq) -> Result<MarginRatioResp, Status> {
    let mut result = MarginRatioResp { ..Default::default() };

    let credit_multiple = self.assets_svc.get_user_level_num(_req.unit_id).await;
    if credit_multiple.is_none() {
      result.ret_code = constdata::DEFAULT_ERROR_CODE.to_string();
      result.ret_msg = constdata::DEFAULT_USER_NOT_FOUND.to_string();
      return Ok(result);
    }
    let r = self.basedata_svc.get_unit_stock_margin_rate(_req.unit_id, _req.stock_id, credit_multiple.unwrap()).await;

    if let Err(err) = r {
      result.ret_code = constdata::DEFAULT_ERROR_CODE.to_string();
      result.ret_msg = err.to_string();
    } else {
      result.stock_id = _req.stock_id;
      result.unit_id = _req.unit_id;
      result.margin_ratio = r.unwrap();
      result.ret_code = constdata::DEFAULT_SUCCESS_CODE.to_string();
      result.ret_msg = constdata::DEFAULT_SUCCESS.to_string();
    }
    Ok(result)
  }

  //查询用户资产
  pub async fn query_user_asset(&self, req: &UserAssetsReq) -> Result<UserAssetsResp, Status> {
    let mut result = UserAssetsResp { ..Default::default() };

    let r = self.user_assets_svc.query_assets(req).await;
    if let Err(err) = r {
      result.ret_code = constdata::DEFAULT_ERROR_CODE.to_string();
      result.ret_msg = err.to_string();
    } else {
      result = r.unwrap();
      result.ret_code = constdata::DEFAULT_SUCCESS_CODE.to_string();
      result.ret_msg = constdata::DEFAULT_SUCCESS.to_string();
    }
    Ok(result)
  }

  //查询用户持仓
  pub async fn query_user_positions(&self, req: &UserPositionReq) -> Result<UserPositionResp, Status> {
    let mut result = UserPositionResp { ..Default::default() };

    let r = self.user_position_svc.query_user_positioins_rpc(req.unit_id).await;
    result.positions = r;
    result.ret_code = constdata::DEFAULT_SUCCESS_CODE.to_string();
    result.ret_msg = constdata::DEFAULT_SUCCESS.to_string();
    Ok(result)
  }

  //查询通道持仓
  // pub async fn query_channel_position(&self, req: &ChannelPositionReq) -> Result<ChannelPositionResp, Status> {
  //   let mut result = ChannelPositionResp { ..Default::default() };
  //   let ret = self.assets_svc.query_channel_position(req, &self.dbconn).await;
  //   if let Err(err) = ret {
  //     result.ret_code = constdata::DEFAULT_ERROR_CODE.to_string();
  //     result.ret_msg = err.to_string();
  //   } else {
  //     result = ret.unwrap();
  //     result.ret_code = constdata::DEFAULT_SUCCESS_CODE.to_string();
  //     result.ret_msg = constdata::DEFAULT_SUCCESS.to_string();
  //   }
  //   Ok(result)
  // }

  //收到资产变更消息
  pub async fn set_unit_assets(&self, req: &NotificationAsset) {
    let setting_rd = self.setting.read().await;
    let data = AssetsDataServie::convert_notificationasset_to_data(&req).await;
    self.assets_svc.set_unit_assets(data, &self.basedata_svc, &setting_rd, &self.dbconn).await;
  }

  //收到持仓变动消息
  pub async fn set_unit_positions(&self, req: &NotificationPosition) {
    let setting_rd = self.setting.read().await;
    let data = AssetsDataServie::convert_notificationposition_to_positioninfo(&req).await;
    self.assets_svc.set_unit_positions(&data, &self.basedata_svc, &setting_rd, &self.dbconn).await;
  }

  //更新最新价
  pub async fn set_stock_price(&self, stock_code: &String, exchange_id: &String, price: f64) {
    let setting_rd = self.setting.read().await;
    let k = format!("{}_{}", exchange_id, stock_code);
    if !self.basedata_svc.stock_code_map.contains_key(&k) {
      return;
    }
    //更新最新价
    self.basedata_svc.set_stock_last_price(stock_code, exchange_id, price).await;

    let stock_id = self.basedata_svc.stock_code_map.get(&k).unwrap().to_owned();
    self.assets_svc.calculation_unit_assets(0, stock_id, &self.basedata_svc, &setting_rd, &self.dbconn).await;
  }
}
