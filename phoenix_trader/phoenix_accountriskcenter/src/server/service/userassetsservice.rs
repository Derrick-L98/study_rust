use crate::dataservice::entities::prelude::PhoenixRiskDetails;
use crate::protofiles::phoenixaccountriskcenter::{PhoenixUserAssets, UserAssetsReq, UserAssetsResp};
use crate::{
  dataview::{userassets::UserAssetsData, userposition::UserPositionData},
  protofiles::assetscenter::PhoenixassetsResultInfo,
};
use anyhow::{anyhow, Result};
use common::constant;
use common::{akaclient::AkaClient, constant::StockType};
use dashmap::DashMap;
// use messagecenter::protofiles::phoenixnotification::NotificationAsset;
use utility::timeutil::current_timestamp;

use super::commonservice::CommonService;
// use std::sync::Arc;
// use tokio::sync::RwLock;

pub struct UserAssetsService {
  user_assets_data: DashMap<i64, UserAssetsData>,
}

impl UserAssetsService {
  pub fn new() -> Self {
    UserAssetsService { user_assets_data: DashMap::new() }
  }
  //初始化数据
  pub async fn init(&self, ret_data: &Vec<PhoenixassetsResultInfo>, akasvc: &AkaClient) -> Result<()> {
    self.user_assets_data.clear();
    if ret_data.len() <= 0 {
      return Err(anyhow!("Empty data"));
    }

    let mut assets_data: Vec<UserAssetsData> = ret_data.into_iter().map(|f| UserAssetsData::convert_assetsinfo_to_assetsdata(f, current_timestamp())).collect();

    for val in assets_data.iter_mut() {
      let ret = self.set_unit_assets(val, akasvc).await;
      if ret.as_ref().is_err() {
        log::error!("query account by id:{} error:{}", val.unit_id, ret.as_ref().unwrap_err().to_string());
        continue;
      }
    }

    Ok(())
  }
  //rpc接口查询资产
  pub async fn query_assets(&self, req: &UserAssetsReq) -> Result<UserAssetsResp> {
    let mut ret_data = UserAssetsResp { ..Default::default() };
    let d_op = self.user_assets_data.get(&req.unit_id);
    if d_op.is_none() {
      return Err(anyhow!("查询用户不存在!"));
    }
    let mut ret = PhoenixUserAssets { ..Default::default() };
    let d = d_op.unwrap();
    ret.unit_id = req.unit_id;
    ret.total_position_value = d.position_value;
    ret.gem_position_value = d.position_value_cyb;
    ret.real_margin = d.margin_use;
    ret.real_cash = d.current_cash;
    ret.risk_rate = 0.0;
    ret.trade_state = d.trade_flag;
    ret.warning_line = d.warn_line;
    ret.available_cash = d.current_cash - d.margin_use;
    ret.net_income = d.today_deposit - d.total_withdraw;
    ret.hold_yk = d.hold_yk;
    ret.total_asset = d.current_cash + d.hold_yk - d.frozen_capital;
    ret.draw_frozen = d.frozen_capital;
    ret.trade_frozen_capital = d.trade_frozen_capital;
    ret.today_yk = ret.total_asset - d.last_cash - d.today_deposit + d.today_withdraw;
    ret.total_yk = ret.total_asset - d.total_deposit + d.total_withdraw;
    ret_data.assets = Some(ret);
    return Ok(ret_data);
  }

  pub async fn update_user_assets_by_positions(&self, unitid: i64, positions: &Vec<UserPositionData>) -> Result<()> {
    // if unitid == constant::VALUE_ALL {
    //   self.user_assets_data.iter_mut().for_each(|f| {
    //     let ret = UserAssetsService::compute_user_assets(&*f, positions);
    //     if ret.is_err() {
    //       log::error!("compute_user_assets error:{}", ret.as_ref().unwrap_err().to_string());
    //     }
    //   });
    // } else {
    //   let userassets = self.user_assets_data.get(&unitid);
    //   if userassets.is_none() {
    //     return Err(anyhow!("can't find assets info by id:{}", unitid));
    //   }
    //   let assets = userassets.unwrap();
    //   let ret = UserAssetsService::compute_user_assets(&assets, positions);
    //   if ret.is_err() {
    //     return Err(anyhow!("compute_user_assets error:{}", ret.as_ref().unwrap_err().to_string()));
    //   }
    //   self.user_assets_data.insert(unitid, ret.unwrap());
    // }
    Ok(())
  }

  //更新用户资产数据
  // pub async fn update_user_assets_by_resultinfo(&self, assets: &PhoenixassetsResultInfo, akasvc: &AkaClient) -> Result<()> {
  //   let userassetsdata = UserAssetsData::convert_assetsinfo_to_assetsdata(assets, current_timestamp());
  //   let ret = self.set_unit_assets(&userassetsdata, akasvc).await;
  //   if ret.as_ref().is_err() {
  //     return Err(anyhow!("set unit assets error"));
  //   }
  //   let positions: Vec<UserPositionData> = Vec::new();
  //   // let rateinfo = akasvc.query_exchange_rate(&userassetsdata.currency_no).await;
  //   let ret = UserAssetsService::compute_user_assets(&userassetsdata, &positions);
  //   if ret.is_err() {
  //     return Err(anyhow!("compute_user_assets error:{}", ret.as_ref().unwrap_err().to_string()));
  //   }
  //   Ok(())
  // }

  //更新用户资产数据
  pub async fn update_user_assets(&self, assets: &UserAssetsData, akasvc: &AkaClient) -> Result<()> {
    let ret = self.set_unit_assets(assets, akasvc).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!("set unit assets error"));
    }
    Ok(())
  }

  pub async fn query_total_account_assets(&self, accountid: i64) -> UserAssetsData {
    let mut ret = UserAssetsData { ..Default::default() };
    for assets in self.user_assets_data.iter() {
      if assets.unit_id == accountid {
        continue;
      }
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
      ret.position_value += assets.position_value;
      ret.position_value_cyb += assets.position_value_cyb;
      ret.margin_use += assets.margin_use;
    }
    ret.unit_id = accountid;
    return ret;
  }

  async fn set_unit_assets(&self, val: &UserAssetsData, akasvc: &AkaClient) -> Result<()> {
    if !self.user_assets_data.contains_key(&val.unit_id) {
      //如果不存在
      let mut val = val.to_owned();
      let ret = akasvc.query_account_info(val.unit_id).await;
      if ret.as_ref().is_err() {
        return Err(anyhow!("query account by id:{} error:{}", val.unit_id, ret.as_ref().unwrap_err().to_string()));
      }
      let account_info = ret.unwrap();
      val.close_line = account_info.close_line;
      val.warn_line = account_info.warning_line;
      val.credit_multiple = account_info.level_rate.parse::<f64>().unwrap_or_default();
      val.trade_flag = account_info.trade_state as i32;
      self.user_assets_data.insert(val.unit_id, val);
    } else {
      self.user_assets_data.entry(val.unit_id).and_modify(|f| {
        f.begin_cash = val.begin_cash;
        f.current_cash = val.current_cash;
        f.frozen_capital = val.frozen_capital;
        f.trade_frozen_capital = val.trade_frozen_capital;
        f.cash_in_transit = val.cash_in_transit;
        f.credit_multiple = val.credit_multiple;
        f.currency_no = val.currency_no.to_owned();
        f.last_time = val.last_time;
        f.total_deposit = val.total_deposit;
        f.today_deposit = val.today_deposit;
        f.today_withdraw = val.today_withdraw;
        f.total_withdraw = val.total_withdraw;
        f.last_cash = val.last_cash;
      });
    }
    Ok(())
  }

  pub fn get_all_unit_id(&self) -> Vec<i64> {
    let mut v = Vec::new();
    for i in self.user_assets_data.iter() {
      v.push(i.unit_id);
    }
    v
  }

  //重算资产
  pub fn compute_user_assets(&self, unit_id: i64, postions: &Vec<UserPositionData>) -> Result<()> {
    if !self.user_assets_data.contains_key(&unit_id) {
      return Ok(());
    }

    let mut userassets = self.user_assets_data.get_mut(&unit_id).unwrap();
    if postions.len() > 0 {
      let mut positionvalue = 0.0;
      let mut positionvalue_cyb = 0.0;
      let mut marginuse = 0.0;
      let mut holdyk = 0.0;
      for pos in postions.iter() {
        if pos.unit_id != userassets.unit_id {
          continue;
        }
        let tvalue = pos.current_amount as f64 * pos.last_price * pos.currency_rate;
        positionvalue += tvalue;
        if pos.stock_type == StockType::HSCY as i32 || pos.stock_type == StockType::HSKC as i32 {
          positionvalue_cyb += tvalue;
        }
        marginuse += tvalue * pos.margin_rate;
        holdyk += tvalue - pos.total_value_hkd;
      }

      userassets.position_value = positionvalue;
      userassets.position_value_cyb = positionvalue_cyb;
      userassets.margin_use = marginuse;
      userassets.hold_yk = holdyk;
    }

    if userassets.current_cash == 0.0 {
      userassets.risk_rate = 0.0;
    } else {
      let riskrate = userassets.margin_use / userassets.current_cash;
      userassets.risk_rate = riskrate;
    }

    self.user_assets_data.entry(userassets.unit_id).and_modify(|f| {
      f.position_value = userassets.position_value;
      f.margin_use = userassets.margin_use;
      f.hold_yk = userassets.hold_yk;
      f.position_value_cyb = userassets.position_value_cyb;
    });

    // let mut risk_warning: Option<PhoenixRiskDetails> = None;
    // if userassets.risk_rate > userassets.warn_line {
    //   //超过预警线
    //   let rwarn = CommonService::phoenix_risk_details_new(&userassets);
    //   risk_warning = Some(rwarn);
    //   //insert into database???
    // }

    Ok(())
  }
}
