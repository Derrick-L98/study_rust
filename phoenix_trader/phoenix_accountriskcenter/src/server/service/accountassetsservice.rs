// use crate::config::settings::Settings;
use crate::dataservice::{dbsetup::DbConnection, entities::prelude::*};
use anyhow::{anyhow, Result};
use common::constant;
use dashmap::DashMap;
use rust_decimal::Decimal;
use utility::timeutil::{self, current_timestamp};
#[derive(Debug, Clone)]
pub struct PhoenixAccountAssetsService {
  account_assets_cache: DashMap<i64, PhoenixAccountAssets>,
}

impl PhoenixAccountAssetsService {
  pub fn new() -> Self {
    PhoenixAccountAssetsService { account_assets_cache: DashMap::new() }
  }

  pub async fn init(&self, db: &DbConnection) -> Result<()> {
    self.account_assets_cache.clear();
    if let Ok(ret) = PhoenixAccountAssets::query_many(constant::VALUE_ALL, &db).await {
      ret.into_iter().for_each(|val| {
        self.account_assets_cache.insert(val.p_account_id, val);
      });

      return Ok(());
    }
    Err(anyhow!("初始化数据失败"))
  }

  pub fn get_account_assets(&self, account_id: i64) -> Vec<PhoenixAccountAssets> {
    let ret: Vec<PhoenixAccountAssets> = self
      .account_assets_cache
      .iter()
      .filter(|val| if account_id > 0 { val.p_account_id == account_id } else { val.p_account_id > 0 })
      .map(|v| v.clone())
      .collect();

    ret
  }

  pub async fn update_account_assets_by_dealinfo(&self, dealinfo: &PhoenixDealDetail, account_pos: &Vec<PhoenixStockPositionChannel>, tradectl: &PhoenixSysSystem) -> Result<PhoenixAccountAssets> {
    let mut avg_price_hkd = dealinfo.p_deal_price * dealinfo.p_currency_rate;
    if let Some(pos) = account_pos
      .iter()
      .find(|x| x.p_stock_id == dealinfo.p_stock_id && x.p_channel_id == dealinfo.p_channel_id as i64 && x.p_exchange_id == dealinfo.p_exchange_id)
    {
      avg_price_hkd = pos.p_avg_price_hkd;
    } else {
      log::error!("找不到品种:{},市场iD:{},通道id:{}的持仓信息,用成交价做均价", dealinfo.p_stock_id, dealinfo.p_exchange_id, dealinfo.p_channel_id);
    }
    let mut real_profit = Decimal::from(0);
    if dealinfo.p_order_direction == constant::OrderDirection::SELL as i32 {
      // 卖单有实际盈亏
      real_profit = Decimal::from(dealinfo.p_deal_amount) * (dealinfo.p_deal_price * dealinfo.p_currency_rate_cj - avg_price_hkd);
      // log::info!(
      //     "卖单的实际盈亏,成交数量:{}* (成交价:{}*成交汇率:{} - 持仓均价HKD:{}), 结果:{}",
      //     dealinfo.p_deal_amount,
      //     dealinfo.p_deal_price,
      //     dealinfo.p_currency_rate_cj,
      //     avg_price_hkd,
      //     real_profit
      // );
    }
    let fee_total = dealinfo.p_currency_rate_cj * dealinfo.p_fee_total;
    // log::info!("本次交易的费用：{}，汇率:{},总费用:{}", dealinfo.p_fee_total, dealinfo.p_currency_rate_cj, fee_total);

    let account_assets_info = self.update_account_assets(dealinfo.p_account_id, real_profit, fee_total, account_pos, tradectl);
    if account_assets_info.is_ok() {
      Ok(account_assets_info.unwrap())
    } else {
      Err(anyhow!("update account assets error"))
    }
    // Ok(())
  }

  pub async fn summarize_account_assets_from_account_positions(&self, accountid: i64, positions: &Vec<PhoenixStockPositionChannel>, curdate: i32) {
    log::info!("更新前的分帐户资金数据:{:#?}", &self.account_assets_cache);
    if accountid != constant::VALUE_ALL as i64 {
      let pos_asset = PhoenixAccountAssetsService::compute_account_assets_from_positions(accountid, positions);
      self
        .account_assets_cache
        .entry(accountid)
        .and_modify(|ret| {
          ret.p_financing_borrowed = pos_asset.p_financing_borrowed;
          ret.p_financing_occupied = pos_asset.p_financing_occupied;
          ret.p_position_value = pos_asset.p_position_value;
          ret.p_position_value_star = pos_asset.p_position_value_star;
          ret.p_prebuy_capital_star = pos_asset.p_prebuy_capital_star;
          ret.p_floating_profit = pos_asset.p_floating_profit;
          ret.p_date = curdate;
          ret.p_updatetime = timeutil::current_timestamp();
        })
        .or_insert(pos_asset); //找不到则插入新数据
    } else {
      self.account_assets_cache.iter_mut().for_each(|mut ret| {
        let pos_asset = PhoenixAccountAssetsService::compute_account_assets_from_positions(ret.key().clone(), positions);
        ret.p_financing_borrowed = pos_asset.p_financing_borrowed;
        ret.p_financing_occupied = pos_asset.p_financing_occupied;
        ret.p_position_value = pos_asset.p_position_value;
        ret.p_position_value_star = pos_asset.p_position_value_star;
        ret.p_prebuy_capital_star = pos_asset.p_prebuy_capital_star;
        ret.p_floating_profit = pos_asset.p_floating_profit;
        ret.p_date = curdate;
        ret.p_updatetime = timeutil::current_timestamp();
      });
    }
    log::info!("更新后的分帐户资金数据: {:#?}", &self.account_assets_cache);
  }

  pub async fn reset_profit(&self, accountid: i64, f_borrowd: Decimal) -> (Decimal, Decimal, Decimal) {
    log::info!("account assets before reset:{:#?}", &self.account_assets_cache);
    let mut ret = (Decimal::from(0), Decimal::from(0), Decimal::from(0));
    self.account_assets_cache.entry(accountid).and_modify(|f| {
      ret.0 = f.p_real_profit;
      ret.1 = f.p_floating_profit;
      ret.2 = f.p_current_financial;
      f.p_real_profit += f.p_floating_profit;
      f.p_floating_profit = Decimal::from(0);
      f.p_financing_borrowed = f_borrowd;
      // ret.2 = f.p_real_profit;
      // ret.3 = f.p_floating_profit;
    });
    log::info!("account assets after reset:{:#?}", &self.account_assets_cache);

    ret
  }

  // pub async fn update_total_account_assets(&self, val: &PhoenixAccountAssets) {
  //   self
  //     .account_assets_cache
  //     .entry(val.p_account_id)
  //     .and_modify(|f| {
  //       // log::info!("update existed total account value");
  //       f.p_current_principal = val.p_current_principal;
  //       f.p_credit_cash = val.p_credit_cash;
  //       f.p_current_financial = val.p_current_financial;
  //       f.p_position_value = val.p_position_value;
  //       f.p_position_value_star = val.p_position_value_star;
  //       f.p_prebuy_capital_star = val.p_prebuy_capital_star;
  //       f.p_financing_borrowed = val.p_financing_borrowed;
  //       f.p_floating_profit = val.p_floating_profit;
  //       f.p_updatetime = val.p_updatetime;
  //       // f.p_curdate = val.p_curdate;
  //       // f.p_lastdate = val.p_lastdate;
  //     })
  //     .or_insert_with(|| val.to_owned());
  // }

  fn update_account_assets(&self, account_id: i64, real_profit: Decimal, fee_total: Decimal, account_pos: &Vec<PhoenixStockPositionChannel>, tradectl: &PhoenixSysSystem) -> Result<PhoenixAccountAssets> {
    // log::info!("更新分帐户的资产数据,实际盈亏:{},费用:{}", real_profit, fee_total);
    let mut ret = self
      .account_assets_cache
      .entry(account_id)
      .and_modify(|f| {
        f.p_real_profit += real_profit;
        f.p_fee_total_hkd += fee_total;
      })
      .or_insert_with(|| PhoenixAccountAssets {
        p_account_id: account_id,
        p_real_profit: real_profit,
        p_fee_total_hkd: fee_total,
        p_date: tradectl.init_date,
        ..Default::default()
      });
    // log::info!("update real profit result:{:#?}", &ret);
    //计算持仓相关数据
    let pos_asset = PhoenixAccountAssetsService::compute_account_assets_from_positions(account_id, account_pos);
    // log::info!("account id: {}, result: {:#?}", account_id, &pos_asset);
    ret.p_financing_borrowed = pos_asset.p_financing_borrowed;
    ret.p_financing_occupied = pos_asset.p_financing_occupied;
    ret.p_position_value = pos_asset.p_position_value;
    ret.p_position_value_star = pos_asset.p_position_value_star;
    ret.p_prebuy_capital_star = pos_asset.p_prebuy_capital_star;
    ret.p_floating_profit = pos_asset.p_floating_profit;
    // ret.p_curdate = current_date();
    ret.p_updatetime = current_timestamp();
    // log::info!("更新后的资产信息:{:#?}", &ret);
    Ok(ret.clone())
  }

  fn compute_account_assets_from_positions(account_id: i64, positions: &Vec<PhoenixStockPositionChannel>) -> PhoenixAccountAssets {
    let mut account_asset = PhoenixAccountAssets {
      p_account_id: account_id,
      ..Default::default()
    };

    let mut financing_borrowed: Decimal = Decimal::new(0, 8); // = acntval.p_financing_borrowed; //= Decimal::new(0, 4);
    let mut position_value: Decimal = Decimal::new(0, 8); //= acntval.p_financing_position; //Decimal::new(0, 4);
    let mut position_value_star: Decimal = Decimal::new(0, 8); // = acntval.p_financing_position_star; //Decimal::new(0, 4);
    let mut floating_profit: Decimal = Decimal::new(0, 8); // = acntval.p_floating_profit; //Decimal::new(0, 4); //= acntval.p_floating_profit;
    let mut star_prebuy_capital: Decimal = Decimal::new(0, 8); //
    let mut financing_occupied = Decimal::new(0, 8);
    let mut total_value: Decimal = Decimal::new(0, 8);
    // let mut fee_total = Decimal::new(0, 8);
    let mut f_borrowed; // = Decimal::new(0, 4);
                        // log::info!("account assets id: {}", account_id);
    for pos in positions {
      if pos.p_account_unit_id != account_id {
        continue;
      }

      if pos.p_current_amount == 0 {
        continue;
      }
      let current_position_value = pos.p_last_price * pos.p_currency_rate * Decimal::from(pos.p_current_amount);
      //持仓成本
      total_value += pos.p_total_value_hkd;
      //∑保证金占用=品种1对应通道的持仓市值*[品种1在该通道的保证金比例]+...品种N对应通道持仓市值*[品种N在该通道的保证金比例]
      let f_ocupied = current_position_value * pos.p_leverage;

      financing_occupied += f_ocupied;
      // log::info!("计算已借的比率(1 - (1 / 杠杆:{})),结果:{}", pos.p_leverage, ratio);
      f_borrowed = pos.p_total_value_hkd * (Decimal::from(1) - pos.p_leverage);

      financing_borrowed += f_borrowed;
      //浮动盈亏,∑((最新价-持仓均价)*数量)
      let f_floating = (pos.p_last_price * pos.p_currency_rate - pos.p_avg_price_hkd) * Decimal::from(pos.p_current_amount);

      floating_profit += f_floating;
      //(pos.p_last_price - pos.p_avg_price) * Decimal::from(pos.p_current_amount) * currentcy_rate;
      //持仓市值,包含科创版和创业板, 全部帐户: 分帐户持仓表中∑(current_amount*last_price)*参考汇率
      position_value += current_position_value;
      if pos.p_stock_type == (constant::StockType::HSCY as i32) || pos.p_stock_type == (constant::StockType::HSKC as i32) {
        //(科创版+创业板)持仓市值,单独计算,全部账户计算方式一样
        position_value_star += current_position_value;
        let f_star_pos = Decimal::from(pos.p_prebuy_amount) * pos.p_last_price * pos.p_currency_rate;

        star_prebuy_capital += f_star_pos;
      }
    }

    account_asset.p_financing_borrowed = financing_borrowed;
    account_asset.p_position_value = position_value;
    account_asset.p_position_value_star = position_value_star;
    account_asset.p_floating_profit = floating_profit;
    account_asset.p_prebuy_capital_star = star_prebuy_capital;
    account_asset.p_financing_occupied = financing_occupied;
    // account_asset.p_fee_total_hkd = fee_total;
    account_asset
  }

  pub async fn transfer_fund(&self, transdetail: &PhoenixTransDetail) -> Result<()> {
    log::info!("start to transfer fund: {:#?}", &transdetail);
    let target_account = self.account_assets_cache.get(&transdetail.p_account_target);
    if target_account.is_none() {
      return Err(anyhow!("target account doesn't exist"));
    }

    // let target_account = target_account.unwrap();

    let source_account = self.account_assets_cache.get(&transdetail.p_account_source);
    if source_account.is_some() && source_account.unwrap().p_account_type == target_account.unwrap().p_account_type {
      return Err(anyhow!("source account type is same as target account"));
    }

    let mut trans_value = transdetail.p_trans_value;
    if transdetail.p_op_flag == constant::TransDirection::DEC as i32 {
      trans_value = -trans_value;
    }
    log::info!("current trans value is: {}", trans_value);

    self.account_assets_cache.iter_mut().for_each(|mut val| {
      let key_val = val.key().to_owned();
      if key_val == transdetail.p_account_target {
        val.p_current_principal += trans_value;
      }
      if key_val == transdetail.p_account_source {
        val.p_current_financial -= trans_value;
      }
    });
    Ok(())
  }
}
