use std::process;
use anyhow::Result;
use tonic::transport::Channel;
// use crate::common::common::OrderDetail;
use crate::protofiles::{
    OrderDealReq, OrderDealResp,
    svr_order_deal_client::SvrOrderDealClient,
};

#[derive(Clone)]
pub struct OrderDealClient {
    pub client: SvrOrderDealClient<Channel>,
    uri: String,
}

impl OrderDealClient {
    pub async fn new(uri: &String) -> Self {
        let client = SvrOrderDealClient::connect(uri.to_owned())
        .await.unwrap_or_else(|err| {
            log::error!("成交服务连接失败: {}", err);
            process::exit(1)
        });

        OrderDealClient {
            client,
            uri: uri.to_owned(),
        }
    }

    pub async fn order_deal(&self, request: &OrderDealReq) -> Result<OrderDealResp>{
        let mut client = self.client.clone();
        match client.order_deal(request.to_owned()).await {
            Ok(val) => Ok(val.into_inner()),
            Err(status) => {
                log::error!("orderdeal center status: {:?}", status);
                log::info!("try connect orderdeal center!");
                let ret = SvrOrderDealClient::connect(self.uri.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connect orderdeal center err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.order_deal(request.to_owned()).await {
                    return Ok(val.into_inner());
                }
                return Err(anyhow!("orderdeal center status: {:?}", status));
            }
        }
    }

    pub async fn get_conn(&self) -> SvrOrderDealClient<Channel>{
        self.client.clone()
    }
}