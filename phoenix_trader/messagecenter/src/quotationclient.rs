use std::collections::HashMap;

// use std::default::default;
use anyhow::Result;
use futures_lite::stream::StreamExt;
use lapin::{options::*, types::FieldTable, Channel};
use std::sync::Arc;

use common::constant;
// use deadpool_lapin::lapin::{options::BasicPublishOptions, BasicProperties};
use deadpool_lapin::{Config, Pool, Runtime};

use crate::protofiles::hqmsg::{HqMsgReq, YsHqInfo};
use prost::Message;

#[derive(Debug, Clone)]
pub struct QuotationClient {
  addr: String,
  routingkey: HashMap<String, i32>,
  qsender: tokio::sync::mpsc::Sender<YsHqInfo>,
  channel: Channel,
  queue_name: String, /*queue name */
  exchanger: String,
  pool: Arc<Pool>, /*connection pool*/
}

impl QuotationClient {
  pub async fn new(exchanger: &str, queue_name: &str, routingkey: HashMap<String, i32>, addr: &str, qtx: tokio::sync::mpsc::Sender<YsHqInfo>) -> Self {
    let mut cfg = Config::default();
    cfg.url = Some(addr.into());
    let pool = Arc::new(cfg.create_pool(Some(Runtime::Tokio1)).expect("create connection pool error"));
    // let connection = Connection::connect(addr, ConnectionProperties::default()).await.unwrap();
    let connection = pool.get().await.expect("get connection error");
    let channel = connection.create_channel().await.expect("create channel error");
    let client = QuotationClient {
      addr: addr.to_string(),
      channel,
      queue_name: String::from(queue_name),
      exchanger: exchanger.to_string(),
      routingkey,
      qsender: qtx,
      pool,
    };
    log::info!("quotation mq client inited successfully...");

    client
  }

  pub async fn retry_consume(&mut self) -> Result<()> {
    // let connection = Connection::connect(self.addr.as_str(), ConnectionProperties::default()).await;
    let mut cfg = Config::default();
    cfg.url = Some(self.addr.clone());
    let ret = cfg.create_pool(Some(Runtime::Tokio1));
    if ret.is_err() {
      log::error!("create connection pool error");
      return Err(anyhow!("create connection error"));
    }
    self.pool = Arc::new(ret.unwrap());

    let ret = self.pool.get().await;
    if ret.is_err() {
      log::error!("get connection error");
      return Err(anyhow!("get connection error"));
    }
    let connection = ret.unwrap();

    let ret = connection.create_channel().await;
    if ret.is_err() {
      log::error!("create channel error");
      return Err(anyhow!("create channel error"));
    }
    // let channel = ret.unwrap();
    self.channel = ret.unwrap();
    self.try_consume().await
  }

  pub async fn update_bindings(&mut self, new_keys: &HashMap<String, i32>) -> Result<()> {
    log::info!("当前的routingkey: {:?}", &self.routingkey);
    log::info!("传入新的routing key:{:?}", &new_keys);

    //如果新传入的值，当前不存在，则绑定
    let to_bind = new_keys.iter().filter(|(x, _u)| self.routingkey.get(x.to_owned()).is_none()).collect::<HashMap<&String, &i32>>();
    log::info!("To Bind:{:?}", &to_bind);
    // 首先根据传入的new_keys，如果当前不存在，则bind
    let to_unbind = self.routingkey.iter().filter(|(x, _u)| new_keys.get(x.to_owned()).is_none()).collect::<HashMap<&String, &i32>>();
    log::info!("To UnBind:{:?}", &to_unbind);

    if to_unbind.len() > 0 {
      for val in to_unbind.keys() {
        let ret = self.channel.queue_unbind(self.queue_name.as_str(), self.exchanger.as_str(), val.as_str(), FieldTable::default()).await;
        if ret.as_ref().is_err() {
          log::error!("update queue unbind error...");
        }
      }
      // let to_unbind_cloned = to_unbind.into_iter().map(|(k,v)| (k.clone(), v.clone())).collect::<HashMap<String, i32>>();
      // self.routingkey.retain(|k,_| to_unbind_cloned.get(k).is_none() );

      let keys = to_unbind.keys().into_iter().map(|&k| k.to_owned()).collect::<Vec<String>>();
      for vk in keys {
        self.routingkey.remove(&vk);
      }
    }
    // bind
    if to_bind.len() > 0 {
      for val in to_bind.keys() {
        self
          .channel
          .queue_bind(
            self.queue_name.as_str(),
            self.exchanger.as_str(),
            // "stock.#",
            val.as_str(),
            QueueBindOptions::default(),
            FieldTable::default(),
          )
          .await?;
      }
      self.routingkey.extend(to_bind.into_iter().map(|(k, v)| (k.clone(), v.clone())));
      // log::info!("更新的routingkey: {:?}", self.routingkey);
    }
    Ok(())
  }

  /// 追加绑定新的行情订阅
  pub async fn append_bindings(&mut self, new_keys: &HashMap<String, i32>) -> Result<()> {
    // let connection = self.get_connection().await.map_err(|e| {
    //     log::info!("could not get rabbitmq connection: {}", e);
    //     e
    // })?;
    log::info!("当前的routingkey: {:?}", &self.routingkey);
    log::info!("传入新的routing key:{:?}", &new_keys);

    //如果新传入的值，当前不存在，则绑定
    let to_bind = new_keys.iter().filter(|(x, _)| self.routingkey.get(x.to_owned()).is_none()).collect::<HashMap<&String, &i32>>();
    log::info!("To Bind:{:?}", &to_bind);

    // let channel = self.connection.create_channel().await;
    // if channel.is_err() {
    //     return Err(anyhow!("create channel error"));
    // }
    // let channel = channel?;
    // bind
    for val in to_bind.keys() {
      self
        .channel
        .queue_bind(
          self.queue_name.as_str(),
          self.exchanger.as_str(),
          // "stock.#",
          val.as_str(),
          QueueBindOptions::default(),
          FieldTable::default(),
        )
        .await?;
    }
    // self.routingkey.extend(to_bind.iter());
    self.routingkey.extend(to_bind.into_iter().map(|(k, v)| (k.clone(), v.clone())));
    log::info!("更新的routingkey: {:?}", self.routingkey);
    Ok(())
  }

  pub async fn append_unbindings(&mut self, new_keys: &HashMap<String, i32>) -> Result<()> {
    // let connection = self.get_connection().await.map_err(|e| {
    //     log::info!("could not get rabbitmq connection: {}", e);
    //     e
    // })?;
    log::info!("当前的routingkey: {:?}", &self.routingkey);
    log::info!("传入新的routing key:{:?}", &new_keys);

    let to_unbind = self.routingkey.iter().filter(|(x, _)| new_keys.get(x.to_owned()).is_none()).collect::<HashMap<&String, &i32>>();
    log::info!("To UnBind:{:?}", &to_unbind);

    // let channel = self.connection.create_channel().await;
    // if channel.is_err() {
    //     return Err(anyhow!("create channel error"));
    // }
    // let channel = channel?;
    // self.routingkey.extend(to_unbind);
    // unbind
    for val in to_unbind.keys() {
      self
        .channel
        .queue_unbind(self.queue_name.as_str(), self.exchanger.as_str(), val.as_str(), FieldTable::default())
        .await?;
    }
    let keys = to_unbind.keys().into_iter().map(|&k| k.to_owned()).collect::<Vec<String>>();
    for vk in keys {
      self.routingkey.remove(&vk);
    }
    // self.routingkey.remove(&"|v| v.".to_string());

    // self.routingkey.retain(|k,_| to_unbind.get(k).is_none());//仅保留取消绑定中不存在的
    log::info!("更新的routingkey: {:?}", self.routingkey);

    Ok(())
  }

  pub async fn update_bind_queue(&mut self, key: &str, isbind: constant::YesOrNo) -> Result<()> {
    // let connection = self.get_connection().await.map_err(|e| {
    //     log::info!("could not get rabbitmq connection: {}", e);
    //     e
    // })?;
    log::info!("当前的routingkey: {:?}", self.routingkey);

    // let channel = self.connection.create_channel().await;
    // if channel.is_err() {
    //     return Err(anyhow!("create channel error"));
    // }
    // let channel = channel?;
    // let vkey = self.routingkey.cache_get(&key);
    if isbind == constant::YesOrNo::YES {
      let vkey = self.routingkey.entry(key.to_owned()).or_insert(0);
      *vkey += 1; //增加新的订阅，加1
      if *vkey == 1 {
        log::info!("增加新的routingkey绑定...");
        let _ret = self
          .channel
          .queue_bind(
            self.queue_name.as_str(),
            self.exchanger.as_str(),
            // "stock.#",
            &key,
            QueueBindOptions::default(),
            FieldTable::default(),
          )
          .await;
      }
      // log::info!("key:{} 绑定成功！", &key);
    } else {
      let vkey = self.routingkey.entry(key.to_string()).or_default();
      *vkey -= 1; //减1，如果结果是<=0,则从key中移除，同时取消订阅
      if *vkey <= 0 {
        log::info!("取消routingkey绑定...");
        self.routingkey.remove(&key.to_string());
        let _ret = self.channel.queue_unbind(self.queue_name.as_str(), self.exchanger.as_str(), key, FieldTable::default()).await;
      }
      // log::info!("key: {}  取消绑定成功！", &key);
    }
    log::info!("更新的routingkey: {:?}", self.routingkey);
    Ok(())
  }

  pub async fn try_consume(&mut self) -> Result<()> {
    let declare_option = QueueDeclareOptions {
      passive: false,
      durable: false,
      exclusive: false,
      auto_delete: true,
      nowait: false,
    };

    self.channel.queue_declare(self.queue_name.as_str(), declare_option, FieldTable::default()).await?;
    // log::info!("Rabbitmq queue_declare done: {:?}", &queue);

    for (key, _val) in self.routingkey.iter() {
      // let rkey = format!("{}{}", "stock.", key.as_str());
      self
        .channel
        .queue_bind(
          self.queue_name.as_str(),
          self.exchanger.as_str(),
          key,
          // "stock.3.002620_XSHE",
          QueueBindOptions::default(),
          FieldTable::default(),
        )
        .await?;
    }
    // log::info!("Rabbitmq queue_bind done: {:?}", &channel);
    let mut consumer = self.channel.basic_consume(self.queue_name.as_str(), "*", BasicConsumeOptions::default(), FieldTable::default()).await?;

    while let Some(delivery) = consumer.next().await {
      if let Ok(delivery) = delivery {
        // log::info!("incoming message: {:?}", delivery.data);
        if let Ok(inforeq) = HqMsgReq::decode(&*delivery.data) {
          if let Some(yshqinfo) = inforeq.yshqinfo {
            //send with
            let ret = self.qsender.send(yshqinfo.to_owned()).await;
            if ret.is_err() {
              log::error!("send quotation to main channel error:{:?}", ret);
            }
          }
        }
        //以下方法不适用
        // let inforeq = HqMsgReq::parse_from_bytes(&delivery.data).unwrap();
        // let yshqinfo = inforeq.get_yshqinfo();
        /* ================================= */
        // let _ = channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default()).await;
        delivery.ack(BasicAckOptions::default()).await.expect("ack");
      }
    }

    Ok(())
  }
}
