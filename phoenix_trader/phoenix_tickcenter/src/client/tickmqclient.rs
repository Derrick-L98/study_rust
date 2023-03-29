// use std::time::Instant;

use anyhow::{anyhow, Result};
use lapin::{options::*, BasicProperties, Channel, Connection, ConnectionProperties};

use crate::protofiles::{hqmsg::HqMsgReq, YsHqInfo};
use prost::Message;

#[derive(Debug, Clone)]
pub struct TickMqClient {
    pub channel: Channel,
    // queue_name: String, /*queue name */
    exchanger: String,
    exchanger_delay: String,
}

impl TickMqClient {
    pub async fn new(exchanger: &str, exchanger_delay: &str, addr: &str) -> Self {
        let connection = Connection::connect(addr, ConnectionProperties::default()).await.unwrap();
        let channel = connection.create_channel().await.unwrap();
        let client = TickMqClient {
            channel,
            exchanger: exchanger.to_string(),
            exchanger_delay: exchanger_delay.to_string(),
        };
        // log::info!("tick mq client inited successfully...");

        client
    }

    pub async fn publish(&self, markettype: i32, hqinfo: &YsHqInfo) -> Result<()> {
        // log::info!("{} {}", &hqinfo.contract_no1, &hqinfo.tapidtstamp);
        let mut payload = Vec::new();
        // log::info!("start to publish to mq.............");
        // let now = Instant::now();

        //"stock.3.002620_XSHE",
        let routingkey = format!("stock.{}.{}", markettype, hqinfo.contract_no1);
        let hqmsgreq = HqMsgReq {
            ctphqinfo: None,
            yshqinfo: Some(hqinfo.to_owned()),
        };
        let ret = hqmsgreq.encode(&mut payload);
        if ret.as_ref().is_err() {
            return Err(anyhow!("encode error:{:?}", ret.as_ref().err().unwrap()));
        }
        let confirm = self
            .channel
            .basic_publish(self.exchanger.as_str(), routingkey.as_str(), BasicPublishOptions::default(), &payload, BasicProperties::default())
            .await;
        if confirm.as_ref().is_err() {
            log::error!("basic publish error:{:?}", confirm.as_ref().err().unwrap());
            return Err(anyhow!("basic publish error:{:?}", confirm.as_ref().err().unwrap()));
        }
        let confirm = confirm.unwrap().await;
        if confirm.as_ref().is_err() {
            log::error!("basic publish cofnirm error:{:?}", confirm.as_ref().err().unwrap());
            return Err(anyhow!("basic publish confirm error:{:?}", confirm.as_ref().err().unwrap()));
        }
        let confirm = confirm.unwrap();
        if confirm.is_nack() {
            log::error!("not confirmed");
            return Err(anyhow!("not confirmed"));
        }

        if hqinfo.contract_no1.contains("XSHG") || hqinfo.contract_no1.contains("XSHE") || hqinfo.contract_no1.contains("HS") {
            let confirm = self
            .channel
            .basic_publish(self.exchanger_delay.as_str(), routingkey.as_str(), BasicPublishOptions::default(), &payload, BasicProperties::default())
            .await;

            if confirm.as_ref().is_err() {
                log::error!("basic publish error:{:?}", confirm.as_ref().err().unwrap());
                return Err(anyhow!("basic publish error:{:?}", confirm.as_ref().err().unwrap()));
            }
            let confirm = confirm.unwrap().await;
            if confirm.as_ref().is_err() {
                log::error!("basic publish cofnirm error:{:?}", confirm.as_ref().err().unwrap());
                return Err(anyhow!("basic publish confirm error:{:?}", confirm.as_ref().err().unwrap()));
            }
            let confirm = confirm.unwrap();
            if confirm.is_nack() {
                log::error!("not confirmed");
                return Err(anyhow!("not confirmed"));
            }
        }
        
        // log::info!("发送到mq完成,用时: {:?}", now.elapsed());
        Ok(())
    }
}
