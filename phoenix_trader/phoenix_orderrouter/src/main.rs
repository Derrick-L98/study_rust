#[macro_use]
extern crate anyhow;
extern crate chrono;
extern crate lazy_static;

mod config;
mod protofiles;
mod server;
use crate::config::settings::Settings;
use crate::protofiles::order_router_service_server::OrderRouterServiceServer;
use crate::server::server::RouterServerHandler;
use anyhow::Result;
// use common::logclient::*;
use utility::loggings;

#[tokio::main]
async fn main() -> Result<()> {
  // 1. 日志中心客户端初始化
  // LogClient::init("http://139.217.232.110:50090".into(), "phoenix_template".to_string());
  loggings::log_init();
  let settings = Settings::new().expect("init setting error");
  log::info!("初始化配置信息:{:#?}", &settings);

  let server = RouterServerHandler::new(&settings).await;

  run_server(server).await.expect("server run error");

  Ok(())
}

async fn run_server(mut server: RouterServerHandler) -> Result<(), Box<dyn std::error::Error>> {
  let app_url = format!("{}:{}", server.settings.application.apphost, server.settings.application.appport);
  let addr = app_url.as_str().parse().expect("parse ip address error");

  log::info!("Starting order_router service on: {}", addr);

  let (tx, rx) = tokio::sync::oneshot::channel::<()>();
  let on_leave = server.on_leave();

  tokio::spawn(async move {
    tokio::signal::ctrl_c().await.ok();
    log::info!("Ctrl-c received, shutting down");
    tx.send(()).ok();
  });

  tonic::transport::Server::builder() //创建可以配置[server]的新服务器生成器。
    .add_service(OrderRouterServiceServer::new(server))
    .serve_with_shutdown(addr, async {
      //使用该服务器创建一个未来，在tokio的服务器上执行该服务器,并在接收到所提供的信号时关闭。
      rx.await.ok();
    })
    .await?;

  log::info!("Shutted down, wait for final clear");
  on_leave.leave().await;
  log::info!("Shutted down");
  Ok(())
}
