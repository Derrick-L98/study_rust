use messagecenter::protofiles::phoenixnotification::NotificationAsset;
use serde::{Deserialize, Serialize};

use crate::protofiles::assetscenter::PhoenixassetsResultInfo;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserAssetsData {
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
  pub credit_multiple: f64, //
  //持仓市值
  pub position_value: f64, //所有持仓汇总
  //创业板市值
  pub position_value_cyb: f64, //创业板汇总
  //保证金占用
  pub margin_use: f64, //∑(持仓保证金比率*市值)
  //持仓盈亏
  pub hold_yk: f64, //∑持仓盈亏((最新价-均价)*数量)
  //交易状态
  pub trade_flag: i32,
  //预警线
  pub warn_line: f64,
  //平仓线
  pub close_line: f64,
  //最后更新时间
  pub last_time: i64,
  //风险率
  pub risk_rate: f64, //保证金占用/当前本金
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

impl UserAssetsData {
  pub fn convert_assetsinfo_to_assetsdata(assets: &PhoenixassetsResultInfo, lasttime: i64) -> UserAssetsData {
    UserAssetsData {
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

  pub fn convert_notificationasset_to_assetsdata(assets: &NotificationAsset) -> UserAssetsData {
    UserAssetsData {
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
}
