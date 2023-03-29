use common::akaclient::AkaClient;
use messagecenter::protofiles::phoenixnotification::NotificationPosition;
use serde::{Deserialize, Serialize};
use utility::timeutil::current_timestamp;

use crate::protofiles::assetscenter::Phoenixassetspostioninfo;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserPositionData {
  pub position_no: i64,
  /// 用户id
  pub unit_id: i64,
  /// 证券代码
  pub stock_code: String,
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
  pub currency_rate: f64, //汇率
}

impl UserPositionData {
  pub async fn convert_positioninfo_to_userposition(i: &Phoenixassetspostioninfo, akasvc: &AkaClient) -> UserPositionData {
    let mut buy_rate = 1.0;
    let rate = akasvc.query_exchange_rate(&akasvc.query_currency_by_exchangeid(i.exchange_id)).await;
    if rate.as_ref().is_ok() {
      buy_rate = rate.unwrap().buy_rate;
    }
    let position_data = UserPositionData {
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
      last_time: current_timestamp(),
      last_price: 0.0,
      currency_rate: buy_rate,
    };
    return position_data;
  }

  //持仓信息转换
  pub async fn convert_notificationposition_to_userpositioninfo(i: &NotificationPosition, akasvc: &AkaClient) -> UserPositionData {
    let mut buy_rate = 1.0;
    let rate = akasvc.query_exchange_rate(&akasvc.query_currency_by_exchangeid(i.exchange_id)).await;
    if rate.as_ref().is_ok() {
      buy_rate = rate.unwrap().buy_rate;
    }
    let position_data = UserPositionData {
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
      currency_rate: buy_rate,
    };
    return position_data;
  }
}
