use anyhow::Result;
use rust_decimal::prelude::ToPrimitive;
use std::process;
use tonic::transport::Channel;
use crate::common::common::OrderDetail;
use crate::protofiles::{
    PhoenixRiskCheckInfo,
    PhoenixRiskCheckRequest,
    PhoenixRiskCheckResponse,
    phoenix_riskcenter_client::PhoenixRiskcenterClient,
};

#[derive(Clone)]
pub struct RiskCenterClient {
    pub client: PhoenixRiskcenterClient<Channel>,
    pub uri: String,
}

impl RiskCenterClient {
    pub async fn new(uri: &String) -> Self {
        let client = PhoenixRiskcenterClient::connect(uri.to_owned())
        .await.unwrap_or_else( |err| {
            log::error!("风控中心连接失败PhoenixRiskcenterClient: {}", err);
            process::exit(1)
        });
        RiskCenterClient {
            client,
            uri: uri.to_owned(),
        }
    }

    pub async fn phoenix_risk_check(&self, order_detail: &OrderDetail) -> Result<PhoenixRiskCheckResponse>{
        let request = RiskCenterClient::convert_to_riskinfo(order_detail).await;
        let mut client = self.client.clone();
        match client.phoenix_risk_check(request.to_owned()).await {
            Ok(val) => {
                Ok(val.into_inner())
            },
            Err(status) => {
                log::error!("risk center status: {:?}", status);
                log::info!("try connect risk center!");
                let ret = PhoenixRiskcenterClient::connect(self.uri.to_owned()).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!("try connect risk center err: {:?}", ret.as_ref().err().unwrap().to_string()));
                }
                let mut client = ret.unwrap();
                if let Ok(val) = client.phoenix_risk_check(request).await {
                    return Ok(val.into_inner());
                }
                return Err(anyhow!("risk center error"));
            }
        }
    }

    pub async fn convert_to_riskinfo(order_detail: &OrderDetail) -> PhoenixRiskCheckRequest {
        let mut req = PhoenixRiskCheckRequest::default();
        let mut queryinfo = PhoenixRiskCheckInfo::default();
        queryinfo.unit_id = order_detail.unit_id;//用户unit_id, 
        queryinfo.stock_id = order_detail.stock_id;//股票ID, 
        queryinfo.order_price = order_detail.order_price.to_f64().unwrap_or_default();//价格, 
        queryinfo.order_amount = order_detail.order_amount;//数量，
        queryinfo.order_direction = order_detail.order_direction;//方向（ 1:买 2：卖）
        // queryinfo.channel_type = 6;//通道类型：1：外盘 2：内盘
        queryinfo.order_type = order_detail.order_type;//委托类型 1:app下单  2:跟单  3:风控止盈止损平仓单,4:风控总资产预警平仓单 5:pc客户端单 6:结算平仓单 7:管理端强平仓单,8:app清仓,9:pc清仓,10,管理员平仓,11,合约到期日强平
        // queryinfo.order_type  = 8;//通道
        queryinfo.market_id = order_detail.exchange_id as i64;//市场ID
        queryinfo.trade_mode = order_detail.trade_mode;        // 1:USER(用户直连) 2:AGENT(代理托管)
        queryinfo.agent_account = order_detail.agent_account;  // 代理账户

        req.queryinfo = Some(queryinfo);
        log::info!("request risk msg: {:#?}", &req);
        req
    }

    pub async fn get_conn(&self) -> PhoenixRiskcenterClient<Channel>{
        self.client.clone()
    }
}