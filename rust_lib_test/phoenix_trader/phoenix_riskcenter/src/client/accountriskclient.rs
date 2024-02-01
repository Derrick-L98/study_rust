use crate::protofiles::account_risk_center_client::AccountRiskCenterClient;
use crate::protofiles::{MarginRatioReq,MarginRatioResp,
    UserAssetReq,UserAssetResp,
    PhoenixStockPositionRequest,PhoenixStockPositionResponse
};
use anyhow::{anyhow, Result};

use tonic::transport::Channel;
#[derive(Clone)]
pub struct AccountRiskClient {
    pub client: AccountRiskCenterClient<Channel>,
    pub url: String,
}
impl AccountRiskClient {
    pub async fn new(url: &String) -> Self {
        let client = AccountRiskCenterClient::connect(url.to_owned()).await.expect("基础数据服务连接失败......");
        AccountRiskClient { client, url: url.to_owned() }
    }
    pub async fn query_margin_ratio(&self, req: &MarginRatioReq) -> Result<MarginRatioResp> {
        let mut client = self.client.clone();
        match client.query_margin_ratio(req.to_owned()).await {
            Ok(val) => Ok(val.into_inner()),
            Err(status) => {
                log::error!("basicdata server status: {:?}", status);
                log::info!("try connect basicdata server !");
                let ret = AccountRiskCenterClient::connect(self.url.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connectb basicdata server  err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.query_margin_ratio(req.to_owned()).await {
                    return Ok(val.into_inner());
                }
                return Err(anyhow!("basicdata server status: {:?}", status));
            }
        }
    }
    pub async fn query_user_asset(&self, req: &UserAssetReq) -> Result<UserAssetResp> {
        let mut client = self.client.clone();
        match client.query_user_asset(req.to_owned()).await {
            Ok(val) => Ok(val.into_inner()),
            Err(status) => {
                log::error!("basicdata server status: {:?}", status);
                log::info!("try connect basicdata server !");
                let ret = AccountRiskCenterClient::connect(self.url.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connectb basicdata server  err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.query_user_asset(req.to_owned()).await {
                    return Ok(val.into_inner());
                }
                return Err(anyhow!("basicdata server status: {:?}", status));
            }
        }
    }
    pub async fn query_stock_positions(&self, req: &PhoenixStockPositionRequest) -> Result<PhoenixStockPositionResponse> {
        let mut client = self.client.clone();
        match client.query_stock_positions(req.to_owned()).await {
            Ok(val) => Ok(val.into_inner()),
            Err(status) => {
                log::error!("basicdata server status: {:?}", status);
                log::info!("try connect basicdata server !");
                let ret = AccountRiskCenterClient::connect(self.url.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connectb basicdata server  err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.query_stock_positions(req.to_owned()).await {
                    return Ok(val.into_inner());
                }
                return Err(anyhow!("basicdata server status: {:?}", status));
            }
        }
    }

}