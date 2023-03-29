#[macro_use]
extern crate anyhow;

extern crate chrono;
// extern crate redis;
// extern crate time;

mod client;
mod commonutil;
mod config;
mod protofiles;
mod server;

use crate::config::settings::Settings;
use crate::protofiles::{hqmsg::HqMsgReq, YsHqInfo,/*WithdrawEnableReq */ };
use crate::protofiles::{phoenix_tick_center_client::PhoenixTickCenterClient, LastPriceReq};
use anyhow::Result;
use commonutil::commonutil::CommonUtil;
use futures_lite::stream::StreamExt;
use lapin::{options::*, types::FieldTable, Channel, Connection, ConnectionProperties};
use prost::Message;
use std::time::Duration;
use utility::loggings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loggings::log_init();
    // let settings = Settings::new().unwrap();
    // let mut client = SvrStockEnableClient::connect("http://0.0.0.0:50058").await?;
    // println!("RESPONSE={:#?}", client.get_withdraw_enable(tonic::Request::new(WithdrawEnableReq {
    //     user_id: 200666,
    // })).await?);
    println!("结束");
    // log::info!("初始化配置信息:{:#?}", &settings);
    // let mut client = PhoenixTickCenterClient::connect("http://[::1]:10001").await.expect("服务连接失败");
    // let data = client.get_last_price(tonic::Request::new(LastPriceReq {
    //     contract_no1: "000657_XSHE".to_string()
    // }))
    // .await?.into_inner();
    // // for i in 0..50{
    //     // println!(
    //     //     "RESPONSE={:#?}",
    //     //     client.get_last_price(tonic::Request::new(LastPriceReq {
    //     //             contract_no1: "301236_XSHE".to_string()
    //     //         }))
    //     //         .await?.into_inner()
    //     // );
    // // }

    // println!("{:#?}", data);
    // let common = CommonUtil::new();
    // common.init(&settings).await.expect("init time error");
    // let ret = common.get_next_interval_begin_time(&"16:00:00".to_string(), &"XSHG".to_string()).await;
    // println!("{:?}", ret);
    mq().await;
    Ok(())
}

async fn mq() -> Result<()> {
    let addr = "amqp://pp:pp123@uat-stock-data.chinaeast2.cloudapp.chinacloudapi.cn:5672";
    // let addr = "amqp://pp:pp123@data.net:5672/%2f";
    // async_global_executor::block_on(async {
    // log::info!("Connecting......");
    println!("Connecting......");

    let conn = Connection::connect(&addr, ConnectionProperties::default()).await.expect("connection error");

    // log::info!("CONNECTED");
    println!("CONNECTED");

    //receive channel
    let channel = conn.create_channel().await.expect("create_channel");
    // info!(state=?conn.status().state());
    let declare_option = QueueDeclareOptions {
        passive: false,
        durable: false,    //持久化标志，表明此交换机是否是持久化的
        exclusive: false,  //只被一个连接（connection）使用，而且当连接关闭后队列即被删除
        auto_delete: true, //删除标志，表明当所有队列在完成使用此exchange时，是否删除
        nowait: false,
    };
    //创建hello队列
    let queue = channel.queue_declare("hello", declare_option, FieldTable::default()).await.expect("queue_declare");
    // log::info!(state=?conn.status().state());
    // log::info!(?queue, "Declared queue");

    channel
        .queue_bind("hello", "StockLive", "stock.#", QueueBindOptions::default(), FieldTable::default())
        .await
        .expect("queue bind");

    // log::info!("will consume");
    //将消耗
    let mut consumer = channel
        .basic_consume("hello", "", BasicConsumeOptions::default(), FieldTable::default())
        .await
        .expect("basic_consume");
    // log::info!("state={:?}}",conn.status().state());

    while let Some(delivery) = consumer.next().await {
        // println!("{:#?}", &delivery);
        // info!(message=?delivery, "received message");
        if let Ok(delivery) = delivery {
            let inforeq = HqMsgReq::decode(&*delivery.data).unwrap();
            // let inforeq = Message::decode::<HqMsgReq>(&delivery.data).unwrap();
            // info!("{:?}", &inforeq);
            // let inforeq = Message::parse_from_bytes(&delivery.data).unwrap();
            // let yshqinfo = inforeq.yshqinfo.unwrap();
            // log::info!("hqinfo:{:?}", &inforeq);
            // println!("hqinfo:{:#?}", &inforeq.yshqinfo);
            if inforeq.yshqinfo.as_ref().unwrap().contract_no1 == "600000_XSHG" {
                log::info!("{:?}", &inforeq.yshqinfo);
            }
            // info!("dkdkdkdkd");
            delivery.ack(BasicAckOptions::default()).await.expect("ack");
        }
    }
    // });

    Ok(())
}

pub async fn initclient_listen(mut client: MqClient) {
    let mut retry_interval = tokio::time::interval(Duration::from_secs(5));

    tokio::spawn(async move {
        // retry_interval.tick().await;
        loop {
            tokio::select! {
               _ = retry_interval.tick() => {
                   log::info!("trying client consume in init......");
                   if let Err(err) = client.try_consume().await{
                       log::error!("client consume error: {:?}. start to re-connecting", err);
                       let _ = client.retry_consume().await;
                   }
               }
            }
        }
    });
    // Ok(())
}

#[derive(Debug, Clone)]
pub struct MqClient {
    pub addr: String,
    pub channel: Channel,
    pub routingkey: String,
    pub qsender: tokio::sync::mpsc::Sender<YsHqInfo>,
    queue_name: String, /*queue name */
    exchanger: String,
}

impl MqClient {
    pub async fn new(exchanger: &str, routingkey: String, addr: &str, qtx: tokio::sync::mpsc::Sender<YsHqInfo>) -> Self {
        // let channel = pool.get().await.unwrap().create_channel().await.unwrap();
        // log::info!("starting create connection");
        let connection = Connection::connect(addr, ConnectionProperties::default()).await.unwrap();
        let channel = connection.create_channel().await.unwrap();
        let client = MqClient {
            addr: addr.to_string(),
            channel,
            queue_name: String::from("phoenix_assetscenter_orders_queue"),
            exchanger: exchanger.to_string(),
            routingkey,
            qsender: qtx,
            // channel,
        };
        log::info!("notification client inited successfully...");
        client
    }

    pub async fn retry_consume(&mut self) -> Result<()> {
        let connection = Connection::connect(self.addr.as_str(), ConnectionProperties::default()).await;
        if connection.is_err() {
            log::error!("create connection error");
            return Err(anyhow!("create connection error"));
        }
        let channel = connection.unwrap().create_channel().await;
        if channel.is_err() {
            log::error!("create channel error");
            return Err(anyhow!("create channel error"));
        }
        self.channel = channel.unwrap();
        self.try_consume().await
    }

    pub async fn try_consume(&mut self) -> Result<()> {
        let declare_option = QueueDeclareOptions {
            passive: false,
            durable: false,
            exclusive: false,
            auto_delete: true,
            nowait: false,
        };
        /*let queue = */
        if let Ok(_) = self.channel.queue_declare(self.queue_name.as_str(), declare_option, FieldTable::default()).await {
            // log::info!("queue declared...");
        } else {
            log::error!("queue_declare error。。。")
        }

        if let Ok(_) = self
            .channel
            .queue_bind(
                self.queue_name.as_str(),
                self.exchanger.as_str(),
                self.routingkey.as_str(),
                // "stock.3.002620_XSHE",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
        {
            // log::info!("queue binded...");
        } else {
            log::error!("queue_bind error。。。")
        }

        // log::info!("Rabbitmq queue_bind done: {:?}", &channel);
        if let Ok(mut consumer) = self
            .channel
            .basic_consume(self.queue_name.as_str(), "*", BasicConsumeOptions::default(), FieldTable::default())
            .await
        {
            while let Some(delivery) = consumer.next().await {
                if let Ok(delivery) = delivery {
                    log::info!("incoming message: {:?}", delivery.data);
                    // log::info!("incoming message: {:?}", delivery.data);
                    if let Ok(inforeq) = YsHqInfo::decode(&*delivery.data) {
                        let _ = self.qsender.send(inforeq.to_owned()).await;
                    }

                    // let _ = channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default()).await;
                    delivery.ack(BasicAckOptions::default()).await.expect("ack");
                }
            }
        } else {
            log::error!("basic_consume error。。。")
        }
        Ok(())
    }
}
