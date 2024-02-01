use std::process;
use anyhow::Result;
use tokio::sync::broadcast::Sender;
use tonic::transport::Channel;
// use tokio_stream::StreamExt;
// use tokio::sync::mpsc::{Sender, Receiver};

use crate::protofiles::{
    RouterMsg, order_router_service_client::OrderRouterServiceClient,
};


#[derive(Clone)]
pub struct OrderRouterClient {
    pub client: OrderRouterServiceClient<Channel>,
    tx_order: Sender<RouterMsg>, //收订单消息 -> 报盘
    tx_repay: Sender<RouterMsg>, //发回报消息 -> 处理
    uri: String,
}

impl OrderRouterClient {
    pub async fn new(uri: &String, tx_order: Sender<RouterMsg>, tx_repay: Sender<RouterMsg>) -> Self {
        let client = OrderRouterServiceClient::connect(uri.to_owned())
        .await.unwrap_or_else( |err|{
            log::error!("路由中心连接失败OrderRouterServiceClient: {}", err);
            process::exit(1)
        });
        OrderRouterClient {
            client,
            tx_order,
            tx_repay,
            uri: uri.to_owned(),
        }
    }

    pub async fn order_routing(&mut self) -> Result<()> {
        let mut rx = self.tx_order.subscribe();
        //报单消息
        let outbound = async_stream::stream! {
            loop {
                if let Ok(val) = rx.recv().await {
                    log::info!("推送到报盘: {:?}", &val);
                    yield val;
                }
            }
        };

        let sub_ret = self.client.order_routing(outbound).await;
        if sub_ret.as_ref().is_err() {
            log::error!("router error:{:?}", sub_ret.as_ref().err());
            //订单到这里失败了怎么办 ?
            return Err(anyhow!("order router error"));
        }
        let response = sub_ret.unwrap();

        let mut inbound = response.into_inner();
        while let Ok(inbound_data) = inbound.message().await {
            if inbound_data.is_some() {
                //回报消息
                let value = inbound_data.unwrap();
                // log::info!("{:#?}", value);
                if let Err(err) = self.tx_repay.send(value) {
                    //订单到这里失败了怎么办 ?
                    log::error!("订单回报发送失败: {:?}", err);
                }
            } else {
                log::info!("inbound data empty");
            }
        }
        Ok(())
    }

    pub async fn retry_order_routing(&mut self) -> Result<()>{
        let client = OrderRouterServiceClient::connect(self.uri.to_owned()).await;
        if client.as_ref().is_err() {
            log::error!("{:?}", client);
            return Err(anyhow!("{:?}", client));
        }
        self.client = client.unwrap();
        let get_ret = self.order_routing().await;
        if get_ret.as_ref().is_err() {
            log::error!("{:?}", get_ret.as_ref().err().unwrap());
            return Err(anyhow!("{:?}", get_ret.as_ref().err().unwrap()));
        }
        Ok(())
    }
}