#[macro_use]
extern crate anyhow;
extern crate chrono;

mod client;
mod commonutil;
mod config;
mod protofiles;
mod server;

use std::process;
use crate::config::settings::Settings;
use crate::protofiles::phoenix_tick_center_server::PhoenixTickCenterServer;
use crate::server::server::TickServerHandler;
use anyhow::Result;
use utility::loggings;

#[tokio::main]
async fn main() -> Result<()> {
    loggings::log_init();
    let settings = Settings::new().unwrap();
    log::info!("初始化配置信息:{:#?}", &settings);

    let server = TickServerHandler::new(&settings).await.unwrap_or_else(|err| {
        log::error!("{:?} 服务启动失败...", err);
        // panic!("{}", err)
        process::exit(1);
    });

    let _ = run_server(server).await.unwrap();

    Ok(())
}

async fn run_server(mut server: TickServerHandler) -> Result<(), Box<dyn std::error::Error>> {
    let app_url = format!("{}:{}", server.settings.application.apphost, server.settings.application.appport);
    let addr = app_url.as_str().parse().unwrap();

    log::info!("Starting tickceter service on: {}\n", addr);

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let on_leave = server.on_leave();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        log::info!("Ctrl-c received, shutting down");
        tx.send(()).ok();
    });

    tonic::transport::Server::builder() //创建可以配置[server]的新服务器生成器。
        .add_service(PhoenixTickCenterServer::new(server))
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
