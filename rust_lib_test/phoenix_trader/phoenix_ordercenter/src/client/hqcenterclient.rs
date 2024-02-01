use anyhow::Result;
use tonic::{transport::Channel, codegen::http::request};
use crate::protofiles::{
    LastPriceMsgReq, LastPriceInfo,
    svr_post_subscribe_hq_msg_client::SvrPostSubscribeHqMsgClient,
};

#[derive(Clone)]
pub struct HqCenterClient {
    pub client: SvrPostSubscribeHqMsgClient<Channel>,
    uri: String,
}

impl HqCenterClient {
    pub async fn new(uri: &String) -> Self {
        let client = SvrPostSubscribeHqMsgClient::connect(uri.to_owned()).await.expect("行情中心连接失败...");
        HqCenterClient {
            client,
            uri: uri.to_owned(),
        }
    }

    pub async fn get_last_price(&self, stock_code: &String, exchange_id: i32) -> Result<LastPriceInfo>{
        let request = LastPriceMsgReq {
            stock_code: stock_code.to_owned(),
            exchange_id,
        };
        let mut client = self.client.clone();
        match client.get_last_price(request.to_owned()).await {
            Ok(val) => {
                if let Some(price_info) = val.into_inner().data {
                    Ok(price_info)
                } else {
                    return Err(anyhow!("未找到{}最新价信息", &stock_code));
                }
            },
            Err(status) => {
                log::error!("Hq center status: {:?}", status);
                log::info!("try connect Hq center!");
                let ret = SvrPostSubscribeHqMsgClient::connect(self.uri.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connect Hq center err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.get_last_price(request).await {
                    if let Some(price_info) = val.into_inner().data {
                        return Ok(price_info)
                    } else {
                        return Err(anyhow!("未找到{}最新价信息", &stock_code));
                    }
                }
                return Err(anyhow!("Hq center status: {:?}", status));
            }
        }
    }

    pub async fn get_conn(&self) -> SvrPostSubscribeHqMsgClient<Channel>{
        self.client.clone()
    }
}
