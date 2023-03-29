use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use anyhow::{anyhow, Ok, Result};
use common::constant::TransDirection;
use dashmap::DashMap;
use messagecenter::protofiles::phoenixnotification::{NotificationAsset, NotificationPosition};
use rust_decimal::Decimal;
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;
use tonic::transport::Channel;
use utility::timeutil;

use super::basedataservice::{BaseDataService, TradeState};
use crate::config::settings::{self, Settings};
use crate::dataservice::dataaccess::*;
use crate::dataservice::dbsetup::DbConnection;
use crate::dataservice::entities::prelude::*;
use crate::dataservice::entities::prelude::{PhoenixRiskDetails, PhoenixStockPositionChannel};
use crate::protofiles::assetscenter::phoenixassetscenter_client::PhoenixassetscenterClient;
use crate::protofiles::assetscenter::{PhoenixassetsResultInfo, PhoenixassetscenterQueryRequest, Phoenixassetspostioninfo};
use crate::protofiles::phoenixaccountriskcenter::{UserAssetsReq, UserAssetsResp};
use serde::Deserialize;
#[derive(Debug, Clone, Deserialize, Default)]
pub struct AssetsData {
  /// 用户id
  pub unit_id: i64,
  /// 当前本金
  pub current_cash: f64,
  /// 冻结资金
  pub frozen_capital: f64,
  /// 交易临时冻结
  pub trade_frozen_capital: f64,
  /// 期初本金
  pub begin_cash: f64,
  /// 在途资金
  pub cash_in_transit: f64,
  /// 币种
  pub currency_no: String,
  /// 信用倍数
  pub credit_multiple: f64,
  //持仓市值
  pub total_value: f64,
  //创业板市值
  pub total_value_cyb: f64,
  //保证金占用
  pub margin_use: f64,
  //持仓盈亏
  pub hold_yk: f64,
  //交易状态
  pub trade_flag: i32,
  //预警线
  pub warn_line: f64,
  //平仓线
  pub close_line: f64,
  //最后更新时间
  pub last_time: i64,
  //风险率
  pub risk_rate: f64,
  //预警触发状态，，0：未触发过预警，1：已触发过预警，2：已触发过平仓线
  pub risk_trigger_state: i32,
  //今日入金
  pub today_deposit: f64,
  //今日出金
  pub today_withdraw: f64,
  //总入金
  pub total_deposit: f64,
  //总出金
  pub total_withdraw: f64,
  //昨日本金
  pub last_cash: f64,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PositionData {
  pub position_no: i64,
  /// 用户id
  pub unit_id: i64,
  /// 证券代码
  pub stock_code: ::prost::alloc::string::String,
  /// 证券id
  pub stock_id: i64,
  /// 市场ID
  pub exchange_id: i64,
  /// 1多 2空
  pub position_flag: i32,
  /// 期初数量
  pub begin_amount: i32,
  /// 当前数量
  pub current_amount: i32,
  /// 冻结数量
  pub frozen_amount: i32,
  /// 临时冻结数量
  pub temp_frozen_amount: i32,
  /// 今买数量
  pub buy_amount: i32,
  /// 今卖数量
  pub sale_amount: i32,
  /// 预买数量
  pub prebuy_amount: i32,
  /// 预卖数量
  pub presale_amount: i32,
  /// 在途持仓数量(买)
  pub buy_in_transit: i32,
  /// 在途持仓数量(卖)
  pub sale_in_transit: i32,
  /// 通道id
  pub channel_id: i64,
  /// 股票类别
  pub stock_type: i32,
  /// 保证金比例
  pub margin_rate: f64,
  /// 开仓成本;
  pub total_value: f64,
  /// 港币开仓成本
  pub total_value_hkd: f64,
  /// qf持仓数量
  pub qfii_amount: i32,
  ///最新价
  pub last_price: f64,
  ///最后更新时间
  pub last_time: i64,
}

#[derive(Clone)]
pub struct AssetsDataServie {
  //资产
  assets_data: DashMap<i64, AssetsData>,
  //持仓
  positions_data: Arc<RwLock<Vec<RwLock<PositionData>>>>,
}

impl AssetsDataServie {
  //初始化
  pub async fn new() -> Self {
    //资产中心
    let ret = AssetsDataServie {
      assets_data: DashMap::new(),
      positions_data: Arc::new(RwLock::new(Vec::new())),
    };
    return ret;
  }

  pub async fn init(&self, basedata_svc: &BaseDataService, setting: &Settings, dbconn: &DbConnection) -> Result<()> {
    let asset_server_url = setting.system.assets_server.clone();
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
    let ret_vec = ret_resp.assetsinfo;
    let currenttime = timeutil::current_timestamp();
    for item in ret_vec.iter() {
      if item.assets.is_none() {
        log::error!("资产初始化失败，用户资产为空{}", &item.unit_id);
        return Err(anyhow!("Error:{}", &item.unit_id));
      }
      let assets_result = item.to_owned();
      let d = assets_result.assets.unwrap();
      let p = assets_result.positionsinfo;

      let assets_data = AssetsDataServie::convert_assetsinfo_to_data(&d, currenttime).await;
      //初始化资产
      self.set_unit_assets(assets_data, basedata_svc, setting, &dbconn).await;

      //初始化持仓
      for i in p {
        let position_data = AssetsDataServie::convert_positioninfo(&i, currenttime).await;
        self.set_unit_positions(&position_data, basedata_svc, setting, &dbconn).await;
      }
    }

    Ok(())
  }

  //接收推送调整用户的杠杠
  pub async fn set_unit_credit_multiple(&self, unit_id: i64, rate: f64, basemodel: &BaseDataService, setting: &Settings, db: &DbConnection) {
    self.assets_data.entry(unit_id).and_modify(|f| {
      f.credit_multiple = rate;
    });
    self.calculation_unit_assets(unit_id, 0, basemodel, setting, db).await;
  }

  //设置用户资产信息
  pub async fn set_unit_assets(&self, assets: AssetsData, basemodel: &BaseDataService, setting: &Settings, db: &DbConnection) {
    let mut assets_data = assets;

    //若不存在,初始化基础信息
    if self.assets_data.contains_key(&assets_data.unit_id) {
      //查询用户基础信息
      let trade_info = basemodel.query_user_trade_state(assets_data.unit_id).await;
      if trade_info.is_none() {
        return;
      }
      let trade_info_data = trade_info.unwrap();
      assets_data.close_line = trade_info_data.close_line;
      assets_data.warn_line = trade_info_data.warn_line;
      assets_data.credit_multiple = trade_info_data.level_num;
      assets_data.trade_flag = trade_info_data.trade_flag;
    }

    let assets_data_clone = assets_data.clone();

    self
      .assets_data
      .entry(assets_data.unit_id)
      .and_modify(|f| {
        if f.last_time > assets_data.last_time {
          return;
        }
        f.begin_cash = assets_data.begin_cash;
        f.current_cash = assets_data.current_cash;
        f.frozen_capital = assets_data.frozen_capital;
        f.trade_frozen_capital = assets_data.trade_frozen_capital;
        f.cash_in_transit = assets_data.cash_in_transit;
        f.credit_multiple = assets_data.credit_multiple;
        f.currency_no = assets_data.currency_no;
        f.last_time = assets_data.last_time;
        f.total_deposit = assets_data.total_deposit;
        f.today_deposit = assets_data.today_deposit;
        f.today_withdraw = assets_data.today_withdraw;
        f.total_withdraw = assets_data.total_withdraw;
        f.last_cash = assets_data.last_cash
      })
      .or_insert(assets_data_clone);

    self.calculation_unit_assets(1, 0, basemodel, setting, db).await;
  }

  //rpc接口查询的资产类转换本服务资产数据
  pub async fn convert_assetsinfo_to_data(assets: &PhoenixassetsResultInfo, lasttime: i64) -> AssetsData {
    AssetsData {
      unit_id: assets.unit_id,
      current_cash: assets.current_cash,
      frozen_capital: assets.frozen_capital,
      trade_frozen_capital: assets.trade_frozen_capital,
      begin_cash: assets.begin_cash,
      cash_in_transit: assets.cash_in_transit,
      credit_multiple: assets.credit_multiple,
      currency_no: assets.currency_no.clone(),
      last_time: lasttime,
      today_deposit: assets.today_deposit,
      today_withdraw: assets.today_withdraw,
      total_deposit: assets.total_deposit,
      total_withdraw: assets.total_withdraw,
      last_cash: assets.last_cash,
      ..Default::default()
    }
  }
  //资产消息推送转换本服务资产数据
  pub async fn convert_notificationasset_to_data(assets: &NotificationAsset) -> AssetsData {
    AssetsData {
      unit_id: assets.unit_id,
      current_cash: assets.current_cash,
      frozen_capital: assets.frozen_capital,
      trade_frozen_capital: assets.trade_frozen_capital,
      begin_cash: assets.begin_cash,
      cash_in_transit: assets.cash_in_transit,
      credit_multiple: assets.credit_multiple,
      currency_no: assets.currency_no.clone(),
      last_time: assets.timestamp,
      today_deposit: assets.today_deposit,
      today_withdraw: assets.today_withdraw,
      total_deposit: assets.total_deposit,
      total_withdraw: assets.total_withdraw,
      last_cash: assets.today_total_value,
      ..Default::default()
    }
  }

  //持仓信息转换
  pub async fn convert_positioninfo(i: &Phoenixassetspostioninfo, currenttime: i64) -> PositionData {
    let position_data = PositionData {
      position_flag: i.position_flag,
      position_no: i.position_no,
      unit_id: i.unit_id,
      sale_amount: i.sale_amount,
      buy_in_transit: i.buy_in_transit,
      sale_in_transit: i.sale_in_transit,
      stock_code: i.stock_code.to_owned(),
      stock_id: i.stock_id,
      stock_type: i.stock_type,
      exchange_id: i.exchange_id,
      begin_amount: i.begin_amount,
      channel_id: i.channel_id,
      current_amount: i.current_amount,
      frozen_amount: i.frozen_amount,
      temp_frozen_amount: i.temp_frozen_amount,
      buy_amount: i.buy_amount,
      prebuy_amount: i.prebuy_amount,
      presale_amount: i.presale_amount,
      margin_rate: i.margin_rate,
      total_value: i.total_value,
      total_value_hkd: i.total_value_hkd,
      qfii_amount: i.qfii_amount,
      last_time: currenttime,
      last_price: 0.0,
    };
    return position_data;
  }

  //持仓信息转换
  pub async fn convert_notificationposition_to_positioninfo(i: &NotificationPosition) -> PositionData {
    let position_data = PositionData {
      position_flag: i.position_flag,
      position_no: i.position_no,
      unit_id: i.unit_id,
      sale_amount: i.sale_amount,
      buy_in_transit: i.buy_in_transit,
      sale_in_transit: i.sale_in_transit,
      stock_code: i.stock_code.to_owned(),
      stock_id: i.stock_id,
      stock_type: i.stock_type,
      exchange_id: i.exchange_id,
      begin_amount: i.begin_amount,
      channel_id: i.channel_id,
      current_amount: i.current_amount,
      frozen_amount: i.frozen_amount,
      temp_frozen_amount: i.temp_frozen_amount,
      buy_amount: i.buy_amount,
      prebuy_amount: i.prebuy_amount,
      presale_amount: i.presale_amount,
      margin_rate: i.margin_rate,
      total_value: i.total_value,
      total_value_hkd: i.total_value_hkd,
      qfii_amount: i.qfii_amount,
      last_time: i.timestamp,
      last_price: 0.0,
    };
    return position_data;
  }

  //设置用户持仓
  pub async fn set_unit_positions(&self, p: &PositionData, basemodel: &BaseDataService, setting: &Settings, db: &DbConnection) {
    let p1 = p.clone();
    //是否已存在该品种仓位
    let mut flag = false;
    //重算资产标志，若持仓数量和未发生变动。不需要重算资产
    let mut cal_flag = false;

    let mut num = 0;

    for i in self.positions_data.read().await.iter() {
      let w = i.read().await;
      if p.unit_id == w.unit_id && p.stock_id == w.stock_id {
        if w.last_time >= p.last_time {
          return;
        } else {
          if p.current_amount != w.current_amount {
            cal_flag = true;
          }
          flag = true;
          break;
        }
      }
      num += 1;
    }

    let mut num1 = 0;
    for i in self.positions_data.read().await.iter() {
      if num1 == num {
        let mut w = i.write().await;
        w.begin_amount = p.begin_amount;
        w.current_amount = p.current_amount;
        w.frozen_amount = p.frozen_amount;
        w.temp_frozen_amount = p.temp_frozen_amount;
        w.buy_amount = p.buy_amount;
        w.qfii_amount = p.qfii_amount;
        w.sale_amount = p.sale_amount;
        w.prebuy_amount = p.prebuy_amount;
        w.presale_amount = p.presale_amount;
        w.buy_in_transit = p.buy_in_transit;
        w.sale_in_transit = p.sale_in_transit;
        w.total_value = p.total_value;
        w.total_value_hkd = p.total_value_hkd;
        w.last_time = p.last_time;
      }
      num1 += 1;
    }

    let unit_id = p1.unit_id.clone();
    if !flag {
      self.positions_data.write().await.push(RwLock::new(p1));
      basemodel.set_stock_code_map(&p.stock_code, p.exchange_id, p.stock_id).await;
      basemodel.query_stock_data(p.stock_id, true).await;
      // basemodel.query_user_stock_margin(p.unit_id, p.stock_id).await;
      cal_flag = true;
    }
    //重算这个用户的资产
    if !cal_flag {
      return;
    }
    self.calculation_unit_assets(unit_id, 0, basemodel, setting, db).await;
  }

  //获取用户的杠杠比例
  pub async fn get_user_level_num(&self, unit_id: i64) -> Option<f64> {
    let mut ret = 0.0;
    if !self.assets_data.contains_key(&unit_id) {
      return None;
    }
    ret = self.assets_data.get(&unit_id).unwrap().credit_multiple.clone();
    return Some(ret);
  }

  //重新计算用户持仓的保证金比例
  pub async fn calculation_position_margin_rate(&self, unit_id: i64, stock_id: i64, basemodel: &BaseDataService) {
    let p_list = self.positions_data.read().await;
    for i in p_list.iter() {
      let r = i.read().await;
      let mut flag = false;
      if stock_id > 0 && r.stock_id == stock_id {
        flag = true;
      }
      if unit_id > 0 && r.unit_id == unit_id {
        flag = true;
      }

      if !self.assets_data.contains_key(&unit_id) {
        continue;
      }

      let credit_multiple = self.assets_data.get(&unit_id).unwrap().credit_multiple.clone();
      if flag {
        let rate_op = basemodel.get_unit_stock_margin_rate(r.unit_id, r.stock_id, credit_multiple).await;
        if let Err(_) = rate_op {
          return;
        }
        let mut w = i.write().await;
        w.margin_rate = rate_op.unwrap();
      }
    }
  }

  //重算资产
  pub async fn calculation_unit_assets(&self, unit_id: i64, stock_id: i64, basemodel: &BaseDataService, setting: &Settings, db: &DbConnection) {
    let p_list = self.positions_data.read().await;

    let buy_rate = *basemodel.cny_hkd_buy_rate.read().await;
    let mut assets_map: HashMap<i64, AssetsData> = HashMap::new();
    for i in p_list.iter() {
      let r = i.read().await;
      //1、重算某个用户的，2：重算某个品种的，3：重算所有用户的
      if !((unit_id == 0 && stock_id == 0) || (stock_id > 0 && r.stock_id == stock_id) || (unit_id > 0 && r.unit_id == unit_id)) {
        continue;
      }

      let unit_assets_op = assets_map.get(&r.unit_id);
      let mut unit_assets = AssetsData { ..Default::default() };
      if unit_assets_op.is_some() {
        unit_assets = unit_assets_op.unwrap().to_owned();
      }

      let price_op = basemodel.comminity_map.get(&r.stock_id);
      if price_op.is_none() {
        continue;
      }
      let price = price_op.unwrap().to_owned().last_price;

      let margin_rate = r.margin_rate;
      let mut t_value = r.current_amount as f64 * price;
      //若是港股

      if r.stock_type != 2 {
        t_value = t_value * buy_rate;
      }
      unit_assets.total_value += t_value;
      unit_assets.margin_use += t_value * margin_rate;
      if r.stock_type == 4 || r.stock_type == 5 {
        unit_assets.total_value_cyb += t_value;
      }
      unit_assets.hold_yk += t_value - r.total_value_hkd;

      assets_map
        .entry(r.unit_id)
        .and_modify(|f| {
          f.total_value = unit_assets.total_value;
          f.margin_use = unit_assets.margin_use;
          f.hold_yk = unit_assets.hold_yk;
          f.total_value_cyb = unit_assets.total_value_cyb;
        })
        .or_insert(unit_assets);
    }

    for i in assets_map.iter() {
      self.assets_data.entry(i.0.to_owned()).and_modify(|f| {
        f.total_value = i.1.total_value;
        f.margin_use = i.1.margin_use;
        f.hold_yk = i.1.hold_yk;
        f.total_value_cyb = i.1.total_value_cyb;
      });

      let f_op = self.assets_data.get(i.0);
      if f_op.is_some() {
        let f = f_op.unwrap();
        //若风险率超过预警线
        let rate = f.margin_use / f.current_cash;

        //风险率高于预警线
        if rate >= f.warn_line {
          //高于预警线禁止交易,正常状态更新为禁止开仓
          if f.trade_flag == 1 {
            self.assets_data.entry(f.unit_id).and_modify(|f| {
              f.trade_flag = 2;
            });
            //调用接口更新状态
          }

          let mut flag = false;
          //今天第一次触发预警或者第一次触发平仓线。出入预警记录
          if f.risk_trigger_state == 0 {
            self.assets_data.entry(f.unit_id).and_modify(|f| {
              f.risk_trigger_state = 1;
            });
            flag = true;
          } else if f.risk_trigger_state == 1 && f.close_line < rate {
            self.assets_data.entry(f.unit_id).and_modify(|f| {
              f.risk_trigger_state = 2;
            });
            flag = true;
          }
          if flag {
            let ret = PhoenixSysSystem::query(db).await;
            if ret.is_err() {
              continue;
            }
            let data = ret.unwrap().init_date;
            let details = PhoenixRiskDetails {
              sys_date: data,
              user_id: f.unit_id,
              current_cash: Decimal::from_f64_retain(f.current_cash).unwrap_or_default(),
              credit_multiple: Decimal::from_f64_retain(f.credit_multiple).unwrap_or_default(),
              warn_line: Decimal::from_f64_retain(f.warn_line).unwrap_or_default(),
              close_line: Decimal::from_f64_retain(f.close_line).unwrap_or_default(),
              risk_rate: Decimal::from_f64_retain(rate).unwrap_or_default(),
              loan_cash: Decimal::from_f64_retain(f.total_value - f.current_cash).unwrap_or_default(),
              total_value: Decimal::from_f64_retain(f.total_value).unwrap_or_default(),
              credit_cash: Decimal::new(0, 0),
              real_cash: Decimal::new(0, 0),
              create_datetime: timeutil::current_timestamp(),
              risk_type: 2,
              total_asset_value: Decimal::new(0, 0),
              id: 0,
            };
            _ = PhoenixRiskDetails::insert(&details, db).await;
          } else {
            // let normal_rate = settings::GLOBAL_CONFIG.otherconfig.normal_risk_rate.clone();
            // let r = normal_rate.parse::<f64>();
            // if r.is_err() {
            //   continue;
            // }
            // let r_rate = r.unwrap();
            let r_rate = setting.system.risk_restore;
            //低于预警线下多少点恢复正常状态,且是禁止开仓状态下
            if rate < f.warn_line * (1.0 - r_rate) {
              if f.trade_flag == 2 {
                self.assets_data.entry(f.unit_id).and_modify(|f| {
                  f.trade_flag = 1;
                });
                //调用接口更新状态
              }
            }
          }
        }
      }
    }
  }

  //提供的rpc接口查询用户资产
  pub async fn query_user_asset(&self, req: &UserAssetsReq, basemodel: &BaseDataService) -> Result<UserAssetsResp> {
    let mut ret = UserAssetsResp { ..Default::default() };

    // let d_op = self.assets_data.get(&req.unit_id);
    // if d_op.is_none() {
    //   return Err(anyhow!("查询用户不存在!"));
    // }
    // let d = d_op.unwrap();

    // ret.unit_id = req.unit_id;
    // ret.total_position_value = d.total_value;
    // ret.gem_position_value = d.total_value_cyb;
    // ret.real_margin = d.margin_use;
    // ret.real_cash = d.current_cash;
    // ret.risk_rate = 0.0;
    // ret.trade_state = d.trade_flag;
    // ret.warning_line = d.warn_line;
    // ret.available_cash = d.current_cash - d.margin_use;
    // ret.net_income = d.today_deposit - d.total_withdraw;
    // ret.hold_yk = d.hold_yk;
    // ret.total_asset = d.current_cash + d.hold_yk - d.frozen_capital;
    // ret.draw_frozen = d.frozen_capital;
    // ret.trade_frozen_capital = d.trade_frozen_capital;
    // ret.today_yk = ret.total_asset - d.last_cash - d.today_deposit + d.today_withdraw;
    // ret.total_yk = ret.total_asset - d.total_deposit + d.total_withdraw;
    // let mut positions = Vec::new();

    // let p_list = self.positions_data.read().await;
    // for i in p_list.iter() {
    //   let r = i.read().await;
    //   if r.unit_id != req.unit_id {
    //     continue;
    //   }
    //   let price_op = basemodel.comminity_map.get(&r.stock_id);
    //   let mut price = 0.0;
    //   if price_op.is_some() {
    //     price = price_op.unwrap().last_price;
    //   }
    //   let position_info = PositionInfo {
    //     unit_id: r.unit_id,
    //     stock_code: r.stock_code.clone(),
    //     stock_id: r.stock_id,
    //     exchange_id: r.exchange_id,
    //     amount: r.current_amount as i64,
    //     frozen_amount: r.frozen_amount as i64,
    //     prebuy_amount: r.prebuy_amount as i64,
    //     qfii_amount: r.qfii_amount as i64,
    //     margin_ratio: r.margin_rate,
    //     total_value_hkd: r.total_value_hkd,
    //     stock_type: r.stock_type,
    //     last_price: price,
    //   };
    //   positions.push(position_info);
    // }

    // ret.positions = positions;
    return Ok(ret);
  }

  //rpc 查询分账户持仓
  // pub async fn query_channel_position(&self, req: &ChannelPositionReq, db: &DbConnection) -> Result<ChannelPositionResp> {
  //   let mut ret = ChannelPositionResp { ..Default::default() };
  //   let ret_list = PhoenixStockPositionChannel::query_many(0, req.stock_id, req.channel_id, db).await;
  //   if let Err(err) = ret_list {
  //     return Err(err);
  //   }
  //   let list = ret_list.unwrap();
  //   let mut p_vec = Vec::new();
  //   for i in list {
  //     let item = ChannelPositionInfo {
  //       stock_code: i.p_stock_code,
  //       stock_id: i.p_stock_id as i64,
  //       exchange_id: i.p_exchange_id,
  //       channel_id: i.p_channel_id,
  //       amount: i.p_current_amount as i64,
  //       is_qfii: false,
  //     };
  //     p_vec.push(item);
  //   }
  //   ret.positions = p_vec;
  //   Ok(ret)
  // }

  //汇总所有用户的资产
  pub async fn query_all_units_cash(&self) -> AssetsData {
    let mut ret = AssetsData { ..Default::default() };
    for assets in self.assets_data.iter() {
      ret.begin_cash += assets.begin_cash;
      ret.cash_in_transit += assets.cash_in_transit;
      ret.current_cash += assets.current_cash;
      ret.frozen_capital += assets.frozen_capital;
      ret.hold_yk += assets.hold_yk;
      ret.last_cash += assets.last_cash;
      ret.today_deposit += assets.today_deposit;
      ret.today_withdraw += assets.today_withdraw;
      ret.total_deposit += assets.today_deposit;
      ret.total_withdraw += assets.total_withdraw;
      ret.trade_frozen_capital += assets.trade_frozen_capital;
      ret.total_value += assets.total_value;
      ret.total_value_cyb += assets.total_value_cyb;
      ret.margin_use += assets.margin_use;
    }
    return ret;
  }
}
