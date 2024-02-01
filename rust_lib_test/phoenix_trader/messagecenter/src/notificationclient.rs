use std::sync::Arc;

// 消息中心客户端，实现publish和consume功能
use crate::protofiles::phoenixnotification::NotificationMessage;
use anyhow::Result;
use futures_lite::stream::StreamExt;

use deadpool_lapin::lapin::{options::BasicPublishOptions, BasicProperties};
use deadpool_lapin::{Config, Pool, Runtime};
use lapin::{options::*, types::FieldTable, Channel};
use prost::Message;

#[derive(Debug, Clone)]
pub struct NotificationClient {
  addr: String,
  publish_channel: Option<Channel>,
  consume_channel: Option<Channel>,
  routingkey: String,
  qsender: tokio::sync::mpsc::Sender<NotificationMessage>,
  queue_name: String, /*queue name */
  exchanger: String,
  // connection: Arc<Connection>,
  pool: Arc<Pool>,
}

impl NotificationClient {
  //routing key通过","分割，多次绑定 mc.order.*,mc.assets.*
  pub async fn new(exchanger: &str, queue_name: &str, routingkey: String, addr: &str, qtx: tokio::sync::mpsc::Sender<NotificationMessage>) -> Self {
    // let connection = Connection::connect(addr, ConnectionProperties::default()).await.expect("connect error.....");
    let mut cfg = Config::default();
    cfg.url = Some(addr.into());
    let pool = cfg.create_pool(Some(Runtime::Tokio1)).expect("create connection pool error");
    // let connection = pool.get().await.expect("get connection error");
    // let publish_channel = connection.create_channel().await.expect("create publish channel error");
    // let consume_channel = connection.create_channel().await.expect("create consume channel error");
    let client = NotificationClient {
      addr: addr.to_string(),
      publish_channel: None,
      consume_channel: None,
      queue_name: String::from(queue_name),
      exchanger: exchanger.to_string(),
      routingkey,
      qsender: qtx,
      // connection: Arc::new(connection),
      pool: Arc::new(pool),
    };
    log::info!("notification client inited successfully...");
    client
  }

  pub async fn retry_connect(&mut self) -> Result<()> {
    // let connection = Connection::connect(self.addr.as_str(), ConnectionProperties::default()).await;
    let mut cfg = Config::default();
    cfg.url = Some(self.addr.clone());
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
    let connection = pool.get().await?;
    let publish_channel = connection.create_channel().await.expect("create publish channel error");
    let consume_channel = connection.create_channel().await.expect("create consume channel error");
    // self.connection = Arc::new(connection.unwrap());
    self.pool = Arc::new(pool);
    self.publish_channel = Some(publish_channel);
    self.consume_channel = Some(consume_channel);
    Ok(())
  }

  pub async fn try_connect(&mut self) -> Result<()> {
    let connection = self.pool.get().await?;
    let channel = connection.create_channel().await;
    if channel.as_ref().is_err() {
      log::error!("create channel error:{}", channel.as_ref().err().unwrap().to_string());
      return Err(anyhow!("create channel error:{}", channel.as_ref().err().unwrap().to_string()));
    }
    self.publish_channel = Some(channel.unwrap());
    let channel = connection.create_channel().await;
    if channel.as_ref().is_err() {
      log::error!("create channel error:{}", channel.as_ref().err().unwrap().to_string());
      return Err(anyhow!("create channel error:{}", channel.as_ref().err().unwrap().to_string()));
    }
    self.consume_channel = Some(channel.unwrap());

    Ok(())
  }

  pub async fn try_publish(&self, routing_key: &str, data: &NotificationMessage) -> Result<()> {
    if self.publish_channel.is_none() {
      return Err(anyhow!("publish channel is none"));
    }

    log::info!("publish notification message, data is:{:?}", data);
    let mut payload = Vec::new();
    let ret = data.encode(&mut payload);
    if ret.as_ref().is_err() {
      return Err(anyhow!("encode error:{:?}", ret.as_ref().err().unwrap()));
    }

    let publish_channel = self.publish_channel.as_ref().unwrap();

    let ret = publish_channel
      .basic_publish(&self.exchanger, routing_key, BasicPublishOptions::default(), &payload, BasicProperties::default())
      .await;

    if ret.as_ref().is_err() {
      log::error!("publish_channel error:{}", ret.as_ref().err().unwrap().to_string());
      return Err(anyhow!("publish_channel error:{}", ret.as_ref().err().unwrap().to_string()));
    }
    let pub_confirm = ret.unwrap().await;
    if pub_confirm.as_ref().is_err() {
      log::error!("wait confirm error:{}", pub_confirm.as_ref().err().unwrap().to_string());
      return Err(anyhow!("wait confirm error:{}", pub_confirm.as_ref().err().unwrap().to_string()));
    }
    let confirm = pub_confirm.unwrap();
    if confirm.is_nack() {
      log::error!("not confirmed");
      return Err(anyhow!("not confirmed"));
    }
    Ok(())
  }

  pub async fn update_bindings(&mut self, new_keys: &str) -> Result<()> {
    if self.consume_channel.is_none() {
      return Err(anyhow!("consume channel is none"));
    }
    let keys = self.routingkey.split(",");

    for key in keys {
      let ret = self
        .consume_channel
        .as_ref()
        .unwrap()
        .queue_unbind(self.queue_name.as_str(), self.exchanger.as_str(), key, FieldTable::default())
        .await;
      if ret.as_ref().is_err() {
        log::error!("queue_unbind error:{}", ret.as_ref().err().unwrap().to_string());
        return Err(anyhow!("queue_unbind error:{}", ret.as_ref().err().unwrap().to_string()));
      }
    }

    self.routingkey = new_keys.to_string();
    let keys = self.routingkey.split(",");

    for key in keys {
      let ret = self
        .consume_channel
        .as_ref()
        .unwrap()
        .queue_bind(self.queue_name.as_str(), self.exchanger.as_str(), key, QueueBindOptions::default(), FieldTable::default())
        .await;
      if ret.as_ref().is_err() {
        log::error!("queue_bind error:{}", ret.as_ref().err().unwrap().to_string());
        return Err(anyhow!("queue_bind error:{}", ret.as_ref().err().unwrap().to_string()));
      }
    }
    Ok(())
  }

  pub async fn try_consume(&self) -> Result<()> {
    if self.consume_channel.is_none() {
      return Err(anyhow!("consume channel is none"));
    }
    let declare_option = QueueDeclareOptions {
      passive: false,
      durable: false,
      exclusive: false,
      auto_delete: true,
      nowait: false,
    };

    let ret = self.consume_channel.as_ref().unwrap().queue_declare(self.queue_name.as_str(), declare_option, FieldTable::default()).await;
    if ret.as_ref().is_err() {
      log::error!("queue_declare error:{}", ret.as_ref().err().unwrap().to_string());
      return Err(anyhow!("queue_declare error:{}", ret.as_ref().err().unwrap().to_string()));
    }
    let keys = self.routingkey.split(",");

    for key in keys {
      let ret = self
        .consume_channel
        .as_ref()
        .unwrap()
        .queue_bind(self.queue_name.as_str(), self.exchanger.as_str(), key, QueueBindOptions::default(), FieldTable::default())
        .await;
      if ret.as_ref().is_err() {
        log::error!("queue_bind error:{}", ret.as_ref().err().unwrap().to_string());
        return Err(anyhow!("queue_bind error:{}", ret.as_ref().err().unwrap().to_string()));
      }
    }

    if let Ok(mut consumer) = self
      .consume_channel
      .as_ref()
      .unwrap()
      .basic_consume(self.queue_name.as_str(), "*", BasicConsumeOptions::default(), FieldTable::default())
      .await
    {
      while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
          log::info!("incoming message: {:?}", delivery.data);
          if let Ok(inforeq) = NotificationMessage::decode(&*delivery.data) {
            let _ = self.qsender.send(inforeq.to_owned()).await;
          }

          delivery.ack(BasicAckOptions::default()).await.expect("ack");
        }
      }
    } else {
      log::error!("basic_consume error......");
      return Err(anyhow!("consume error..."));
    }
    Ok(())
  }
}
