use crate::{
  dataview::userposition::UserPositionData,
  protofiles::{assetscenter::Phoenixassetspostioninfo, phoenixaccountriskcenter::PhoenixUserPositions},
};
use anyhow::{anyhow, Result};
use common::akaclient::AkaClient;
// use common::akaclient::AkaClient;
use std::sync::Arc;
use tokio::sync::RwLock;
use utility::timeutil::current_timestamp;

pub struct UserPositionService {
  user_positions_data: Arc<RwLock<Vec<RwLock<UserPositionData>>>>,
}

impl UserPositionService {
  pub fn new() -> Self {
    UserPositionService {
      user_positions_data: Arc::new(RwLock::new(Vec::new())),
    }
  }
  //初始化数据
  pub async fn init(&self, ret_data: &Vec<Phoenixassetspostioninfo>, akasvc: &AkaClient) -> Result<()> {
    let positions: Vec<UserPositionData> = ret_data
      .into_iter()
      .map(|f| async_global_executor::block_on(async { UserPositionData::convert_positioninfo_to_userposition(f, akasvc).await }))
      .collect();
    for val in positions.iter() {
      self.set_unit_positions(val).await;
    }
    Ok(())
  }

  //更新用户持仓数据
  pub async fn update_user_positions(&self, posdata: &UserPositionData) -> bool {
    // let user_posinfo = UserPositionData::convert_positioninfo_to_userposition(posdata, akasvc).await;
    self.set_unit_positions(&posdata).await
  }

  //根据stockid查找所有的持仓信息
  pub async fn query_user_positions(&self, unitid: i64, stockid: i64) -> Vec<UserPositionData> {
    let stock_rd = self.user_positions_data.read().await;
    let stocks = stock_rd
      .iter()
      .filter(|f| {
        async_global_executor::block_on(async {
          let fd = f.read().await;
          let b1 = if unitid > 0 { fd.unit_id == unitid } else { fd.unit_id > 0 };
          let b2 = if stockid > 0 { fd.stock_id == stockid } else { fd.stock_id > 0 };
          b1 && b2
        })
      })
      .map(|v| async_global_executor::block_on(async { v.read().await.to_owned() }))
      .collect();

    stocks
  }

  pub async fn query_user_positioins_rpc(&self, unit_id: i64) -> Vec<PhoenixUserPositions> {
    let mut ret = Vec::new();
    let list = self.query_user_positions(unit_id, 0).await;
    for i in list {
      ret.push(PhoenixUserPositions {
        unit_id: i.unit_id,
        stock_id: i.stock_id,
        stock_code: i.stock_code,
        exchange_id: i.channel_id,
        amount: i.current_amount as i64,
        frozen_amount: i.frozen_amount as i64,
        prebuy_amount: i.prebuy_amount as i64,
        qfii_amount: i.qfii_amount as i64,
        margin_ratio: i.margin_rate,
        total_value_hkd: i.total_value_hkd,
        last_price: i.last_price,
        stock_type: i.stock_type,
      })
    }
    ret
  }

  //更新保证金比率
  pub async fn update_positions_margin_rate(&self, rate: f64, unitid: i64, stockid: i64) {
    let stock_rd = self.user_positions_data.read().await;

    for val in stock_rd.iter() {
      let f_rd = val.read().await;
      let b1 = if unitid > 0 { f_rd.unit_id == unitid } else { f_rd.unit_id > 0 };
      let b2 = if stockid > 0 { f_rd.stock_id == stockid } else { f_rd.stock_id > 0 };
      if b1 && b2 {
        val.write().await.margin_rate = rate;
      }
    }
  }

  //更新最新价
  pub async fn update_positions_last_price(&self, price: f64, stockid: i64) {
    let stock_rd = self.user_positions_data.read().await;

    for val in stock_rd.iter() {
      let f_rd = val.read().await;
      if f_rd.stock_id == stockid {
        // let mut wr = val.write().await;
        val.write().await.last_price = price;
      }
    }
  }

  async fn set_unit_positions(&self, posinfo: &UserPositionData) -> bool {
    let mut index: Option<usize> = None;
    {
      let pos_rd = self.user_positions_data.read().await;
      index = pos_rd.iter().position(|f| {
        async_global_executor::block_on(async {
          let f_re = f.read().await;
          f_re.unit_id == posinfo.unit_id && f_re.stock_id == posinfo.stock_id
        })
      });
    }
    //
    let mut re_calc: bool = false; //是否重算的标志
    {
      if index.is_none() {
        //insert
        let mut pos_wr = self.user_positions_data.write().await;
        pos_wr.push(RwLock::new(posinfo.to_owned()));
        re_calc = true;
      } else {
        //update
        let pos_rd = self.user_positions_data.read().await;
        let index = index.unwrap();
        let get_val = pos_rd.get(index);
        if get_val.is_none() {
          re_calc = false;
        } else {
          let gval = get_val.unwrap();
          let mut wr_val = gval.write().await;
          if wr_val.current_amount != posinfo.current_amount {
            *wr_val = posinfo.to_owned();
            re_calc = true;
          } else {
            re_calc = false;
          }
        }
      }
    }
    log::info!("updated user position data:{:?}", &self.user_positions_data.read().await);
    re_calc
  }
}
