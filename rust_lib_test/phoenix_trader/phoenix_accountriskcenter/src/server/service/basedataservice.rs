// use std::collections::{HashMap, HashSet};

use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use dashmap::DashMap;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::Deserialize;
use tokio::sync::RwLock;
use tonic::transport::Channel;

use crate::{
  config::settings::Settings,
  protofiles::phoenixakacenter::{phoenix_aka_center_client::PhoenixAkaCenterClient, AccountInfoReq, ExchangeRateReq, StockInfoReq, UserStockMarginReq},
};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CommidityData {
  pub id: i64,
  pub code: String,
  pub exchange_id: i64,
  pub last_price: f64,
  pub margin_rate: f64,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TradeState {
  //交易状态
  pub trade_flag: i32,
  //融资杠杠
  pub level_num: f64,
  //预警线
  pub warn_line: f64,
  //平仓线
  pub close_line: f64,
}

#[derive(Debug, Clone)]
pub struct BaseDataService {
  pub akacenter_client: Arc<RwLock<PhoenixAkaCenterClient<Channel>>>, //基础数据服务客户端
  pub cny_hkd_buy_rate: Arc<RwLock<f64>>,                             //买汇率
  pub cny_hkd_sell_rate: Arc<RwLock<f64>>,                            //卖汇率
  pub comminity_map: DashMap<i64, CommidityData>,                     //品种集合
  pub user_stock_margin_rate: DashMap<String, f64>,                   //用户品种的保证金比例
  pub stock_code_map: DashMap<String, i64>,                           //品种code对应id的集合
}

impl BaseDataService {
  pub async fn new(config: &Settings) -> Self {
    let serverurl = config.system.aka_server.clone();
    let akacenter_client = PhoenixAkaCenterClient::connect(serverurl).await.expect("基础服务无法连上");
    let ret = BaseDataService {
      akacenter_client: Arc::new(RwLock::new(akacenter_client)),
      cny_hkd_buy_rate: Arc::new(RwLock::new(0.0)),
      cny_hkd_sell_rate: Arc::new(RwLock::new(0.0)),
      comminity_map: DashMap::new(),
      user_stock_margin_rate: DashMap::new(),
      stock_code_map: DashMap::new(),
    };
    return ret;
  }

  //设置stock_code_map
  //600000_XSHG
  pub async fn set_stock_code_map(&self, code: &String, exchange_id: i64, id: i64) {
    let k = format!("{}_{}", exchange_id, code);
    self.stock_code_map.insert(k, id);
  }

  //设置汇率，汇率推送收到通知时更新汇率
  pub async fn set_rate(&self, buy_rate: f64, sell_rate: f64) {
    if buy_rate > 0.0 {
      let mut buy_write = self.cny_hkd_buy_rate.write().await;
      *buy_write = buy_rate;
      // self.cny_hkd_buy_rate = buy_rate;
    }
    if sell_rate > 0.0 {
      let mut sell_write = self.cny_hkd_sell_rate.write().await;
      *sell_write = sell_rate;
      // self.cny_hkd_sell_rate = sell_rate
    }
  }

  //设置品种最新价,行情推送设置最新价
  pub async fn set_stock_last_price(&self, code: &String, exchange_id: &String, price: f64) {
    let k = format!("{}_{}", exchange_id, code);
    if !self.stock_code_map.contains_key(&k) {
      return;
    }
    let stock_id = self.stock_code_map.get(&k).unwrap();
    self.comminity_map.entry(stock_id.to_owned()).and_modify(|f| {
      f.last_price = price;
    });
  }

  //推送接收品种信息发生变更
  pub async fn set_stock_margin_rate(&self, stock_id: i64, rate: f64) -> bool {
    if !self.comminity_map.contains_key(&stock_id) {
      return false;
    }

    let margin_rate = self.comminity_map.get(&stock_id).unwrap().margin_rate;
    if margin_rate == rate {
      return false;
    }

    self.comminity_map.entry(stock_id).and_modify(|f| {
      f.margin_rate = rate;
    });
    return true;
  }

  //接收推送设置用户品种的保证金比例
  pub async fn set_user_stock_margin_rate(&self, unit_id: i64, stock_id: i64, rate: f64) -> bool {
    let k = format!("{}_{}", unit_id, stock_id);
    if !self.comminity_map.contains_key(&stock_id) {
      return false;
    }

    if !self.user_stock_margin_rate.contains_key(&k) {
      return false;
    }
    let rate_source = self.user_stock_margin_rate.get(&k).unwrap();
    if rate_source.to_owned() == rate {
      return false;
    }

    self.user_stock_margin_rate.entry(k).and_modify(|f| *f = rate).or_insert(rate);
    return true;
  }

  //查询人民币兑港币买卖汇率
  pub async fn query_rate(&self) -> Option<(f64, f64)> {
    None
    // let buy_read = self.cny_hkd_buy_rate.read().await;
    // let sell_read = self.cny_hkd_sell_rate.read().await;
    // if *buy_read != 0.0 && *sell_read != 0.0 {
    //   let ret = (*buy_read, *sell_read);
    //   return Some(ret);
    // }

    // let req = ExchangeRateReq { currency: 0 };
    // let result = self.akacenter_client.write().await.query_exchange_rate(req).await;

    // if let Err(err) = result {
    //   log::error!("汇率查询失败！{:?}", err);
    //   return None;
    // }
    // log::info!("{:?}", result);
    // let rate = result.unwrap().into_inner();
    // let mut buy_write = self.cny_hkd_buy_rate.write().await;
    // *buy_write = rate.buy_rate;
    // let mut sell_write = self.cny_hkd_sell_rate.write().await;
    // *sell_write = rate.sell_rate;
    // // self.cny_hkd_buy_rate = rate.buy_rate;
    // // self.cny_hkd_sell_rate = rate.sell_rate;
    // let ret = (rate.buy_rate, rate.sell_rate);

    // return Some(ret);
  }

  //查询品种的属性
  pub async fn query_stock_data(&self, stock_id: i64, flag: bool) -> Option<CommidityData> {
    None
    //   let mut ret = CommidityData { ..Default::default() };

    //   if flag && self.comminity_map.contains_key(&stock_id) {
    //     return Some(self.comminity_map.get(&stock_id).unwrap().to_owned());
    //   }

    //   let req = StockInfoReq { stock_id: stock_id };
    //   let result = self.akacenter_client.write().await.query_stock_info(req).await;
    //   if let Err(err) = result {
    //     log::error!("品种信息查询失败！{:?}", err);
    //     return None;
    //   }
    //   log::info!("{:?}", result);

    //   let data = result.unwrap().into_inner();
    //   if !data.ret_code.eq("0") {
    //     log::error!("query_stock_data查询失败!");
    //     return None;
    //   }

    //   let d = data.data;
    //   if d.len() == 0 {
    //     log::error!("query_stock_data查询失败!");
    //     return None;
    //   }

    //   let d_op = d.get(0);
    //   if d_op.is_none() {
    //     log::error!("query_stock_data查询失败!");
    //     return None;
    //   }
    //   let d = d_op.unwrap();
    //   ret.id = d.stock_id;
    //   ret.exchange_id = d.market_id;
    //   ret.margin_rate = d.margin_rate;
    //   ret.code = d.stock_code.clone();
    //   self.comminity_map.insert(stock_id, ret.clone());
    //   return Some(ret);
    // }

    // //查询用户品种的保证金比例
    // pub async fn query_user_stock_margin(&self, unit_id: i64, stock_id: i64) -> Option<f64> {
    //   let mut ret = 0.0;
    //   let k = format!("{}_{}", unit_id, stock_id);
    //   if self.user_stock_margin_rate.contains_key(&k) {
    //     return Some(self.user_stock_margin_rate.get(&k).unwrap().to_owned());
    //   }

    //   let req = UserStockMarginReq { unit_id: unit_id, stock_id: stock_id };
    //   let result = self.akacenter_client.write().await.query_unit_stock_margin(req).await;
    //   if let Err(err) = result {
    //     log::error!("用户品种保证金信息查询失败！{:?}", err);
    //     return None;
    //   }

    //   let r = result.unwrap().into_inner();
    //   ret = r.margin_rate;

    //   self.user_stock_margin_rate.insert(k, r.margin_rate);

    //   return Some(ret);
  }

  //计算获得用户最终的品种保证金比例
  pub async fn get_unit_stock_margin_rate(&self, unit_id: i64, stock_id: i64, credit_multiple: f64) -> Result<f64> {
    let k = format!("{}_{}", unit_id, stock_id);
    let rate = self.user_stock_margin_rate.get(&k);
    if rate.is_some() {
      return Ok(rate.unwrap().to_owned());
    }

    let mut ret = 0.0;

    let rate = credit_multiple;
    ret = 1.0 / (rate + 1.0);

    //获取品种的保证金比例
    let rate_op = self.comminity_map.get(&stock_id);

    let margin_rate: f64;
    if rate_op.is_some() {
      margin_rate = rate_op.unwrap().margin_rate;
    } else {
      let data_op = self.query_stock_data(stock_id, true).await;
      if data_op.is_none() {
        return Err(anyhow!("查询品种失败！"));
      }
      margin_rate = data_op.unwrap().margin_rate;
    }
    if margin_rate > ret {
      ret = rate;
    }

    return Ok(ret);
  }

  //查询用户的交易状态信息
  pub async fn query_user_trade_state(&self, unit_id: i64) -> Option<TradeState> {
    let mut ret = TradeState {
      trade_flag: 0,
      level_num: 0.0,
      warn_line: 0.0,
      close_line: 0.0,
    };

    let req = AccountInfoReq { unit_id: unit_id };
    let result = self.akacenter_client.write().await.query_account_info(req).await;
    if let Err(err) = result {
      log::error!("query_user_trade_state查询失败！{:?}", err);
      return None;
    }

    let d = result.unwrap().into_inner().data;
    if d.len() == 0 {
      log::error!("query_user_trade_state查询失败！");
      return None;
    }

    let d = d.get(0);
    if d.is_none() {
      log::error!("query_user_trade_state查询失败！");
      return None;
    }

    let data = d.unwrap();

    let rate_r = data.level_rate.parse::<f64>();
    if rate_r.is_err() {
      log::error!("query_user_trade_state查询失败！");
      return None;
    }
    let rate = rate_r.unwrap();
    ret.close_line = data.close_line;
    ret.level_num = rate;
    ret.trade_flag = data.trade_state as i32;
    ret.warn_line = data.warning_line;
    return Some(ret);
  }
}
