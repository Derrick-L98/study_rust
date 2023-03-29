use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::{thread, vec};

use crate::controller::PersistData;
// use crate::config::settings::{self, GLOBAL_CONFIG};
use crate::protofiles::assetscenter::{PhoenixassetscenterRequest, Phoenixassetspostioninfo, PhoenixassetspostionrequestInfo};

use crate::dataservice::entities::prelude::*;
use crate::{app::constdata, dataservice::dbsetup::DbConnection};
use anyhow::{anyhow, Ok, Result};
use common::akaclient::AkaClient;
use common::constant;
use common::redisclient::redisclient::RedisClient;
use common::uidservice::{self, UidgenService};
// use messagecenter::assetsclient::{AssetsChangeClient, self};
// use super::akacenterclient::AkacenterClient;
// use super::systemdate_redis;
use common::logclient::{LogClient, LogLevel};
use messagecenter::protofiles::phoenixnotification::{NotificationAsset, NotificationAssetPositions, NotificationPosition};
use rust_decimal::Decimal;
use tokio::sync::RwLock;
use utility::timeutil;

pub struct UnitPositionService {}

//主要业务逻辑：
//1）初始化时，从数据库读取资产并保存到redis
//2) 查询时直接从redis读取（无锁）
//3) 资产调整时，从redis读取数据（加锁），更新后写回redis，并解锁
impl UnitPositionService {
  pub fn new() -> Self {
    UnitPositionService {}
  }

  pub async fn push_log(&self, log: String) {
    let logclient = LogClient::get();
    log::error!("{}", log);
    _ = logclient.push(LogLevel::Error, log.as_str()).await;
  }

  pub async fn init(&self, unit_id: i64, stock_id: i64, db: &DbConnection, redis: &RedisClient) -> Result<Vec<PhoenixAstStockposition>> {
    let ret = PhoenixAstStockposition::query_many(unit_id, stock_id, db).await;
    if let Err(err) = ret {
      self.push_log(format!("查询用户持仓失败:{:?}", err)).await;
      return Err(err);
    }
    let retdata = ret.unwrap();
    let r = retdata.clone();
    for item in retdata {
      _ = self.update_position_redis(unit_id, stock_id, &item, redis);
    }
    Ok(r)
  }

  pub async fn update_position_redis(&self, unit_id: i64, stock_id: i64, position: &PhoenixAstStockposition, redis: &RedisClient) -> Result<()> {
    let mut rkey = constdata::USER_POSITION_KEY.to_string();
    rkey = rkey.replacen("{}", unit_id.to_string().as_str(), 1).replacen("{}", stock_id.to_string().as_str(), 1);
    let ret = serde_json::json!(position).to_string();
    let r = redis.set_str_value(&rkey, constdata::EXPARE_TIME_8_HOUR, &ret).await;
    if let Err(err) = r {
      self.push_log(format!("用户持仓更新异常，data:{:?},err:{:?}", position, err)).await;
      return Err(err);
    }

    Ok(())
  }

  pub async fn query_position(&self, unit_id: i64, stock_id: i64, redis: &RedisClient, db: &DbConnection) -> Result<Option<PhoenixAstStockposition>> {
    let mut rkey = constdata::USER_POSITION_KEY.to_string();
    rkey = rkey.replacen("{}", unit_id.to_string().as_str(), 1).replacen("{}", stock_id.to_string().as_str(), 1);
    let assets = redis.get_value_by_get(&rkey).await;
    if !assets.is_empty() {
      let retdata: Result<PhoenixAstStockposition, serde_json::Error> = serde_json::from_str(&assets);
      if let Err(_) = retdata {
        self.push_log(format!("get_assets_by_id数据解析失败{}", &assets)).await;
      } else {
        let ret = retdata.unwrap();
        return Ok(Some(ret));
      }
    }

    let ret = self.init(unit_id, stock_id, db, redis).await;
    if let Err(err) = ret {
      return Err(err);
    }
    let retdata = ret.unwrap();
    for item in retdata {
      if item.stock_id == stock_id {
        return Ok(Some(item));
      }
    }
    Ok(None)
  }

  //持仓更新
  pub async fn phenix_postions_change(
    &self,
    req: &PhoenixassetscenterRequest,
    redis: &RedisClient,
    db: &DbConnection,
    uidgen: &UidgenService,
    akaclient: &AkaClient,
    sysinfo: &PhoenixSysSystem,
    txpersist: &tokio::sync::mpsc::Sender<PersistData>,
  ) -> Result<Vec<NotificationPosition>> {
    let current_date_ok = sysinfo.init_date;
    let mut vet_vec = Vec::new();
    let rate_op = akaclient.query_exchange_rate("CNY").await;
    if let Err(err) = rate_op {
      self.push_log(format!("汇率查询失败{:?}", err)).await;
      return Err(err);
    }

    let rate = rate_op.unwrap();
    let positions = req.to_owned().postions;

    for item in positions.iter() {
      let ret = self.postions_change(req, item, db, redis, current_date_ok, rate.buy_rate, rate.sell_rate, uidgen, akaclient, txpersist).await;
      if let Err(err) = ret {
        self.push_log(format!("持仓更新失败{:?},{:?},{:?}", err, item, req)).await;
        let mut lockkey = constdata::LOCK_USER_POSITION_KEY.to_string();
        lockkey = lockkey.replacen("{}", req.unit_id.to_string().as_str(), 1).replacen("{}", item.stock_id.to_string().as_str(), 1);
        let _ = redis.delele(&lockkey);
      } else {
        vet_vec.push(ret.unwrap());
      }
    }

    Ok(vet_vec)
  }

  pub async fn postions_change(
    &self,
    req: &PhoenixassetscenterRequest,
    item: &PhoenixassetspostionrequestInfo,
    db: &DbConnection,
    redis: &RedisClient,
    system_date: i32,
    current_rate_buy: f64,
    current_rate_sell: f64,
    uidgen: &UidgenService,
    akaclient: &AkaClient,
    txpersist: &tokio::sync::mpsc::Sender<PersistData>,
  ) -> Result<NotificationPosition> {
    //加锁
    let mut lockkey = constdata::LOCK_USER_POSITION_KEY.to_string();
    lockkey = lockkey.replacen("{}", req.unit_id.to_string().as_str(), 1).replacen("{}", item.stock_id.to_string().as_str(), 1);
    let mut count = 0;
    //获取锁，超过20次取锁失败，不继续获取锁,锁0.5秒
    while redis.lock(&lockkey, 5).await != 1 && count < 20 {
      thread::sleep(Duration::from_millis(1000));
      count += 1;
    }

    if count > 20 {
      self.push_log(format!("postions_change获取锁超过20次,{:?},{:?}", req, item)).await;
    }

    log::info!("postions_change持仓变更开始处理...{},{:?}", req.unit_id, item);

    let position_result = self.query_position(req.unit_id, item.stock_id, redis, db).await;
    if let Err(err) = position_result {
      self.push_log(format!("查询持仓失败,{},{},{:?}", req.unit_id, item.stock_id, err)).await;
      return Err(anyhow!("持仓查询失败{}", item.stock_id));
    }
    let position_op = position_result.unwrap();

    let stock_op = akaclient.query_stock_info(item.stock_id).await;
    if let Err(err) = stock_op {
      self.push_log(format!("查询持仓品种失败,{}", item.stock_id)).await;
      return Err(anyhow!(constdata::DATA_ERROR));
    }
    let stock = stock_op.unwrap();

    let mut d1 = uidgen.to_owned();
    let position_no = d1.get_uid();

    let margin_rate = Decimal::from_f64_retain(item.margin_rate);
    if margin_rate.is_none() {
      self.push_log(format!("保证金为空,{:?},{:?}", req, item)).await;
      return Err(anyhow!(constdata::DATA_ERROR));
    }

    let mut update_position = PhoenixAstStockposition {
      sys_date: system_date,
      unit_id: req.unit_id,
      stock_code: stock.stock_code,
      stock_id: item.stock_id as i64,
      exchange_id: stock.market_id as i32,
      position_flag: item.position_flag,
      margin_rate: margin_rate.unwrap(),
      stock_type: stock.stock_type,
      position_no: position_no,
      ..Default::default()
    };
    if position_op.is_some() {
      update_position = position_op.unwrap();
    }
    //默认多方向
    if update_position.position_flag == 0 {
      update_position.position_flag = 1;
    }

    let mut current_amount = 0;
    let mut buy_amount = 0;
    let mut sale_amount = 0;
    let mut frozen_amount = 0;
    let mut buy_fee: f64 = 0.0;
    let mut sale_fee: f64 = 0.0;
    let mut current_cost: f64 = 0.0;
    let mut total_value: f64 = 0.0;
    let mut qf_amount = 0;
    let mut total_value_hkd: f64 = 0.0;
    let mut rate = 1.0;
    let mut temp_frozen_amount = 0;
    let mut prebuy_amount = 0;
    let mut presale_amount = 0;
    let mut buy_in_transit = 0;
    let mut sale_in_transit = 0;

    let mut avg_price: f64 = 0.0;
    let mut avg_price_hkd: f64 = 0.0;

    if update_position.current_amount > 0 {
      let t_value_r = update_position.total_value.to_string().parse::<f64>();
      if t_value_r.is_err() {
        return Err(anyhow!(constdata::DATA_ERROR));
      }

      avg_price = t_value_r.unwrap() / (update_position.current_amount as f64);

      let t_value_hkd = update_position.total_value_hkd.to_string().parse::<f64>();
      if t_value_hkd.is_err() {
        return Err(anyhow!(constdata::DATA_ERROR));
      }

      avg_price_hkd = t_value_hkd.unwrap() / (update_position.current_amount as f64);
    }

    if item.op_type == constant::AssetChangeDirection::AddPosition as i32 {
      current_amount = item.deal_amount;
      buy_amount = item.deal_amount;
      buy_fee = item.fee_value;
      current_cost = (item.deal_amount as f64) * item.deal_price + item.fee_value;
      total_value = (item.deal_amount as f64) * item.deal_price;
      if item.qfii_state == 1 {
        qf_amount = item.deal_amount;
      }
      if stock.trade_currency == 1 {
        rate = current_rate_buy;
      }
      total_value_hkd = total_value * rate;

      if stock.market_id == constant::EXCHANGE_HK {
        buy_in_transit = item.deal_amount;
      }
    } else if item.op_type == constant::AssetChangeDirection::ReducePosition as i32 {
      current_amount = 0 - item.deal_amount;
      sale_amount = item.deal_amount;
      sale_fee = item.fee_value;
      current_cost = 0.0 - (item.deal_amount as f64) * item.deal_price + item.fee_value;
      total_value = 0.0 - (item.deal_amount as f64) * avg_price;
      if item.qfii_state == 1 {
        qf_amount = 0 - item.deal_amount;
      }
      if stock.trade_currency == 1 {
        rate = current_rate_sell;
      }
      total_value_hkd = 0.0 - (item.deal_amount as f64) * avg_price_hkd * rate;
      if stock.market_id == constant::EXCHANGE_HK {
        sale_in_transit = item.deal_amount;
      }
    } else {
      frozen_amount = item.frozen_amount;
      prebuy_amount = item.prebuy_amount;
      presale_amount = item.presale_amount;
      temp_frozen_amount = item.temp_frozen_amount;
    }

    update_position.current_amount += current_amount;
    update_position.buy_amount += buy_amount;
    update_position.sale_amount += sale_amount;
    update_position.frozen_amount += frozen_amount;
    update_position.qf_amount += qf_amount;
    let d = Decimal::from_f64_retain(buy_fee);
    if d.is_none() {
      return Err(anyhow!(constdata::DATA_ERROR));
    }
    update_position.buy_fee = d.unwrap();

    let d = Decimal::from_f64_retain(sale_fee);
    if d.is_none() {
      return Err(anyhow!(constdata::DATA_ERROR));
    }
    update_position.sale_fee = d.unwrap();
    let d = Decimal::from_f64_retain(current_cost);
    if d.is_none() {
      return Err(anyhow!(constdata::DATA_ERROR));
    }
    update_position.current_cost += d.unwrap();

    let d = Decimal::from_f64_retain(total_value);
    if d.is_none() {
      return Err(anyhow!(constdata::DATA_ERROR));
    }
    update_position.total_value += d.unwrap();
    let d = Decimal::from_f64_retain(item.deal_price);
    if d.is_none() {
      return Err(anyhow!(constdata::DATA_ERROR));
    }
    update_position.last_price = d.unwrap();
    update_position.temp_frozen_amount = temp_frozen_amount;
    update_position.prebuy_amount = prebuy_amount;
    update_position.presale_amount = presale_amount;
    update_position.buy_in_transit = buy_in_transit;
    update_position.sale_in_transit = sale_in_transit;
    let d = Decimal::from_f64_retain(total_value_hkd);
    if d.is_none() {
      return Err(anyhow!(constdata::DATA_ERROR));
    }
    update_position.total_value_hkd += d.unwrap();
    let p = update_position.clone();

    //若是新建持仓
    if update_position.id == 0 {
      log::info!("PhoenixAstStockposition插入记录{:?}", update_position);
      let ret = PhoenixAstStockposition::insert(&update_position, db).await;
      if let Err(err) = ret {
        self.push_log(format!("持仓插入失败:{:?},err:{:?}", update_position, err)).await;
        return Err(err);
      }
      update_position.id = ret.unwrap()
    }

    //更新redis
    let ret = self.update_position_redis(req.unit_id, item.stock_id, &update_position, redis).await;

    if let Err(err) = ret {
      self.push_log(format!("update_position_redis,redis更新失败：{:?},{:?}", req, err)).await;
      return Err(err);
    }

    //更新数据库
    if p.id > 0 {
      let ret = txpersist.send(PersistData::UserPosition(Box::new(vec![p]))).await;
      if ret.as_ref().is_err() {
        log::error!("push error:{}", ret.as_ref().err().unwrap().to_string());
      }
    }

    let mut currency = String::from("HKD");
    if stock.trade_currency == 1 {
      currency = "CNY".to_string();
    } else {
      currency = "USD".to_string();
    }
    //插入资金流水
    if item.op_type == constant::AssetChangeDirection::AddPosition as i32 || item.op_type == constant::AssetChangeDirection::ReducePosition as i32 {
      let rate_deciaml = Decimal::from_f64_retain(rate);
      let flow = PhoenixOmsAssetflow {
        sys_date: system_date.to_owned(),
        unit_id: req.unit_id,
        busin_flag: req.business_flag,
        occur_capital: Decimal::new(item.deal_amount as i64, 0),
        post_capital: Decimal::new(current_amount as i64, 0),
        datetime: timeutil::current_timestamp(),
        remarks: "".to_string(),
        op_type: item.op_type,
        currency_no: currency.clone(),
        currency_rate: rate_deciaml.unwrap(),
        ..Default::default()
      };

      let operator_id = req.operator_id as i32;
      //创建资金操作表记录
      let option_detail = PhoenixAstOperationDetail {
        sys_date: system_date.to_owned(),
        unit_id: req.unit_id,
        op_businflag: req.business_flag.to_string(),
        remark: "".to_string(),
        currency_no: currency,
        create_time: timeutil::current_timestamp(),
        operator: operator_id,
        op_type: item.op_type,
        occur_capital: Decimal::new(item.deal_amount as i64, 0),
        ..Default::default()
      };
      let data = (flow, option_detail);
      let ret = txpersist.send(PersistData::PhoenixOmsAssetflow(Box::new(data))).await;
      if ret.as_ref().is_err() {
        log::error!("push error:{}", ret.as_ref().err().unwrap().to_string());
      }
    }
    let ret = self.phoenix_query_positions_formatdata(&update_position).await;
    if let Err(err) = ret {
      return Err(err);
    }
    let _ = redis.delele(&lockkey);
    Ok(ret.unwrap())
  }

  pub async fn phoenix_query_positions_formatdata(&self, item: &PhoenixAstStockposition) -> Result<NotificationPosition> {
    let margin_rate = item.margin_rate.to_string().parse::<f64>();
    let total_value = item.total_value.to_string().parse::<f64>();
    let total_value_hkd = item.total_value_hkd.to_string().parse::<f64>();
    if margin_rate.is_err() || total_value.is_err() || total_value_hkd.is_err() {
      return Err(anyhow!(constdata::DATA_ERROR));
    }

    let info = NotificationPosition {
      position_flag: item.position_flag,
      unit_id: item.unit_id,
      position_no: item.position_no,
      stock_code: item.to_owned().stock_code,
      stock_id: item.stock_id,
      exchange_id: item.exchange_id as i64,
      begin_amount: item.begin_amount,
      current_amount: item.current_amount,
      frozen_amount: item.frozen_amount,
      temp_frozen_amount: item.temp_frozen_amount,
      buy_amount: item.buy_amount,
      sale_amount: item.sale_amount,
      prebuy_amount: item.prebuy_amount,
      presale_amount: item.presale_amount,
      buy_in_transit: item.buy_in_transit,
      sale_in_transit: item.sale_in_transit,
      channel_id: 0,
      stock_type: item.stock_type,
      margin_rate: margin_rate.unwrap(),
      total_value: total_value.unwrap(),
      total_value_hkd: total_value_hkd.unwrap(),
      qfii_amount: item.qf_amount,
      timestamp: timeutil::current_timestamp(),
    };
    Ok(info)
  }

  //查询所有持仓
  pub async fn phoenix_query_positions(&self, unit_id: i64, redis: &RedisClient, db: &DbConnection) -> Result<Vec<Phoenixassetspostioninfo>> {
    let mut ret: Vec<Phoenixassetspostioninfo> = Vec::new();

    let vec_posion_ret = PhoenixAstStockposition::query_many(unit_id, 0, db).await;
    if let Err(err) = vec_posion_ret {
      self.push_log(format!("查询用户持仓失败:{:?}", err)).await;
      return Err(err);
    }
    let vec_posion = vec_posion_ret.unwrap();
    for item in vec_posion {
      let margin_rate = item.margin_rate.to_string().parse::<f64>();
      let total_value = item.total_value.to_string().parse::<f64>();
      let total_value_hkd = item.total_value_hkd.to_string().parse::<f64>();
      if margin_rate.is_err() || total_value.is_err() || total_value_hkd.is_err() {
        return Err(anyhow!(constdata::DATA_ERROR));
      }
      let info = Phoenixassetspostioninfo {
        position_flag: item.position_flag,
        unit_id: item.unit_id,
        position_no: item.position_no,
        stock_code: item.stock_code,
        stock_id: item.stock_id,
        exchange_id: item.exchange_id as i64,
        begin_amount: item.begin_amount,
        current_amount: item.current_amount,
        frozen_amount: item.frozen_amount,
        temp_frozen_amount: item.temp_frozen_amount,
        buy_amount: item.buy_amount,
        sale_amount: item.sale_amount,
        prebuy_amount: item.prebuy_amount,
        presale_amount: item.presale_amount,
        buy_in_transit: item.buy_in_transit,
        sale_in_transit: item.sale_in_transit,
        channel_id: 0,
        stock_type: item.stock_type,
        margin_rate: margin_rate.unwrap(),
        total_value: total_value.unwrap(),
        total_value_hkd: total_value_hkd.unwrap(),
        qfii_amount: item.qf_amount,
      };
      ret.push(info);
    }
    return Result::Ok(ret);
  }

  //更新持仓数据到db
  pub async fn update_positions_dbdata(&self, data: &Vec<PhoenixAstStockposition>, db: &DbConnection) -> Result<()> {
    for item in data.iter() {
      let ret = PhoenixAstStockposition::save(item, db).await;
      if let Err(err) = ret {
        self.push_log(format!("持仓更新到数据库失败,data:{:?},err:{:?}", item, err)).await;
        return Err(err);
      }
    }
    Ok(())
  }

  //更新持仓流水数据
  pub async fn save_positions_flow_data(&self, data: (PhoenixOmsAssetflow, PhoenixAstOperationDetail), db: &DbConnection) -> Result<()> {
    let insert_ret = PhoenixOmsAssetflow::insert(&data.0, db).await;
    if let Err(err) = insert_ret {
      return Err(err);
    }
    let mut option_detail = data.1;
    option_detail.ext_id = insert_ret.unwrap();
    let insert_detal = PhoenixAstOperationDetail::insert(&option_detail, db).await;
    if let Err(err) = insert_detal {
      return Err(err);
    }
    Ok(())
  }
}
