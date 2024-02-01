use crate::config::settings::Settings;
use crate::dataservice::dbinit::DbConnection;
use crate::protofiles::phoenixordermsg::OrderMsg;
use anyhow::Result;
use messagecenter::protofiles::hqmsg::YsHqInfo;
// use parking_lot::RwLock;
use std::sync::Arc;
use std::{collections::HashMap, sync::atomic::AtomicBool};

use async_channel::{Receiver, Sender};
use dashmap::DashMap;
use tokio::sync::RwLock;

//
// #[derive(Clone)]
pub struct ServerController {
  pub settings: Arc<Settings>,
  pub order_receiver: Receiver<OrderMsg>,
  pub order_sender: Sender<OrderMsg>,
  pub order_cache: Arc<DashMap<String, Vec<RwLock<OrderMsg>>>>, //订单缓存,每次收到订单，需要保存数据库,key:stock_code,value:order,订单信息放入redis缓存？
  pub db_conn: Arc<DbConnection>,
  // pub deal_time: AtomicBool, //是否交易时间
}

//处理业务逻辑
impl ServerController {
  pub async fn new_order(&self, order: &OrderMsg) -> Result<()> {
    // self.order_cache.get(order.stock_code);
    let key_val = order.stock_code.to_owned();
    let new_order = RwLock::new(order.to_owned());
    //如果不存在，则插入队列
    match self.order_cache.get_mut(&key_val) {
      Some(mut val) => val.push(new_order),
      None => {
        let orders = vec![new_order];
        self.order_cache.insert(key_val, orders);
      }
    }

    Ok(())
  }
  //处理成交
  pub async fn deal_order(&self, quotation: &YsHqInfo) -> Result<()> {
    //如果是交易时间，则处理成交
    let key_val = quotation.contract_no1.to_owned();
    let stock_orders = self.order_cache.get(&key_val);
    if stock_orders.is_none() {
      return Ok(()); //无订单
    }
    let orders = stock_orders.unwrap();
    for val in orders.iter() {
      let order = val.write().await;
      //处理成交
      if order.order_direction == common::constant::OrderDirection::BUY as i32 {
        //处理卖单
      } else if order.order_direction == common::constant::OrderDirection::SELL as i32 {
        //处理卖单
      }
      //发送到channel？
      let ret = self.order_sender.send(OrderMsg { ..Default::default() }).await;
    }
    log::info!("当前行情价格：{}", &quotation.q_average_price);
    Ok(())
  }

  pub async fn persist_database(&self) -> Result<()> {
    Ok(())
  }
}
