use std::{default, ops::Mul};

use crate::{
  dataservice::{dbsetup::DbConnection, entities::prelude::*},
  dataview::userstockposition::UserStockPosistion,
  server::service::commonservice::CommonService,
};
use anyhow::{anyhow, Result};
use common::constant::{self};
use rust_decimal::Decimal;
// use std::collections::HashMap;
use dashmap::DashMap;

//注意:存在多个通道对应一个账户的情况,所以做分帐户持仓时,需要按照账户来处理
//key:股票ID_通道ID
#[derive(Clone)]
pub struct PhoenixAccountStockPositionService {
  account_position_cache: DashMap<String, PhoenixStockPositionChannel>,
}

impl PhoenixAccountStockPositionService {
  pub fn new() -> Self {
    PhoenixAccountStockPositionService { account_position_cache: DashMap::new() }
  }

  pub async fn init(&self, db: &DbConnection) -> Result<()> {
    self.account_position_cache.clear();

    let ret = PhoenixStockPositionChannel::query_many(constant::VALUE_ALL, constant::VALUE_ALL, constant::VALUE_ALL, &db).await;
    if ret.as_ref().is_err() {
      log::error!("初始化分帐户持仓数据错误:{}", ret.as_ref().err().unwrap().to_string());
      return Err(anyhow!("初始化分帐户持仓数据错误:{}", ret.as_ref().err().unwrap().to_string()));
    }

    ret.unwrap().into_iter().for_each(|val| {
      self.account_position_cache.insert(format!("{}-{}", val.p_stock_id, val.p_channel_id), val.clone());
    });

    Ok(())
  }

  pub fn get_account_stock_positions(&self, account_id: i64) -> Vec<PhoenixStockPositionChannel> {
    let ret: Vec<PhoenixStockPositionChannel> = self
      .account_position_cache
      .iter()
      .filter(|x| {
        if account_id == constant::VALUE_ALL {
          x.value().p_account_unit_id > 0
        } else {
          x.value().p_account_unit_id == account_id
        }
      })
      .map(|x| x.clone())
      .collect();

    ret
  }

  pub async fn update_account_positions_last_price(&self, lastprice: &Decimal, stockid: i64) -> Result<()> {
    //循环遍历,更新最新价
    let start_key = format!("{}_", stockid);

    self.account_position_cache.iter_mut().for_each(|mut val| {
      if val.key().starts_with(start_key.as_str()) {
        val.p_last_price = lastprice.clone();
      }
    });

    Ok(())
  }

  pub async fn update_account_positions_frozen_amount(&self, dealinfo: &PhoenixDealDetail, frozen_amount: i32, temp_frozen_amount: i32) -> Result<PhoenixStockPositionChannel> {
    //update prebuy amount
    let key = format!("{}-{}", dealinfo.p_stock_id, dealinfo.p_channel_id);

    self.account_position_cache.entry(key.clone()).and_modify(|val| {
      log::info!("更新冻结量，冻结数量:{}, 临时冻结数量:{}", dealinfo.p_deal_amount, temp_frozen_amount);
      val.p_frozen_amount += frozen_amount;
      val.p_frozen_amount_temp += temp_frozen_amount;
    });

    let retval = self.account_position_cache.get(&key).unwrap().clone();
    Ok(retval)
  }
  pub async fn update_account_positions_prebuy_amount(&self, dealinfo: &PhoenixDealDetail, tradectl: &PhoenixSysSystem, dbconn: &DbConnection) -> Result<(PhoenixStockPositionChannel, bool)> {
    //update prebuy amount
    let key = format!("{}-{}", dealinfo.p_stock_id, dealinfo.p_channel_id);
    let mut is_new = false;
    self
      .account_position_cache
      .entry(key.clone())
      .and_modify(|val| {
        log::info!("更新{}预买量，原预买量:{},新增预买量:{}", &key, val.p_prebuy_amount, dealinfo.p_prebuy_amount);
        val.p_prebuy_amount += dealinfo.p_prebuy_amount;
        if val.p_prebuy_amount <= 0 {
          val.p_prebuy_amount = 0;
        }
      })
      .or_insert_with(|| {
        log::info!("更新预买量时,分帐户持仓信息表不存在持仓信息，需要写入新持仓数据");
        let as_pos = UserStockPosistion::convert_from_deal(&dealinfo);
        let pos_info = PhoenixAccountStockPositionService::convert_from_user_stock_position(&as_pos, &tradectl);
        is_new = true;
        pos_info
      });

    //插入新的记录，需要在历史记录表里也写入一条数据
    // if is_new {
    //   //处理历史记录表
    //   let his_account_position = CommonService::convert_from_stock_positions_channel(&position_info, tradectl.preinit_date);
    //   log::info!("需要写入的分帐户持仓信息:{:#?}", his_account_position);
    //   let ret = PhoenixStockPositionChannelHis::insert(&his_account_position, &dbconn).await;
    //   if ret.as_ref().is_err() {
    //     log::error!("插入历史分帐户信息失败");
    //   }
    // }

    let retval = self.account_position_cache.get(&key).unwrap().clone();
    Ok((retval, is_new))
  }

  pub async fn update_account_stock_positions_by_dealinfo(&self, dealinfo: &PhoenixDealDetail, tradectl: &PhoenixSysSystem, dbconn: &DbConnection) -> Result<Vec<PhoenixStockPositionChannel>> {
    //1）从缓存中查找该分帐号的持仓信息是否存在
    // 1.1 不存在则从交易账户持仓初始化

    //卖单
    //2)根据均价*数量*currency_rate计算total_value_hkd
    //3)把缓存的total_value_hkd减去计算的total_value_hkd,缓存的current_amount减去订单的current_amount

    //买单
    //2)根据current_amout*deal_price*currency_rate计算total_value_hkd
    //3)增加current_amount,增加total_value_hkd，然后计算并更新均价

    //4)更新last_price_hkd
    let new_total_value: Decimal; // = Decimal::from(0);
    let new_total_value_hkd: Decimal;
    let new_current_amout: i32; // = 0;

    let key = format!("{}-{}", dealinfo.p_stock_id, dealinfo.p_channel_id);

    if let Some(mut val) = self.account_position_cache.get_mut(&key) {
      if dealinfo.p_order_direction == constant::OrderDirection::BUY as i32 {
        //买单:增加总total_value，增加current_amount,计算avg_price
        let deal_value = Decimal::from(dealinfo.p_deal_amount).mul(dealinfo.p_deal_price);

        new_total_value = val.p_total_value + deal_value;
        new_total_value_hkd = val.p_total_value_hkd + deal_value * dealinfo.p_currency_rate_cj;
        new_current_amout = val.p_current_amount + dealinfo.p_deal_amount;
        if new_current_amout == 0 {
          val.p_avg_price = Decimal::from(0);
          val.p_avg_price_hkd = Decimal::from(0);
        } else {
          val.p_avg_price = new_total_value / Decimal::from(new_current_amout);
          val.p_avg_price_hkd = new_total_value_hkd / Decimal::from(new_current_amout);
        }
      } else {
        //卖单，卖单不影响均价
        let deal_value = Decimal::from(dealinfo.p_deal_amount) * val.p_avg_price;
        let deal_value_hkd = Decimal::from(dealinfo.p_deal_amount) * val.p_avg_price_hkd;
        new_total_value = val.p_total_value - deal_value;
        new_total_value_hkd = val.p_total_value_hkd - deal_value_hkd;
        new_current_amout = val.p_current_amount - dealinfo.p_deal_amount;
      }

      val.p_total_value = new_total_value;
      val.p_total_value_hkd = new_total_value_hkd;
      val.p_current_amount = new_current_amout;
      val.p_currency_rate = dealinfo.p_currency_rate;
      val.p_date = tradectl.init_date;
      val.p_fee_hkd += dealinfo.p_fee_total * dealinfo.p_currency_rate_cj;
    } else {
      log::error!("SHOULD NOT HAPPEN!!! can not find stock posotion info by key:{}", &key);
      return Err(anyhow!("Can't find positions by key:{}", &key));
      // if let Ok(mut tstock_pos) = StockPosition::query_many(dealinfo.p_unit_id, dealinfo.p_stock_id, dealinfo.p_channel_id, &dbconn).await {
      //   if (&tstock_pos).len() <= 0 {
      //     log::error!("不能根据订单信息{:?}找到持仓信息", &dealinfo);
      //   } else {
      //     let mut stock_pos: Vec<UserStockPosistion> = Vec::new();
      //     for position in tstock_pos.iter_mut() {
      //       position.l_current_amount = 0; //重置当前数量为0，防止发生获取时已有新的交易产生导致持仓量不准确
      //       let as_pos = UserStockPosistion::convert_from_stock_positions(&position, dealinfo.p_account_id, dealinfo.p_leverage, dealinfo.p_currency_rate);
      //       stock_pos.push(as_pos);
      //     }
      //     self.update_account_stock_position_cache(&stock_pos, &tradectl);
      //     //更新费用
      //     self.account_position_cache.entry(key).and_modify(|f| f.p_fee_hkd += dealinfo.p_fee_total * dealinfo.p_currency_rate_cj);
      //   }
      // }
    }
    Ok(self.get_account_stock_positions(dealinfo.p_account_id))
  }

  pub async fn query_stock_positions(&self, stockid: i64, channelid: i64) -> Vec<PhoenixStockPositionChannel> {
    let ret: Vec<PhoenixStockPositionChannel> = self
      .account_position_cache
      .iter()
      .filter(|x| x.p_stock_id == stockid && if channelid > 0 { x.p_channel_id == channelid } else { x.p_channel_id >= 0 })
      .map(|v| v.clone())
      .collect();
    ret
  }

  //1) 使用最新价代替均价,同时用最新价*数量替换总市值
  pub async fn reset_profit(&self, account_id: i64) -> Decimal {
    log::info!("account positions before reset:{:#?}", &self.account_position_cache);
    // let mut ret: Vec<PhoenixStockPositionChannel> = Vec::new();
    let mut financial_borrowed = Decimal::from(0);

    self.account_position_cache.iter_mut().for_each(|mut val| {
      if val.p_account_unit_id == account_id {
        // for pos in val {
        val.p_avg_price = val.p_last_price;
        // val.p_avg_price_hkd = val.p_last_price_hkd;
        val.p_avg_price_hkd = val.p_last_price * val.p_currency_rate;
        val.p_total_value_hkd = val.p_avg_price_hkd * Decimal::from(val.p_current_amount);
        val.p_total_value = val.p_last_price * Decimal::from(val.p_current_amount);
        financial_borrowed += val.p_total_value_hkd * (Decimal::from(1) - val.p_leverage);
      }
    });

    // for (_k, val) in self.account_position_cache.iter_mut() {
    //   if val.p_account_unit_id == account_id {
    //     // for pos in val {
    //     val.p_avg_price = val.p_last_price;
    //     // val.p_avg_price_hkd = val.p_last_price_hkd;
    //     val.p_avg_price_hkd = val.p_last_price * val.p_currency_rate;
    //     val.p_total_value_hkd = val.p_avg_price_hkd * Decimal::from(val.p_current_amount);
    //     val.p_total_value = val.p_last_price * Decimal::from(val.p_current_amount);
    //     financial_borrowed += val.p_total_value_hkd * (Decimal::from(1) - val.p_leverage);
    //   }
    // }
    log::info!("account positions after reseted:{:#?}", &self.account_position_cache);
    financial_borrowed
  }

  fn convert_from_user_stock_position(v_stock: &UserStockPosistion, tradectl: &PhoenixSysSystem) -> PhoenixStockPositionChannel {
    let avg_price: Decimal;
    let avg_price_hkd: Decimal;
    if v_stock.l_current_amount == 0 {
      avg_price = Decimal::from(0);
      avg_price_hkd = Decimal::from(0);
    } else {
      avg_price = v_stock.en_total_value / Decimal::from(v_stock.l_current_amount);
      avg_price_hkd = v_stock.en_total_value_hkd / Decimal::from(v_stock.l_current_amount);
    }

    PhoenixStockPositionChannel {
      id: 0,
      p_stock_code: v_stock.vc_stock_code.clone(),
      p_stock_id: v_stock.l_stock_id,
      p_exchange_id: v_stock.l_exchange_id,
      p_current_amount: v_stock.l_current_amount,
      p_prebuy_amount: v_stock.l_prebuy_amount,
      p_frozen_amount: v_stock.l_frozen_amount,
      p_frozen_amount_temp: v_stock.l_frozen_amount_temp,
      p_avg_price: avg_price,
      p_avg_price_hkd: avg_price_hkd,
      p_total_value: v_stock.en_total_value,
      p_total_value_hkd: v_stock.en_total_value_hkd,
      p_last_price: v_stock.en_last_price,
      p_fee_hkd: Decimal::from(0),
      p_currency_rate: v_stock.en_currency_rate,
      p_stock_type: v_stock.l_stock_type,
      p_channel_id: v_stock.l_channel_id as i64,
      p_qfii_state: v_stock.p_qfii_state,
      p_leverage: v_stock.l_leverage,
      p_account_unit_id: v_stock.l_channel_unit_id, //需要根据channel_id从后台获取对应的账号信息
      p_date: tradectl.init_date,
    }
  }
}
