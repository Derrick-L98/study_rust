use std::sync::Arc;

use crate::config::settings::Settings;
use crate::protofiles::phoenixriskcenter::{PhoenixRiskCheckRequest, PhoenixRiskCheckResponse, PhoenixRiskRequest, PhoenixRiskResponse};
use anyhow::Result;
use common::uidservice::UidgenService;
use tokio::sync::RwLock;
// use rust_decimal::{prelude::*, Decimal};
use tonic::{self};
// use utility::{constant, errors, errors::ErrorCode};

// #[derive(Clone)]
pub struct ServerController {
  pub settings: Arc<RwLock<Settings>>,
  pub uidsvc: Arc<RwLock<UidgenService>>,
}

//处理业务逻辑
impl ServerController {
  pub async fn update_configurations(&self, settings: &Settings) -> Result<()> {
    let mut wr = self.settings.write().await;
    *wr = settings.to_owned();
    Ok(())
  }

  pub async fn print_configurations(&self) -> Result<()> {
    log::info!("new configurations:{:#?}", &self.settings.read().await);
    Ok(())
  }

  pub async fn phoenix_risk_check(&self, req: &PhoenixRiskCheckRequest) -> Result<PhoenixRiskCheckResponse> {
    let result = PhoenixRiskCheckResponse {
      ret_code: "1111".to_string(),
      ret_msg: String::from("aaa"),
      retinfo: vec![],
    };
    log::info!("收请求");
    std::thread::sleep(std::time::Duration::from_secs(2));
    Ok(result)
  }

  pub async fn phoenix_risk_test(&self, req: &PhoenixRiskRequest) -> Result<PhoenixRiskResponse> {
    let result = PhoenixRiskResponse {
      ret_code: 1111,
      ret_msg: String::from("aaa"),
    };
    log::info!("收到请求");
    std::thread::sleep(std::time::Duration::from_secs(2));
    Ok(result)
  }
}
