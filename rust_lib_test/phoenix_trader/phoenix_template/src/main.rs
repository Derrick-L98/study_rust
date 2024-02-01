mod config;
mod controller;
mod protofiles;
mod server;
mod service;

use crate::config::settings::Settings;
use anyhow::Result;

use common::{akaclient::AkaClient, logclient::*};
use server::ServerHandler;
use utility::loggings;

use protofiles::phoenixriskcenter::phoenix_riskcenter_server::PhoenixRiskcenterServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  loggings::log_init();
  let settings = Settings::new().expect("init configurtion error");
  log::info!("初始化配置信息:{:?}", &settings);

  // 1. 日志中心客户端初始化
  let logclient = LogClient::init(&settings.system.logserver, "phoenix_template").await.expect("init logclient error");
  LOGCLIENT.set(logclient).expect("set global logclient error");
  //2. 写入日志的办法
  LogClient::get().push_error("error message is founded......").await;

  let server = prepare(&settings).await.expect("Init server error......");

  // log::info!("开始启动系统...");
  server_run(server, &settings).await
}

async fn prepare(settings: &Settings) -> anyhow::Result<ServerHandler> {
  // let grpc_stub = create_controller(settings).await;

  let grpc = ServerHandler::new(&settings).await;

  Ok(grpc)
}

async fn server_run(mut svr: ServerHandler, settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
  // let addr = "0.0.0.0:60000".parse().unwrap();
  let addr = format!("{}:{}", settings.application.apphost, settings.application.appport).parse().unwrap();
  log::info!("Starting  service on {}", addr);

  // 2. 获取客户端, 实际业务中不要使用unwrap
  LogClient::get().push(LogLevel::Info, "server is runnig".into()).await;
  //lc.push(LogLevel::Error, "server had halted".into()).await;

  // b. aka客户端初化 可以省略, 不指定时使用默认地址ls
  let ac = AkaClient::init(settings.system.akaserver.clone(), true, settings.system.cachelong as i64).await;

  // c. 调用aka接口
  // AccountInfoReq的定义在common下的protofiles中,使用时需要引入相关包
  // use common::{akaclient::akacenter::AccountInfoReq, akaclient::akacenter::AccountInfoResp};
  let accounts = ac.query_channel_info(123).await;
  if accounts.as_ref().is_err() {
    log::error!("查询账户信息出错");
  }
  let acc = accounts.unwrap();
  log::debug!("{}", serde_json::to_string(&acc).unwrap());

  //receive ctrl-c exit signal
  let (tx, rx) = tokio::sync::oneshot::channel::<()>();
  let on_leave = svr.on_leave();
  tokio::spawn(async move {
    tokio::signal::ctrl_c().await.ok();
    log::info!("Ctrl-c received, shutting down");
    tx.send(()).ok();
  });

  tonic::transport::Server::builder()
    .add_service(PhoenixRiskcenterServer::new(svr))
    .serve_with_shutdown(addr, async {
      rx.await.ok();
    })
    .await
    .expect("build server error");

  log::info!("Shutted down, wait for final clear");
  on_leave.leave().await;
  log::info!("Shutted down");
  Ok(())
}
