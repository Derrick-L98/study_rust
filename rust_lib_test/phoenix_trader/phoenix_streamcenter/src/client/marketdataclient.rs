use anyhow::Result;

use tokio::sync::broadcast::Sender;

use tokio_stream::StreamExt;
use tonic::transport::Channel;
use common::logclient::LogClient;
use crate::protofiles::{hqmsg::YsHqInfo, market_data_servers_client::MarketDataServersClient, ContractMsg};

#[derive(Clone)]
pub struct MarketDataclient {
    exchangeno: Vec<String>,
    pub client: MarketDataServersClient<Channel>,
    tx_tick: Sender<YsHqInfo>,
    uri: String,
}

impl MarketDataclient {
    pub async fn new(uri: &String, exchangenos: &String, tx_tick: Sender<YsHqInfo>) -> Self {
        log::info!("连接MarketDataServers服务: {}", &uri);
        let ret = MarketDataServersClient::connect(uri.to_owned()).await;//.expect("行情服务连接失败");
        if ret.as_ref().is_err() {
            LogClient::get().push_error(format!("MarketDataServers连接失败: {:?}", uri).as_str()).await;
        }
        let client = ret.expect("MarketDataServers服务连接失败");
        let exchangeno: Vec<String> = exchangenos.split(',').map(|x| x.to_string()).collect();
        log::info!("行情服务器连接成功...");
        MarketDataclient { client, exchangeno, tx_tick, uri: uri.to_string() }
    }

    pub async fn do_subscribe_market_data(&mut self) -> Result<()> {
        let data = tokio_stream::iter(1..usize::MAX).map(move |_i| ContractMsg {
            exchange_no: "ALL".to_owned(),
            commodity_no: "ALL".to_string(),
            contract_no: "ALL".to_string(),
            channel_no: 0,
            subscribe: true,
        });
        let in_stream = data.take(10); //创建一个新流，该流最多包含n个基础流项。

        let sub_ret = self.client.subscribe_market_data(in_stream).await;
        if sub_ret.as_ref().is_err() {
            log::error!("subscribe error:{:?}", sub_ret.as_ref().err());
            return Err(anyhow!("subscribe error"));
        }
        let response = sub_ret.unwrap();

        let mut inbound = response.into_inner();
        while let Ok(inbound_data) = inbound.message().await {
            if inbound_data.is_some() {
                let hqinfo = inbound_data.unwrap();
                // log::info!("received hqinfo: {:?}", &hqinfo);
                if self.exchangeno.iter().find(|&x| x == &hqinfo.exchange_id).is_some() {
                    if let Err(e) = self.tx_tick.send(hqinfo.to_owned()) {
                        log::error!("send to channel error...........:{:?}", &e);
                    }
                }
            } else {
                log::error!("inbound data empty")
            }
        }
        Ok(())
    }

    pub async fn retry_do_subscribe_market_data(&mut self) -> Result<()>{
        let client = MarketDataServersClient::connect(self.uri.to_owned()).await;//.expect("行情服务连接失败");
        if client.as_ref().is_err() {
            log::error!("{:?}", client);
            return Err(anyhow!("{:?}", client));
        }
        self.client = client.unwrap();
        let get_ret = self.do_subscribe_market_data().await;
        if get_ret.as_ref().is_err() {
            log::error!("can't subscribe market data......{:?}", get_ret.as_ref().err().unwrap());
            return Err(anyhow!("{:?}", get_ret));
        }
        Ok(())
    }
    // pub async fn get_client(&self) -> MarketDataServersClient<Channel> {
    //     self.client.clone()
    // }
}
