
use anyhow::Result;
use std::process;
use tonic::transport::Channel;
use crate::protofiles::{
    MarginRatioReq,
    MarginRatioResp,
    // PhoenixassetscenterRequestInfo,
    // PhoenixassetspostionrequestInfo,
    account_risk_center_client::AccountRiskCenterClient
};
// use crate::common::common::OrderDetail;

#[derive(Clone)]
pub struct AccountRiskClient {
    pub client: AccountRiskCenterClient<Channel>,
    uri: String,
}


impl AccountRiskClient {
    pub async fn new(uri: &String) -> Self {
        let client = AccountRiskCenterClient::connect(uri.to_owned())
        .await.unwrap_or_else( |err| {
            log::error!("分帐户分控中心连接失败AccountRiskCenterClient: {}", err);
            process::exit(1)
        });
        AccountRiskClient {
            client,
            uri: uri.to_owned(),
        }
    }

    pub async fn query_margin_ratio(&self, unit_id: i64, stock_id: i64) -> Result<MarginRatioResp>{
        let mut client = self.client.clone();
        let request = MarginRatioReq { unit_id, stock_id };
        match client.query_margin_ratio(request.to_owned()).await {
            Ok(val) => Ok(val.into_inner()),
            Err(status) => {
                log::error!("AccountRisk center status: {:?}", status);
                log::info!("try connect AccountRisk center!");
                let ret = AccountRiskCenterClient::connect(self.uri.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connect AccountRisk center err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.query_margin_ratio(request.to_owned()).await {
                    return Ok(val.into_inner());
                }
                return Err(anyhow!("AccountRisk center error"));
            }
        }
    }

    pub async fn get_conn(&self) -> AccountRiskCenterClient<Channel>{
        self.client.clone()
    }

}