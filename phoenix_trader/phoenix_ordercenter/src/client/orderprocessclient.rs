use std::process;
use anyhow::Result;
use tonic::transport::Channel;
use crate::protofiles::{
    svr_order_process_client::SvrOrderProcessClient,
    OrderConfirmReq, CancelWasteReq, OrderCancelReq,
    OrderConfirmResp, CancelWasteResp, OrderCancelResp,
};
// use crate::common::common::OrderDetail;

#[derive(Clone)]
pub struct OrderProcessClient {
    pub client: SvrOrderProcessClient<Channel>,
    uri: String,
}

impl OrderProcessClient {
    pub async fn new(uri: &String) -> Self {
        let client = SvrOrderProcessClient::connect(uri.to_owned())
        .await.unwrap_or_else(|err| {
            log::error!("订单处理服务连接失败: {}", err);
            process::exit(1)
        });

        OrderProcessClient {
            client,
            uri: uri.to_owned(),
        }
    }

    pub async fn get_conn(&self) -> SvrOrderProcessClient<Channel>{
        self.client.clone()
    }

    pub async fn order_confirm(&self, request: &OrderConfirmReq) -> Result<OrderConfirmResp>{
        let mut client = self.client.clone();
        match client.order_confirm(request.to_owned()).await {
            Ok(val) => Ok(val.into_inner()),
            Err(status) => {
                log::error!("SvrOrderProcess center status: {:?}", status);
                log::info!("try connect SvrOrderProcess center!");
                let ret = SvrOrderProcessClient::connect(self.uri.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connect SvrOrderProcess center err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.order_confirm(request.to_owned()).await {
                    return Ok(val.into_inner());
                }
                return Err(anyhow!("SvrOrderProcess center status: {:?}", status));
            }
        }
    }

    pub async fn order_cancel(&self, request: &OrderCancelReq) -> Result<OrderCancelResp>{
        let mut client = self.client.clone();
        match client.order_cancel(request.to_owned()).await {
            Ok(val) => Ok(val.into_inner()),
            Err(status) => {
                log::error!("SvrOrderProcess center status: {:?}", status);
                log::info!("try connect SvrOrderProcess center!");
                let ret = SvrOrderProcessClient::connect(self.uri.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connect SvrOrderProcess center err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.order_cancel(request.to_owned()).await {
                    return Ok(val.into_inner());
                }
                return Err(anyhow!("SvrOrderProcess center status: {:?}", status));
            }
        }
    }

    pub async fn cancel_waste(&self, request: &CancelWasteReq) -> Result<CancelWasteResp>{
        let mut client = self.client.clone();
        match client.cancel_waste(request.to_owned()).await {
            Ok(val) => Ok(val.into_inner()),
            Err(status) => {
                log::error!("SvrOrderProcess center status: {:?}", status);
                log::info!("try connect SvrOrderProcess center!");
                let ret = SvrOrderProcessClient::connect(self.uri.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connect SvrOrderProcess center err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.cancel_waste(request.to_owned()).await {
                    return Ok(val.into_inner());
                }
                return Err(anyhow!("SvrOrderProcess center status: {:?}", status));
            }
        }
    }
}