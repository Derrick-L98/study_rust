mod config;
mod controller;
mod protofiles;
mod server;
mod service;

use crate::config::settings::Settings;
use anyhow::Result;
use server::ServerHandler;
use utility::loggings;

use protofiles::phoenixriskcenter::phoenix_riskcenter_server::PhoenixRiskcenterServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loggings::log_init();
    let settings = Settings::new().expect("init configuration error");

    log::info!("初始化配置信息:{:?}", &settings);

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
