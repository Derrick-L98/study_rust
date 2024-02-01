use crate::protofiles::svr_post_subscribe_hq_msg_client::SvrPostSubscribeHqMsgClient;
use crate::protofiles::{LastPriceMsgReq,LastPriceMsgResp
};
use anyhow::{anyhow, Result};

use tonic::transport::Channel;
#[derive(Clone)]
pub struct HqCenterClient {
    pub client: SvrPostSubscribeHqMsgClient<Channel>,
    pub url: String,
}
impl HqCenterClient {
    pub async fn new(url: &String) -> Self {
        let client = SvrPostSubscribeHqMsgClient::connect(url.to_owned()).await.expect("行情数据服务连接失败......");
        HqCenterClient { client, url: url.to_owned() }
    }
    pub async fn get_last_price(&self, req: &LastPriceMsgReq) -> Result<LastPriceMsgResp> {
        let mut client = self.client.clone();
        match client.get_last_price(req.to_owned()).await {
            Ok(val) => Ok(val.into_inner()),
            Err(status) => {
                log::error!("hqcenter server status: {:?}", status);
                log::info!("try connect hqcenter server !");
                let ret = SvrPostSubscribeHqMsgClient::connect(self.url.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connectb hqcenter server  err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.get_last_price(req.to_owned()).await {
                    return Ok(val.into_inner());
                }
                return Err(anyhow!("hqcenter server status: {:?}", status));
            }
        }
    }

}