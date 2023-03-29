// use crate::server::ServerHandler;
mod config;
mod controller;
// mod dto;
mod basicdata;
mod client;
mod protofiles;
mod server;
mod service;

// use crate::config::settings::Settings;
use crate::{config::settings::Settings, protofiles::phoenixriskcenter::phoenix_riskcenter_server::PhoenixRiskcenterServer};
use anyhow::Result;
// use dto::*;
// use controller::create_controller;
use server::ServerHandler;
use utility::loggings;

// use phoenix_service::riskcenter::server::{PhoenixRiskcenterServer, ServerHandler};
// use phoenix_service::{riskcenter::server::create_controller, utility::loggings};
// use utility::settings::Settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  loggings::log_init();
  let settings = Settings::new().unwrap();

  log::info!("初始化配置信息:{:?}", &settings);
  // persist::setup::DbConnection::create_connection();

  // log::info!("{:?}", persist::dealdetail::ExDealDetail::query_by_ids().await);
  // let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
  //     .enable_all()
  //     .build()
  //     .expect("build runtime error");

  // rt.block_on(async {
  let server = prepare(&settings).await.expect("Init server error......");
  // log::info!("开始启动系统...");
  server_run(server, &settings).await
  // })
  // .unwrap();
}

async fn prepare(settings: &Settings) -> anyhow::Result<ServerHandler> {
  // let grpc_stub = create_controller(settings).await;

  let grpc = ServerHandler::new(&settings).await;

  Ok(grpc)
}

async fn server_run(mut svr: ServerHandler, settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
  // let addr = "0.0.0.0:60000".parse().unwrap();
  let addr = format!("{}:{}", settings.application.apphost, settings.application.appport).parse().unwrap();
  log::info!("Starting phoenix riskcenter service on {}", addr);

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
    .await?;

  log::info!("Shutted down, wait for final clear");
  on_leave.leave().await;
  log::info!("Shutted down");
  Ok(())
}
