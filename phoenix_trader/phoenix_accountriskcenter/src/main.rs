mod app;
mod config;
mod dataservice;
mod dataview;
mod protofiles;
mod server;
mod webserver;

use std::{net::SocketAddr, sync::Arc};

use common::logclient::{LogClient, LOGCLIENT};
use server::server::ServerHandler;
use utility::loggings;

use crate::{config::settings::Settings, dataservice::dbsetup::DbConnection, protofiles::phoenixaccountriskcenter::account_risk_center_server::AccountRiskCenterServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  loggings::log_init();

  let settings: Settings = Settings::new().expect("init configurtion error");
  log::info!("初始化配置信息:{:?}", &settings);

  let logclient = LogClient::init(&settings.system.log_server, "phoenix_accountriskcenter").await.expect("init log client error");
  LOGCLIENT.set(logclient).expect("set log clint error");

  let dbconn = DbConnection::new(&settings.database.uri.as_str()).await;

  let server = prepare(&settings, &dbconn).await.expect("Init server error......");
  // log::info!("开始启动系统...");
  server_run(server, &settings, &dbconn).await
}

async fn prepare(settings: &Settings, dbconn: &DbConnection) -> anyhow::Result<ServerHandler> {
  let grpc = ServerHandler::new(&settings, &dbconn).await;

  Ok(grpc)
}

async fn server_run(mut svr: ServerHandler, settings: &Settings, dbconn: &DbConnection) -> Result<(), Box<dyn std::error::Error>> {
  // let addr = "0.0.0.0:60000".parse().unwrap();
  let addr = format!("{}:{}", settings.application.apphost, settings.application.appport).parse().unwrap();
  log::info!("Starting  service on {}", addr);

  //receive ctrl-c exit signal
  let (tx, rx) = tokio::sync::oneshot::channel::<()>();
  let on_leave = svr.on_leave();
  tokio::spawn(async move {
    tokio::signal::ctrl_c().await.ok();
    log::info!("Ctrl-c received, shutting down");
    tx.send(()).ok();
  });

  //增加http接口
  let http_url = format!("{}:{}", settings.application.apphost, settings.application.httpport);
  let httpaddr: SocketAddr = http_url.as_str().parse().expect("parse httpaddr error");
  let app = webserver::routers::create_route(dbconn);
  log::info!("Starting assetscenter http server on {:?}", &httpaddr);
  tokio::spawn(async move {
    axum::Server::bind(&httpaddr).serve(app.into_make_service()).await.unwrap();
  });

  tonic::transport::Server::builder()
    .add_service(AccountRiskCenterServer::new(svr))
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
