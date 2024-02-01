#[macro_use]
extern crate anyhow;
extern crate chrono;

mod client;
mod config;
mod protofiles;
mod server;
use crate::config::settings::Settings;
use crate::protofiles::market_data_servers_server::MarketDataServersServer;
use crate::server::server::StreamServerHandler;
use anyhow::Result;
use utility::loggings;

#[tokio::main]
async fn main() -> Result<()> {
    loggings::log_init();
    let settings = Settings::new().unwrap();
    log::info!("初始化配置信息:{:#?}", &settings);

    let server = StreamServerHandler::new(&settings).await;

    let _ = run_server(server).await.unwrap();

    Ok(())
}

async fn run_server(mut server: StreamServerHandler) -> Result<(), Box<dyn std::error::Error>> {
    let app_url = format!("{}:{}", server.settings.application.apphost, server.settings.application.appport);
    let addr = app_url.as_str().parse().unwrap();

    log::info!("Starting streamcenter service on: {}", addr);

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let on_leave = server.on_leave();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        log::info!("Ctrl-c received, shutting down");
        tx.send(()).ok();
    });

    tonic::transport::Server::builder() //创建可以配置[server]的新服务器生成器。
        .add_service(MarketDataServersServer::new(server))
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
