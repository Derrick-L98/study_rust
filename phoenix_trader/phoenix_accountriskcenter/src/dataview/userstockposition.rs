use crate::dataservice::entities::prelude::*;
use rust_decimal::Decimal;

#[derive(Default, Debug, Clone)]
pub struct UserStockPosistion {
  pub l_unit_id: i64,              //交易账号
  pub l_date: i32,                 //交易时间
  pub l_stock_id: i64,             //股票ID
  pub vc_stock_code: String,       //股票代码
  pub l_exchange_id: i64,          //市场编号,
  pub l_prebuy_amount: i32,        //预买数量
  pub l_current_amount: i32,       //当前数量
  pub l_frozen_amount: i32,        //冻结数量
  pub l_frozen_amount_temp: i32,   //临时数量
  pub en_avg_price: Decimal,       //均价,
  pub en_avg_price_hkd: Decimal,   //均价,通过total_value_hkd/l_current_amount计算取得
  pub en_total_value: Decimal,     //总成本
  pub en_total_value_hkd: Decimal, //总成本-hkd
  pub en_last_price: Decimal,      //最新价
  pub en_currency_rate: Decimal,   //参考汇率 -
  pub en_security_value: Decimal,  //该持仓的保证金占用, - 港币
  pub l_leverage: Decimal,         //该品种在该通道下的杠杆(成交时)
  pub l_channel_id: i64,           //通道id,
  pub p_qfii_state: i32,           //qfii 1:是 2：不是
  pub l_stock_type: i32,           //股票类型
  pub l_channel_unit_id: i64,      //通道的账号ID
}

impl UserStockPosistion {
  pub fn convert_from_deal(sp: &PhoenixDealDetail) -> Self {
    // let en_last_price_hkd = sp.p_deal_price * sp.p_currency_rate;
    let total_value_hkd = sp.p_deal_price * Decimal::from(sp.p_deal_amount) * sp.p_currency_rate_cj;
    //计算每个品种在不同通道的保证金
    // let security_val = (en_last_price_hkd * Decimal::from(sp.l_current_amount)) * (Decimal::from(1) / (Decimal::from(1) + leverage));
    let security_val = total_value_hkd * (Decimal::from(1) - sp.p_leverage);

    let avg_price_hkd: Decimal;
    if sp.p_deal_amount == 0 {
      avg_price_hkd = Decimal::from(0);
    } else {
      avg_price_hkd = total_value_hkd / Decimal::from(sp.p_deal_amount);
    }

    UserStockPosistion {
      l_unit_id: sp.p_unit_id,
      l_date: sp.p_date as i32,
      l_stock_id: sp.p_stock_id,
      vc_stock_code: sp.p_stock_code.clone(),
      l_exchange_id: sp.p_exchange_id,
      l_current_amount: sp.p_deal_amount,
      l_prebuy_amount: sp.p_prebuy_amount, //sp.l_prebuy_amount,
      l_frozen_amount: 0,
      l_frozen_amount_temp: 0,
      en_avg_price: sp.p_deal_price,
      en_avg_price_hkd: avg_price_hkd,
      en_currency_rate: sp.p_currency_rate,
      en_total_value: sp.p_deal_price * Decimal::from(sp.p_deal_amount),
      en_total_value_hkd: total_value_hkd,
      en_last_price: sp.p_deal_price,
      l_channel_id: sp.p_channel_id,
      p_qfii_state: sp.p_qfii_state,
      l_stock_type: sp.p_stock_type, //stock type??????
      l_channel_unit_id: sp.p_account_id,
      en_security_value: security_val,
      l_leverage: sp.p_leverage,
      // p_date_archive: timeutil::current_date(),
    }
  }
}
