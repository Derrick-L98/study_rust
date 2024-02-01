use std::thread;
use std::time::Duration;

use crate::app::constdata;
use crate::controller::PersistData;
// use crate::controller::NotificationData;
use crate::dataservice::dbsetup::DbConnection;
use crate::dataservice::entities::prelude::*;
use crate::protofiles::assetscenter::{PhoenixassetsResult, PhoenixassetsResultInfo, PhoenixassetscenterRequest, PhoenixassetscenterRequestInfo};
use anyhow::{anyhow, Ok, Result};
// use common::akaclient::AkaClient;
use common::constant;
use common::logclient::{LogClient, LogLevel};
use common::redisclient::redisclient::{self, RedisClient};
// use futures::channel::mpsc;
// use log::Log;
// use messagecenter::assetsclient::AssetsChangeClient;
use messagecenter::protofiles::phoenixnotification::NotificationAsset;
use rust_decimal::Decimal;
// use tokio::sync::mpsc;
use utility::timeutil;
// use super::akacenterclient::AkacenterClient;
use super::utils;

pub struct UnitAssetsService {}

//主要业务逻辑：
//1）初始化时，从数据库读取资产并保存到redis
//2) 查询时直接从redis读取（无锁）
//3) 资产调整时，从redis读取数据（加锁），更新后写回redis，并解锁
impl UnitAssetsService {
  pub fn new() -> Self {
    UnitAssetsService {}
  }

  pub async fn init(&self, unit_id: i64, db: &DbConnection, redisclient: &RedisClient) -> Result<Option<PhoenixAstUnitasset>> {
    let ret = PhoenixAstUnitasset::find_by_unitid(unit_id, db).await;
    if let Err(err) = ret {
      _ = LogClient::get().push(LogLevel::Error, format!("db查询资产失败:{}", err).as_str()).await;
      return Err(err);
    }
    let assets_result_ok = ret.unwrap();
    if assets_result_ok.is_some() {
      let assets = assets_result_ok.as_ref().unwrap();
      let _ = self.update_assets(unit_id, assets, redisclient).await;
    }
    Ok(assets_result_ok)
  }

  pub async fn query_assets_by_unitid(&self, unit_id: i64, redisclient: &RedisClient, db: &DbConnection) -> Result<Option<PhoenixAstUnitasset>> {
    let rkey = format!("{}{}", constdata::USER_ASSETS_REDIS_KEY, unit_id);
    let assets = redisclient.get_value_by_get(&rkey).await;
    if !assets.is_empty() {
      let retdata: Result<PhoenixAstUnitasset, serde_json::Error> = serde_json::from_str(&assets);
      if let Err(_) = retdata {
        let _ = LogClient::get().push(LogLevel::Error, format!("get_assets_by_id数据解析失败{}", &assets).as_str()).await;
      } else {
        let ret = retdata.unwrap();
        return Result::Ok(Some(ret));
      }
    }
    self.init(unit_id, db, redisclient).await
  }

  pub async fn update_assets(&self, unit_id: i64, assets: &PhoenixAstUnitasset, redisclient: &RedisClient) -> Result<()> {
    let rkey = format!("{}{}", constdata::USER_ASSETS_REDIS_KEY, unit_id);
    let ret = serde_json::json!(assets).to_string();
    let r = redisclient.set_str_value(&rkey, constdata::EXPARE_TIME_8_HOUR, &ret).await;
    if let Err(err) = r {
      _ = LogClient::get().push(LogLevel::Error, format!("用户{}资产更新异常{:?}", unit_id, err).as_str()).await;
      return Err(err);
    }

    Ok(())
  }

  //创建资产用户
  pub async fn phoenix_assets_crate(
    &self,
    req: &PhoenixassetscenterRequest,
    reqinfo: &Vec<PhoenixassetscenterRequestInfo>,
    redis: &RedisClient,
    db: &DbConnection,
    sysinfo: &PhoenixSysSystem,
  ) -> Result<NotificationAsset> {
    let current_date_ok = sysinfo.init_date;

    let unit_id = req.unit_id;
    for item in reqinfo.iter() {
      let ret = self.assets_create(unit_id, current_date_ok, item, redis, db).await;
      if !ret.is_err() {
        return Result::Ok(ret.unwrap());
      }
    }

    return Result::Ok(NotificationAsset::default());
  }

  //资产更新
  pub async fn phoenix_assets_change(
    &self,
    req: &PhoenixassetscenterRequest,
    reqinfo: &Vec<PhoenixassetscenterRequestInfo>,
    redis: &RedisClient,
    db: &DbConnection,
    sysinfo: &PhoenixSysSystem,
    txpersist: &tokio::sync::mpsc::Sender<PersistData>,
  ) -> Result<NotificationAsset> {
    let lockkey = format!("{}{}", constdata::LOCK_USER_REDIS_KEY, req.unit_id);
    let mut count = 0;
    //获取锁，超过20次取锁失败，不继续获取锁,锁1秒
    while redis.lock(&lockkey, 5).await != 1 && count < 20 {
      thread::sleep(Duration::from_millis(1));
      count += 1;
    }

    let current_date_ok = sysinfo.init_date;

    //查询资产缓存
    let assets_data = self.query_assets_by_unitid(req.unit_id, redis, db).await;
    if let Err(err) = assets_data {
      return Err(err);
    }
    let assets_data_ok = assets_data.unwrap();
    if assets_data_ok.is_none() {
      let _ = LogClient::get().push(LogLevel::Error, format!("用户{}不存在,更新资产失败!", req.unit_id).as_str()).await;
      return Err(anyhow!("用户{}不存在,更新资产失败!", req.unit_id));
    }
    let mut assets_data = assets_data_ok.unwrap();

    let mut flow_vec = Vec::new();
    let mut operation_vec = Vec::new();
    let unit_id = req.unit_id;
    for item in reqinfo.iter() {
      if item.op_type == constant::AssetChangeDirection::AddCapital as i32 || item.op_type == constant::AssetChangeDirection::ReduceCapital as i32 {
        let ret = self.assets_change_cash(&mut assets_data, req, item, current_date_ok).await;
        if let Err(err) = ret {
          return Err(err);
        }
        let retdata = ret.unwrap();
        flow_vec.push(retdata.0);
        operation_vec.push(retdata.1);
      } else if item.op_type == constant::AssetChangeDirection::FrozenCapital as i32 || item.op_type == constant::AssetChangeDirection::UnFrozenCapital as i32 {
        let ret = self.assets_change_frozen_capital(&mut assets_data, req, item, current_date_ok).await;
        if let Err(err) = ret {
          return Err(err);
        }
        let retdata = ret.unwrap();
        flow_vec.push(retdata.0);
        operation_vec.push(retdata.1);
      } else if item.op_type == constant::AssetChangeDirection::FrozenTradeCapital as i32 || item.op_type == constant::AssetChangeDirection::UnFrozenTradeCapital as i32 {
        let ret = self.assets_change_trade_frozen_capital(&mut assets_data, req, item, current_date_ok).await;
        if let Err(err) = ret {
          return Err(err);
        }
        let retdata = ret.unwrap();
        flow_vec.push(retdata.0);
        operation_vec.push(retdata.1);
      } else if item.op_type == constant::AssetChangeDirection::ModifyUnitCredit as i32 {
        let ret = self.assets_change_credit_multiple(&mut assets_data, req, item, current_date_ok).await;
        if let Err(err) = ret {
          return Err(err);
        }
        let retdata = ret.unwrap();
        flow_vec.push(retdata.0);
        operation_vec.push(retdata.1);
      }
    }

    //更新资产缓存
    let ret = self.update_assets(req.unit_id, &assets_data, redis).await;
    if let Err(err) = ret {
      return Err(err);
    }
    //更新资产数据库

    let ret = txpersist.send(PersistData::UserAssets(Box::new(vec![assets_data.clone()]))).await;
    if ret.as_ref().is_err() {
      log::error!("push error:{}", ret.as_ref().err().unwrap().to_string());
    }

    //插入流水记录
    if flow_vec.len() > 0 {
      //发送到管道
      let ret = txpersist.send(PersistData::AssetsFlow(Box::new(flow_vec))).await;
      if ret.as_ref().is_err() {
        log::error!("push error:{}", ret.as_ref().err().unwrap().to_string());
      }
    }
    if operation_vec.len() > 0 {
      let ret = txpersist.send(PersistData::OperationDetail(Box::new(operation_vec))).await;
      if ret.as_ref().is_err() {
        log::error!("push error:{}", ret.as_ref().err().unwrap().to_string());
      }
    }

    //释放锁
    let _ = redis.delele(&lockkey);

    //返回需要推送的数据
    self.phoenix_query_assets_formatdata(unit_id, &assets_data).await
  }

  //创建资产用户
  pub async fn assets_create(&self, unit_id: i64, current_date: i32, req: &PhoenixassetscenterRequestInfo, redisclient: &redisclient::RedisClient, db: &DbConnection) -> Result<NotificationAsset> {
    //查询用户是否存在
    let assets_result = self.query_assets_by_unitid(unit_id, redisclient, db).await;
    if let Err(err) = assets_result {
      _ = LogClient::get().push(LogLevel::Error, format!("查询get_assets_by_id异常{}", err).as_str()).await;
      return Err(err);
    }

    if assets_result.unwrap().is_some() {
      _ = LogClient::get().push(LogLevel::Error, format!("创建用户资产失败,已存在该 用户{}", unit_id).as_str()).await;
      return Err(anyhow!(constdata::PARAM_USER_EXISTS));
    }

    let credit_multiple = req.change_amount.to_string().parse::<Decimal>().unwrap();
    let mut update_assets = PhoenixAstUnitasset {
      credit_multiple: credit_multiple,
      unit_id: unit_id,
      sys_date: current_date,
      currency_no: "HKD".to_string(),
      ..Default::default()
    };

    let last_insert_id = PhoenixAstUnitasset::insert(&update_assets, db).await;
    if let Err(err) = last_insert_id {
      _ = LogClient::get().push(LogLevel::Error, format!("用户数据插入失败{:?}", err).as_str()).await;
      return Err(err);
    }
    //缓存redis
    update_assets.id = last_insert_id.unwrap();
    let _ = self.update_assets(unit_id, &update_assets, redisclient).await;

    self.phoenix_query_assets_formatdata(unit_id, &update_assets).await
  }

  //资产调整
  pub async fn assets_change_cash(
    &self,
    assets_data: &mut PhoenixAstUnitasset,
    req: &PhoenixassetscenterRequest,
    item: &PhoenixassetscenterRequestInfo,
    system_date: i32,
  ) -> Result<(PhoenixOmsAssetflow, PhoenixAstOperationDetail)> {
    let mut cash = item.change_amount;
    if item.op_type == constant::AssetChangeDirection::ReduceCapital as i32 {
      cash = 0.0 - cash;
    }

    //增加本金
    let d = utils::decimal_add(assets_data.current_cash, cash);
    if d.is_none() {
      _ = LogClient::get().push(LogLevel::Error, format!("数据处理异常{},{}", assets_data.current_cash, item.change_amount).as_str()).await;
      return Err(anyhow!(constdata::DATA_ERROR));
    }
    assets_data.current_cash = d.unwrap();
    if item.flag == constant::YesOrNo::YES as i32 {
      //出入金处理
      let today_deposit_op = utils::decimal_add(assets_data.today_deposit, cash);
      let total_deposit_op = utils::decimal_add(assets_data.total_deposit, cash);
      if today_deposit_op.is_none() || total_deposit_op.is_none() {
        _ = LogClient::get()
          .push(LogLevel::Error, format!("数据处理异常{},{}，{}", assets_data.today_deposit, assets_data.total_deposit, item.change_amount).as_str())
          .await;
        return Err(anyhow!(constdata::DATA_ERROR));
      }

      assets_data.today_deposit = today_deposit_op.unwrap();
      assets_data.total_deposit = total_deposit_op.unwrap();
    }

    return self.assets_change_create_flow(assets_data, req, item, system_date).await;
  }

  //资产冻结解冻操作
  pub async fn assets_change_frozen_capital(
    &self,
    assets_data: &mut PhoenixAstUnitasset,
    req: &PhoenixassetscenterRequest,
    item: &PhoenixassetscenterRequestInfo,
    system_date: i32,
  ) -> Result<(PhoenixOmsAssetflow, PhoenixAstOperationDetail)> {
    let mut cash = item.change_amount;
    if item.op_type == constant::AssetChangeDirection::UnFrozenCapital as i32 {
      cash = 0.0 - cash;
    }
    let d = utils::decimal_add(assets_data.frozen_capital, cash);
    if d.is_none() {
      _ = LogClient::get().push(LogLevel::Error, format!("数据处理异常{},{}", assets_data.frozen_capital, item.change_amount).as_str()).await;
      return Err(anyhow!(constdata::DATA_ERROR));
    }

    assets_data.frozen_capital = d.unwrap();
    return self.assets_change_create_flow(assets_data, req, item, system_date).await;
  }

  //资产临时冻结解冻操作
  pub async fn assets_change_trade_frozen_capital(
    &self,
    assets_data: &mut PhoenixAstUnitasset,
    req: &PhoenixassetscenterRequest,
    item: &PhoenixassetscenterRequestInfo,
    system_date: i32,
  ) -> Result<(PhoenixOmsAssetflow, PhoenixAstOperationDetail)> {
    let mut cash = item.change_amount;
    if item.op_type == constant::AssetChangeDirection::UnFrozenTradeCapital as i32 {
      cash = 0.0 - cash;
    }
    let d = utils::decimal_add(assets_data.trade_frozen_capital, cash);
    if d.is_none() {
      _ = LogClient::get().push(LogLevel::Error, format!("数据处理异常{},{}", assets_data.frozen_capital, item.change_amount).as_str()).await;
      return Err(anyhow!(constdata::DATA_ERROR));
    }

    assets_data.trade_frozen_capital = d.unwrap();
    if item.flag == constant::YesOrNo::YES as i32 {
      let gem_frozen_capital_op = utils::decimal_add(assets_data.gem_frozen_capital, cash);
      if gem_frozen_capital_op.is_some() {
        assets_data.gem_frozen_capital = gem_frozen_capital_op.unwrap();
      }
    }
    return self.assets_change_create_flow(assets_data, req, item, system_date).await;
  }

  //资产信用倍数调整
  pub async fn assets_change_credit_multiple(
    &self,
    assets_data: &mut PhoenixAstUnitasset,
    req: &PhoenixassetscenterRequest,
    item: &PhoenixassetscenterRequestInfo,
    system_date: i32,
  ) -> Result<(PhoenixOmsAssetflow, PhoenixAstOperationDetail)> {
    let d_op = Decimal::from_f64_retain(item.change_amount);
    if d_op.is_none() {
      _ = LogClient::get().push(LogLevel::Error, format!("数据处理异常{}", item.change_amount).as_str()).await;
    }
    assets_data.credit_multiple = d_op.unwrap();
    return self.assets_change_create_flow(assets_data, req, item, system_date).await;
  }

  //创建流水记录
  pub async fn assets_change_create_flow(
    &self,
    assets_data: &PhoenixAstUnitasset,
    req: &PhoenixassetscenterRequest,
    item: &PhoenixassetscenterRequestInfo,
    system_date: i32,
  ) -> Result<(PhoenixOmsAssetflow, PhoenixAstOperationDetail)> {
    //创建资金流水表
    let amount = Decimal::from_f64_retain(item.change_amount).unwrap();
    let mark = item.memo.clone();
    let flow = PhoenixOmsAssetflow {
      sys_date: system_date,
      unit_id: assets_data.unit_id,
      busin_flag: req.business_flag,
      occur_capital: amount,
      post_capital: assets_data.current_cash,
      datetime: timeutil::current_timestamp(),
      remarks: item.to_owned().memo,
      op_type: item.op_type,
      currency_no: "HKD".to_string(),
      currency_rate: Decimal::from(1),
      ..Default::default()
    };

    //创建资金操作表记录
    let option_detail = PhoenixAstOperationDetail {
      sys_date: system_date,
      unit_id: req.unit_id,
      op_businflag: req.business_flag.to_string(),
      remark: mark,
      currency_no: "HKD".to_string(),
      create_time: timeutil::current_timestamp(),
      operator: req.operator_id as i32,
      op_type: item.op_type,
      occur_capital: amount,
      ..Default::default()
    };

    let ret = (flow, option_detail);
    Ok(ret)
  }

  pub async fn phoenix_query_assets_formatdata(&self, unit_id: i64, asset: &PhoenixAstUnitasset) -> Result<NotificationAsset> {
    let current_cash = asset.current_cash.to_string().parse::<f64>();
    let frozen_capital = asset.frozen_capital.to_string().parse::<f64>();
    let trade_frozen_capital = asset.trade_frozen_capital.to_string().parse::<f64>();
    let begin_cash = asset.begin_cash.to_string().parse::<f64>();
    let cash_in_transit = asset.cash_in_transit.to_string().parse::<f64>();
    let credit_multiple = asset.credit_multiple.to_string().parse::<f64>();
    let today_deposit = asset.today_deposit.to_string().parse::<f64>();
    let today_withdraw = asset.today_withdraw.to_string().parse::<f64>();
    let total_deposit = asset.total_deposit.to_string().parse::<f64>();
    let total_withdraw = asset.total_withdraw.to_string().parse::<f64>();
    let today_total_value = asset.today_total_value.to_string().parse::<f64>();
    let gem_frozen_capital = asset.gem_frozen_capital.to_string().parse::<f64>();
    if current_cash.is_err()
      || frozen_capital.is_err()
      || trade_frozen_capital.is_err()
      || begin_cash.is_err()
      || cash_in_transit.is_err()
      || credit_multiple.is_err()
      || today_deposit.is_err()
      || today_withdraw.is_err()
      || total_withdraw.is_err()
      || total_deposit.is_err()
      || today_total_value.is_err()
      || gem_frozen_capital.is_err()
    {
      return Err(anyhow!(constdata::DATA_ERROR));
    }

    let ret = NotificationAsset {
      unit_id: unit_id,
      current_cash: current_cash.unwrap(),
      frozen_capital: frozen_capital.unwrap(),
      trade_frozen_capital: trade_frozen_capital.unwrap(),
      begin_cash: begin_cash.unwrap(),
      cash_in_transit: cash_in_transit.unwrap(),
      currency_no: asset.currency_no.to_owned(),
      credit_multiple: credit_multiple.unwrap(),
      timestamp: timeutil::current_timestamp(),
      today_deposit: today_deposit.unwrap(),
      today_withdraw: today_withdraw.unwrap(),
      total_deposit: total_deposit.unwrap(),
      total_withdraw: total_withdraw.unwrap(),
      today_total_value: today_total_value.unwrap(),
      gem_frozen_capital: gem_frozen_capital.unwrap(),
    };

    Result::Ok(ret)
  }

  pub async fn phoenix_query_assets(&self, unit_id: i64, redisclient: &RedisClient, db: &DbConnection) -> Result<Option<PhoenixassetsResultInfo>> {
    let assets = self.query_assets_by_unitid(unit_id, redisclient, db).await;
    if let Err(err) = assets {
      return Err(err);
    }

    let assets_op = assets.unwrap();
    if assets_op.is_none() {
      return Ok(None);
    }
    let asset = assets_op.unwrap();
    let current_cash = asset.current_cash.to_string().parse::<f64>();
    let frozen_capital = asset.frozen_capital.to_string().parse::<f64>();
    let trade_frozen_capital = asset.trade_frozen_capital.to_string().parse::<f64>();
    let begin_cash = asset.begin_cash.to_string().parse::<f64>();
    let cash_in_transit = asset.cash_in_transit.to_string().parse::<f64>();
    let credit_multiple = asset.credit_multiple.to_string().parse::<f64>();
    let today_deposit = asset.today_deposit.to_string().parse::<f64>();
    let total_deposit = asset.total_deposit.to_string().parse::<f64>();
    let today_withdraw = asset.today_withdraw.to_string().parse::<f64>();
    let total_withdraw = asset.total_withdraw.to_string().parse::<f64>();
    let today_total_value = asset.today_total_value.to_string().parse::<f64>();
    let gem_frozen_capital = asset.gem_frozen_capital.to_string().parse::<f64>();
    if current_cash.is_err()
      || frozen_capital.is_err()
      || trade_frozen_capital.is_err()
      || begin_cash.is_err()
      || cash_in_transit.is_err()
      || credit_multiple.is_err()
      || today_deposit.is_err()
      || today_withdraw.is_err()
      || total_deposit.is_err()
      || total_withdraw.is_err()
      || today_total_value.is_err()
      || gem_frozen_capital.is_err()
    {
      return Err(anyhow!(constdata::DATA_ERROR));
    }

    let ret = PhoenixassetsResultInfo {
      unit_id: unit_id,
      current_cash: current_cash.unwrap(),
      frozen_capital: frozen_capital.unwrap(),
      trade_frozen_capital: trade_frozen_capital.unwrap(),
      begin_cash: begin_cash.unwrap(),
      cash_in_transit: cash_in_transit.unwrap(),
      currency_no: asset.currency_no,
      credit_multiple: credit_multiple.unwrap(),
      today_deposit: today_deposit.unwrap(),
      today_withdraw: today_withdraw.unwrap(),
      total_deposit: total_deposit.unwrap(),
      total_withdraw: total_withdraw.unwrap(),
      last_cash: today_total_value.unwrap(),
      gem_frozen_capital: gem_frozen_capital.unwrap(),
    };
    return Ok(Some(ret));
  }

  pub async fn phoenix_query_all_units(&self, db: &DbConnection, redis: &RedisClient) -> Result<Vec<PhoenixassetsResult>> {
    let mut ret = Vec::new();
    //查询所有用户信息
    let ret_units = PhoenixAstUnitasset::find_all(db).await;
    if let Err(err) = ret_units {
      _ = LogClient::get().push(LogLevel::Error, format!("phoenix_query_all_units资产查询失败{:?}", err).as_str()).await;
      return Err(err);
    }
    let units_vec = ret_units.unwrap();
    for item in units_vec {
      let assets = self.phoenix_query_assets(item.unit_id, redis, db).await;
      if let Err(err) = assets {
        _ = LogClient::get().push(LogLevel::Error, format!("phoenix_query_all_units资产查询失败{:?}", err).as_str()).await;
        return Err(err);
      }
      let result_item = PhoenixassetsResult {
        unit_id: item.unit_id,
        assets: assets.unwrap(),
        ..Default::default()
      };
      ret.push(result_item);
    }
    return Ok(ret);
  }

  //保存数据到db
  pub async fn update_assets_model_data(&self, assets_data: &Vec<PhoenixAstUnitasset>, db: &DbConnection) -> Result<()> {
    for item in assets_data.iter() {
      let ret = PhoenixAstUnitasset::update(item, db).await;
      if ret.is_err() {
        log::error!("update_assets_model_data异常{:?}", ret);
        _ = LogClient::get().push(LogLevel::Error, format!("save_assets_data插入失败{:?}", ret).as_str()).await;
      }
    }
    Ok(())
  }
  //保存流水到db
  pub async fn save_assets_flow_data(&self, flow_data: &Vec<PhoenixOmsAssetflow>, db: &DbConnection) -> Result<()> {
    let ret = PhoenixOmsAssetflow::insert_many(flow_data, db).await;
    if ret.is_err() {
      log::error!("save_assets_flow_data异常{:?}", ret);
      _ = LogClient::get().push(LogLevel::Error, format!("save_assets_flow_data插入失败{:?}", ret).as_str()).await;
    }
    Ok(())
  }

  //保存操作记录到db
  pub async fn save_assets_operations_data(&self, operation_vec: &Vec<PhoenixAstOperationDetail>, db: &DbConnection) -> Result<()> {
    let ret = PhoenixAstOperationDetail::insert_many(&operation_vec, db).await;
    if ret.is_err() {
      log::error!("save_assets_operations_data异常{:?}", ret);
      _ = LogClient::get().push(LogLevel::Error, format!("save_assets_operations_data插入失败{:?}", ret).as_str()).await;
    }
    Ok(())
  }
}
