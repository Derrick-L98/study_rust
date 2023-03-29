
use anyhow::Result;
use std::process;
use common::constant::{AssetChangeType, AssetChangeDirection};
use tonic::{transport::Channel, codegen::http::request};
use crate::protofiles::{
    PhoenixassetscenterRequest,
    PhoenixassetscenterResponse,
    PhoenixassetscenterQueryRequest,
    // PhoenixassetscenterRequestInfo,
    // PhoenixassetspostionrequestInfo,
    phoenixassetscenter_client::PhoenixassetscenterClient
};
// use crate::common::common::OrderDetail;

#[derive(Clone)]
pub struct AssetsCenterClient {
    pub client: PhoenixassetscenterClient<Channel>,
    uri: String,
}


impl AssetsCenterClient {
    pub async fn new(uri: &String) -> Self {
        let client = PhoenixassetscenterClient::connect(uri.to_owned())
        .await.unwrap_or_else( |err| {
            log::error!("资产中心连接失败PhoenixassetscenterClient: {}", err);
            process::exit(1)
        });
        AssetsCenterClient {
            client,
            uri: uri.to_owned(),
        }
    }

    pub async fn phoenix_assets_change(&self, request: &PhoenixassetscenterRequest) -> Result<PhoenixassetscenterResponse>{
        let mut client = self.client.clone();
        match client.phoenix_assets_change(request.to_owned()).await {
            Ok(val) => Ok(val.into_inner()),
            Err(status) => {
                log::error!("phoenix_assets_change: {:?}", status);
                log::info!("try connect asset center!");
                let ret = PhoenixassetscenterClient::connect(self.uri.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connect asset center err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.phoenix_assets_change(request.to_owned()).await {
                    return Ok(val.into_inner());
                }
                return Err(anyhow!("asset center error"));
            }
        }
    }


    pub async fn phoenix_assets_query(&self, request: &PhoenixassetscenterQueryRequest) -> Result<PhoenixassetscenterResponse>{
        let mut client = self.client.clone();
        match client.phoenix_assets_query(request.to_owned()).await {
            Ok(val) => Ok(val.into_inner()),
            Err(status) => {
                log::error!("phoenix_assets_query: {:?}", status);
                log::info!("try connect asset center!");
                let ret = PhoenixassetscenterClient::connect(self.uri.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connect asset center err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.phoenix_assets_query(request.to_owned()).await {
                    return Ok(val.into_inner());
                }
                return Err(anyhow!("asset center error"));
            }
        }
    }

    pub async fn get_conn(&self) -> PhoenixassetscenterClient<Channel>{
        self.client.clone()
    }

}