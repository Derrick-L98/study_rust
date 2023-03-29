#[macro_use]
extern crate anyhow;
extern crate chrono;

mod common;
mod server;
mod config;
mod client;
mod protofiles;
mod dataservice;

use std::process;
use anyhow::Result;
use utility::loggings;
use crate::config::settings::Settings;
use crate::server::server::OrderServerHandler;
use crate::protofiles::order_center_service_server::OrderCenterServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loggings::log_init();
    // Display: rgb(0,255,0)
    // rgb(111,11,1)
    // rgb(1,1,255)
    let settings = Settings::new().expect("read config error");
    log::info!("{:#?}", settings);
    let server = OrderServerHandler::new(&settings).await.unwrap_or_else(|err| {
        log::error!("{:?} 服务启动失败...", err);
        // panic!("{}", err)
        process::exit(1);
    });
    run_server(server).await
}

async fn run_server(mut server: OrderServerHandler) -> Result<(), Box<dyn std::error::Error>> {
    let app_url = format!("{}:{}", server.settings.application.ip, server.settings.application.port);
    let addr = app_url.as_str().parse().unwrap();

    log::info!("Starting orderceter service on: {}\n", addr);

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let on_leave = server.on_leave();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        log::info!("Ctrl-c received, shutting down");
        tx.send(()).ok();
    });

    use std::time::Duration;
    use tonic::Request;
    
    // let mut request = Request::new(());
    
    // request.set_timeout(Duration::from_secs(30));
    
    // let value = request.metadata().get("grpc-timeout").unwrap();

    tonic::transport::Server::builder().timeout(Duration::from_millis(1000))//设置全部请求超时时间,超过时间直接返回
        .add_service(OrderCenterServiceServer::new(server))
        .serve_with_shutdown(addr, async {
            rx.await.ok();
        })
        .await?;

    log::info!("Shutted down, wait for final clear");
    on_leave.leave().await;
    log::info!("Shutted down");
    Ok(())
}