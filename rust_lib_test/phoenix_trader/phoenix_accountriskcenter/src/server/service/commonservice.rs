use std::cmp::min;

use crate::{dataservice::entities::prelude::*, dataview::userassets::UserAssetsData, protofiles::phoenixaccountriskcenter::PhoenixAccountAssetsInfo};
use common::constant;
use messagecenter::protofiles::phoenixnotification::NotificationOrderExec;
use rust_decimal::{
  prelude::{FromPrimitive, ToPrimitive},
  Decimal,
};
use utility::timeutil::{self, current_date};

pub struct CommonService;

impl CommonService {
  pub fn phoenix_risk_details_new(f: &UserAssetsData) -> PhoenixRiskDetails {
    let details = PhoenixRiskDetails {
      sys_date: current_date(),
      user_id: f.unit_id,
      current_cash: Decimal::from_f64_retain(f.current_cash).unwrap_or_default(),
      credit_multiple: Decimal::from_f64_retain(f.credit_multiple).unwrap_or_default(),
      warn_line: Decimal::from_f64_retain(f.warn_line).unwrap_or_default(),
      close_line: Decimal::from_f64_retain(f.close_line).unwrap_or_default(),
      risk_rate: Decimal::from(0),
      loan_cash: Decimal::from_f64_retain(f.position_value - f.current_cash).unwrap_or_default(),
      total_value: Decimal::from_f64_retain(f.position_value).unwrap_or_default(),
      credit_cash: Decimal::new(0, 0),
      real_cash: Decimal::new(0, 0),
      create_datetime: timeutil::current_timestamp(),
      risk_type: 2,
      total_asset_value: Decimal::new(0, 0),
      id: 0,
    };
    details
  }

  pub fn convert_dealinfo_to_dealdetail(dealinfo: &NotificationOrderExec, leverage: Decimal, accountid: i64, currency_rate: Decimal, currency_rate_cj: Decimal, stocktype: i32, qfii: i32) -> PhoenixDealDetail {
    PhoenixDealDetail {
      id: 0,
      p_date: timeutil::current_date(),
      p_order_no: dealinfo.order_id,
      p_exchange_id: 0,
      p_stock_id: 0,
      p_stock_code: "".to_string(),
      p_order_direction: dealinfo.order_direction,
      p_deal_amount: dealinfo.order_quantity,
      p_prebuy_amount: 0,
      p_deal_price: Decimal::from_f64(dealinfo.order_price).unwrap_or_default(),
      p_fee_total: Decimal::from(0),
      p_currency_rate: currency_rate,
      p_currency_rate_cj: currency_rate_cj,
      p_unit_id: dealinfo.unit_id,
      p_channel_type: dealinfo.channel_type,
      p_channel_id: dealinfo.channel_id as i64,
      p_account_id: accountid,
      p_qfii_state: qfii,
      p_trade_type: constant::TradeType::Normal as i32,
      p_stock_type: stocktype,
      p_leverage: leverage,
      p_updatetime: timeutil::current_timestamp(),
    }
  }

  pub fn convert_dealdetailhis_to_dealdetail(val: &PhoenixDealDetailHis) -> PhoenixDealDetail {
    PhoenixDealDetail {
      id: val.id,
      p_date: val.p_date,
      p_order_no: val.p_order_no,
      p_exchange_id: val.p_exchange_id,
      p_stock_id: val.p_stock_id,
      p_stock_code: val.p_stock_code.to_owned(),
      p_order_direction: val.p_order_direction.to_owned(),
      p_deal_amount: val.p_deal_amount,
      p_deal_price: val.p_deal_price,
      p_fee_total: val.p_fee_total,
      p_leverage: val.p_leverage,
      p_currency_rate: val.p_currency_rate,
      p_currency_rate_cj: val.p_currency_rate_cj,
      p_unit_id: val.p_unit_id,
      p_channel_type: val.p_channel_type,
      p_channel_id: val.p_channel_id,
      p_account_id: val.p_account_id,
      p_qfii_state: val.p_qfii_state,
      p_trade_type: val.p_trade_type,
      p_prebuy_amount: 0,
      p_stock_type: val.p_stock_type,
      p_updatetime: val.p_updatetime,
    }
  }
  pub fn convert_stockpositions_to_stockpositionhis(val: &PhoenixStockPositionChannel, cur_date: i32) -> PhoenixStockPositionChannelHis {
    PhoenixStockPositionChannelHis {
      id: 0,            //bigint(12) NO
      p_date: cur_date, //decimal(8,0)
      p_account_unit_id: val.p_account_unit_id,
      p_stock_id: val.p_stock_id,
      p_stock_code: val.p_stock_code.clone(),
      p_exchange_id: val.p_exchange_id,
      p_prebuy_amount: val.p_prebuy_amount,
      p_current_amount: val.p_current_amount,
      p_avg_price: val.p_avg_price,
      p_avg_price_hkd: val.p_avg_price_hkd,
      p_total_value: val.p_total_value,
      p_total_value_hkd: val.p_total_value_hkd,
      p_frozen_amount: val.p_frozen_amount,
      p_frozen_amount_temp: val.p_frozen_amount_temp,
      p_last_price: val.p_last_price,
      p_currency_rate: val.p_currency_rate,
      p_fee_hkd: val.p_fee_hkd,
      p_stock_type: val.p_stock_type,
      p_channel_id: val.p_channel_id,
      p_qfii_state: val.p_qfii_state,
      p_leverage: val.p_leverage,
    }
  }

  pub fn convert_accountassets_to_accountassetshis(val: &PhoenixAccountAssets, dealdate: i32) -> PhoenixAccountAssetsHis {
    let mut deal_date = dealdate;
    if deal_date == 0 {
      deal_date = val.p_date;
    }

    PhoenixAccountAssetsHis {
      id: 0,
      p_account_id: val.p_account_id,
      p_current_principal: val.p_current_principal,
      p_credit_cash: val.p_credit_cash,
      p_financing_borrowed: val.p_financing_borrowed,
      p_financing_occupied: val.p_financing_occupied,
      p_position_value: val.p_position_value,
      p_position_value_star: val.p_position_value_star,
      p_real_profit: val.p_real_profit,
      p_floating_profit: val.p_floating_profit,
      p_fee_total_hkd: val.p_fee_total_hkd,
      p_prebuy_capital_star: val.p_prebuy_capital_star,
      p_account_type: val.p_account_type,
      p_updatetime: val.p_updatetime,
      p_date: deal_date,
      p_current_financial: val.p_current_financial,
    }
  }

  pub fn convert_accountassets_to_assetsinfo(info: &PhoenixAccountAssets) -> PhoenixAccountAssetsInfo {
    if info.p_account_type == constant::AccountType::MainAccount as i32 {
      return PhoenixAccountAssetsInfo {
        p_current_principal: info.p_current_principal.to_f64().unwrap_or_default(),
        p_account_id: info.p_account_id,
        p_account_type: info.p_account_type,
        p_lastdate: info.p_date,
        p_updatetime: info.p_updatetime,

        // p_credit_cash: info.p_credit_cash.to_f64().unwrap_or_default(),
        // p_current_financial: info.p_current_financial.to_f64().unwrap_or_default(),
        // p_financing_borrowed: info.p_financing_borrowed.to_f64().unwrap_or_default(),
        // p_financing_occupied: info.p_financing_occupied.to_f64().unwrap_or_default(),
        // p_floating_profit: info.p_floating_profit.to_f64().unwrap_or_default(),
        // p_position_value: info.p_position_value.to_f64().unwrap_or_default(),
        // p_position_value_star: info.p_position_value_star.to_f64().unwrap_or_default(),
        ..Default::default()
      };
    }

    //创业板比例 = [（所属通道持仓市值（创）汇总 + 所属通道对应创业板开仓挂单金额汇总）/ 当前本金 *（杠杆 + 1）] * 100%
    let current_principal: Decimal; //当前本金
    let mut rich_security = Decimal::from(0);

    if info.p_account_type == constant::AccountType::ChannelAccount as i32 {
      //分帐户
      current_principal = info.p_current_principal + info.p_real_profit + info.p_floating_profit;
      //运营子账户运营本金+min(0,浮动盈亏）- 保证金占用
      rich_security = info.p_current_principal + info.p_real_profit + min(Decimal::from(0), info.p_floating_profit) - info.p_financing_occupied;
    } else if info.p_account_type == constant::AccountType::TotalAccount as i32 {
      current_principal = info.p_current_principal;
    } else {
      return PhoenixAccountAssetsInfo {
        p_current_principal: info.p_current_principal.to_f64().unwrap(),
        ..Default::default()
      };
    }
    let mut star_rate: Decimal = Decimal::from(0);
    if current_principal != Decimal::from(0) {
      star_rate = (info.p_position_value_star) / (current_principal * Decimal::from(constant::LEVER + 1));
    }
    //风险率 = [ 已借金额 /（持仓市值*0.75）] * 100%
    let mut risk_rate = Decimal::from(0);
    if info.p_position_value != Decimal::from(0) {
      risk_rate = info.p_financing_borrowed / (info.p_position_value * Decimal::from_f32(constant::RISK_COEFFICIENT).unwrap());
    }
    PhoenixAccountAssetsInfo {
      id: info.id,
      p_account_id: info.p_account_id,
      p_current_principal: (info.p_current_principal + info.p_real_profit).to_f64().unwrap_or_default(),
      p_credit_cash: info.p_credit_cash.to_f64().unwrap_or_default(),
      p_current_financial: info.p_current_financial.to_f64().unwrap_or_default(),
      p_position_value: info.p_position_value.to_f64().unwrap_or_default(),
      p_position_value_star: info.p_position_value_star.to_f64().unwrap_or_default(),
      p_prebuy_capital_star: info.p_prebuy_capital_star.to_f64().unwrap_or_default(),
      p_floating_profit: info.p_floating_profit.to_f64().unwrap_or_default(),
      p_account_type: info.p_account_type,
      p_lastdate: info.p_date,
      p_updatetime: info.p_updatetime,
      p_financing_borrowed: info.p_financing_borrowed.to_f64().unwrap_or_default(),
      p_financing_occupied: info.p_financing_occupied.to_f64().unwrap_or_default(),
      p_rich_security: rich_security.to_f64().unwrap_or_default(),
      p_risk_value: risk_rate.to_f64().unwrap_or_default(),
      p_star_rate: star_rate.to_f64().unwrap_or_default(),
      p_real_profit: info.p_real_profit.to_f64().unwrap_or_default(),
      p_fee_total_hkd: info.p_fee_total_hkd.to_f64().unwrap_or_default(),
    }
  }

  pub fn convert_accountassetshis_to_accountassets(val: &PhoenixAccountAssetsHis) -> PhoenixAccountAssets {
    PhoenixAccountAssets {
      id: 0,
      p_account_id: val.p_account_id,
      p_credit_cash: val.p_credit_cash,
      p_current_principal: val.p_current_principal,
      p_financing_borrowed: val.p_financing_borrowed,
      p_financing_occupied: val.p_financing_occupied,
      p_position_value: val.p_position_value,
      p_position_value_star: val.p_position_value_star,
      p_real_profit: val.p_real_profit,
      p_floating_profit: val.p_floating_profit,
      p_prebuy_capital_star: val.p_prebuy_capital_star,
      p_fee_total_hkd: val.p_fee_total_hkd,
      p_account_type: val.p_account_type,
      p_date: val.p_date,
      // p_curdate: current_date(),
      p_updatetime: val.p_updatetime,
      p_current_financial: val.p_current_financial,
      // p_updatetime: val.,
    }
  }

  pub fn convert_accountassetshis_to_assetsinfo(info: &PhoenixAccountAssetsHis) -> PhoenixAccountAssetsInfo {
    let assets_info = CommonService::convert_accountassetshis_to_accountassets(info);
    CommonService::convert_accountassets_to_assetsinfo(&assets_info)
  }
}
