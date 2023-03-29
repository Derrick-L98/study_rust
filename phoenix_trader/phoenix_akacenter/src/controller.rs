use crate::config::settings::Settings;
use crate::protofiles::phoenixriskcenter::{PhoenixRiskCheckRequest, PhoenixRiskCheckResponse, PhoenixRiskRequest, PhoenixRiskResponse};
use anyhow::Result;
// use rust_decimal::{prelude::*, Decimal};
use tonic::{self, Status};
// use utility::{constant, errors, errors::ErrorCode};

// #[derive(Clone)]
pub struct ServerController {
    pub settings: Settings,
}

//处理业务逻辑
impl ServerController {
    pub async fn phoenix_risk_check(&mut self, _req: PhoenixRiskCheckRequest) -> Result<PhoenixRiskCheckResponse, Status> {
        let result = PhoenixRiskCheckResponse {
            ret_code: "1111".to_string(),
            ret_msg: String::from("aaa"),
            retinfo: vec![],
        };
        log::info!("收请求");
        std::thread::sleep(std::time::Duration::from_secs(2));
        Ok(result)
    }

    pub async fn phoenix_risk_test(&mut self, _req: PhoenixRiskRequest) -> Result<PhoenixRiskResponse, Status> {
        let result = PhoenixRiskResponse {
            ret_code: 1111,
            ret_msg: String::from("aaa"),
        };
        log::info!("收到请求");
        std::thread::sleep(std::time::Duration::from_secs(2));
        Ok(result)
    }
}
