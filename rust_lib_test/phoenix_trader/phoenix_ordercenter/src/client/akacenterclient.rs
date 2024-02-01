use std::process;
use anyhow::Result;
use tonic::transport::Channel;
use common::protofiles::{
    StockInfoReq, StockInfoResp, MarketInfoReq, 
    MarketInfoResp, ExchangeRateReq, ExchangeRateResp,
    phoenix_aka_center_client::PhoenixAkaCenterClient, FeeSettingReq, FeeSettingResp
};
use crate::common::common::OrderDetail;

#[derive(Clone)]
pub struct AkaCenterClient {
    pub client: PhoenixAkaCenterClient<Channel>,
    uri: String,
}

impl AkaCenterClient {
    pub async fn new(uri: &String) -> Self {
        let client = PhoenixAkaCenterClient::connect(uri.to_owned())
        .await.unwrap_or_else(|err| {
            log::error!("基础数据服务连接失败: {}", err);
            process::exit(1)
        });

        AkaCenterClient {
            client,
            uri: uri.to_owned(),
        }
    }

    //stock_id 为0查所有
    pub async fn query_stock_info(&self, stock_id: i64) -> Result<StockInfoResp>{
        let request = StockInfoReq {
            stock_id,
        };
        let mut client = self.client.clone();
        match client.query_stock_info(request.to_owned()).await {
            Ok(val) => Ok(val.into_inner()),
            Err(status) => {
                log::error!("Aka center status: {:?}", status);
                log::info!("try connect Aka center!");
                let ret = PhoenixAkaCenterClient::connect(self.uri.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connect Aka center err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.query_stock_info(request).await {
                    return Ok(val.into_inner());
                }
                return Err(anyhow!("Aka center status: {:?}", status));
            }
        }
    }

    pub async fn query_market_info(&self, exchange_id: i64) -> Result<MarketInfoResp>{
        let request = MarketInfoReq {
            market_id: exchange_id,
        };
        let mut client = self.client.clone();
        match client.query_market_info(request.to_owned()).await {
            Ok(val) => Ok(val.into_inner()),
            Err(status) => {
                log::error!("Aka center status: {:?}", status);
                log::info!("try connect Aka center!");
                let ret = PhoenixAkaCenterClient::connect(self.uri.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connect Aka center err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.query_market_info(request).await {
                    return Ok(val.into_inner());
                }
                return Err(anyhow!("Aka center status: {:?}", status));
            }
        }
    }

    pub async fn query_exchange_rate(&self, currency: i32) -> Result<ExchangeRateResp>{
        let request = ExchangeRateReq {
            currency,
        };
        let mut client = self.client.clone();
        match client.query_exchange_rate(request.to_owned()).await {
            Ok(val) => Ok(val.into_inner()),
            Err(status) => {
                log::error!("Aka center status: {:?}", status);
                log::info!("try connect Aka center!");
                let ret = PhoenixAkaCenterClient::connect(self.uri.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connect Aka center err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.query_exchange_rate(request).await {
                    return Ok(val.into_inner());
                }
                return Err(anyhow!("Aka center status: {:?}", status));
            }
        }
    }

    pub async fn query_fee_setting(&self, detail: &OrderDetail) -> Result<FeeSettingResp>{
        let request = FeeSettingReq {
            fee_type: detail.fee_type.to_owned(),
            exchange_id: detail.exchange_id,
            order_direction: detail.order_direction,
            unit_id: detail.unit_id,
            channel_id: detail.channel_id,
            stock_type: detail.stock_type,
        };
        let mut client = self.client.clone();
        match client.query_fee_setting(request.to_owned()).await {
            Ok(val) => Ok(val.into_inner()),
            Err(status) => {
                log::error!("Aka center status: {:?}", status);
                log::info!("try connect Aka center!");
                let ret = PhoenixAkaCenterClient::connect(self.uri.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connect Aka center err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.query_fee_setting(request).await {
                    return Ok(val.into_inner());
                }
                return Err(anyhow!("Aka center status: {:?}", status));
            }
        }
    }

    pub async fn get_conn(&self) -> PhoenixAkaCenterClient<Channel>{
        self.client.clone()
    }
}