pub use super::controller::*;

use crate::config::settings::Settings;
use common::uidservice::UidgenService;

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use tonic::{self, Request, Response, Status};
// use utility::constant::MessageType;
// use crate::protofiles::phoenixriskcenter::phoenix_riskcenter_server::PhoenixRiskcenter;
use crate::protofiles::phoenixriskcenter::{phoenix_riskcenter_server::PhoenixRiskcenter, PhoenixRiskCheckRequest, PhoenixRiskCheckResponse, PhoenixRiskRequest, PhoenixRiskResponse};
use futures::{
  channel::mpsc::{channel, Receiver},
  SinkExt, StreamExt,
};
use std::error::Error;
use std::path::Path;
// use std::fmt::Debug;
use std::sync::Arc;
use std::{pin::Pin, time::Duration};

use tokio::sync::{mpsc, oneshot, RwLock};

type StubType = Arc<ServerController>;
type ControllerAction = Box<dyn FnOnce(StubType) -> Pin<Box<dyn futures::Future<Output = ()> + Send>> + Send>;

pub struct ServerHandler {
  stub: StubType,
  task_dispacther: mpsc::Sender<ControllerAction>,
  // order_dispacther: mpsc::Sender<PhoenixRiskCheckInfo>,
  set_close: Option<oneshot::Sender<()>>,
  // mqclient: QuotationClient,
}
pub struct ServerLeave(mpsc::Sender<ControllerAction>, oneshot::Sender<()>);

impl ServerLeave {
  pub async fn leave(self) {
    self.1.send(()).unwrap();
    self.0.closed().await;
  }
}

impl ServerHandler {
  pub async fn new(settings: &Settings) -> Self {
    let mut persist_interval = tokio::time::interval(std::time::Duration::from_secs(5 as u64));

    let path = Path::new("config/default.toml");
    if path.exists() {
      log::info!("exist......");
    }

    let (tx_config, mut rx_config) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
      if let Err(e) = async_watch(path, tx_config).await {
        println!("error: {:?}", e)
      }
    });
    // let (tx_notification, mut rx_notification) = tokio::sync::mpsc::channel::<NotificationMessage>(128);

    // let message_key = "notification.datetime.#";
    // let notification_client = MessageClient::new(
    //     &settings.notification.exchanger.as_str(),
    //     message_key.to_string(),
    //     &settings.notification.amqpaddr.as_str(),
    //     tx_notification,
    // )
    // .await;
    // //监听时间
    // tokio::spawn(async move {
    //     msgcontroller::init::init_notification_listen(notification_client).await;
    // });
    let uidsvc = Arc::new(RwLock::new(UidgenService::new(settings.application.machineid, settings.application.nodeid)));
    let stub = ServerController {
      settings: Arc::new(RwLock::new(settings.clone())), // rbcon: rb.to_owned(),
      uidsvc,
    };

    let stub = Arc::new(stub);

    let (tx, mut rx) = mpsc::channel(16);
    let (tx_close, mut rx_close) = oneshot::channel();

    let stub_clone = stub.clone();
    let stub_for_dispatch = stub.clone();

    let svr_handler = ServerHandler {
      task_dispacther: tx,
      set_close: Some(tx_close),
      stub,
    };

    tokio::spawn(async move {
      persist_interval.tick().await; //skip first tick
      loop {
        tokio::select! {
            may_task = rx.recv() => {
                if let Some(task) = may_task{
                    task(stub_for_dispatch.clone()).await;
                }
            }
            _ = persist_interval.tick() => {
                log::info!("Start a time interval task (persist, computing):{:?}",std::thread::current());
                let _ = stub_clone.print_configurations().await;

            }
            config = rx_config.recv() =>{
              log::info!("configuration file changed......：{:?}", config);
              let ret = Settings::new();
              if ret.as_ref().is_ok(){
                let _ = stub_clone.update_configurations(&ret.unwrap()).await;
              }
            }
            // notification = rx_notification.recv() => {
            //     //收到消息通知
            //     if let Some(message) = &notification{
            //         //receive message
            //         log::info!("receive message from message center: {:?}", &message);
            //     }
            // }
            _ = &mut rx_close => {
                log::info!("Server scheduler is notified to close");
                rx.close();
                break;
            }
        }
      }

      //drain unhandled task
      while let Some(task) = rx.recv().await {
        task(stub_for_dispatch.clone()).await;
      }

      log::warn!("Server scheduler has exited");
    });

    svr_handler
  }
  pub fn on_leave(&mut self) -> ServerLeave {
    ServerLeave(self.task_dispacther.clone(), self.set_close.take().expect("Do not call twice with on_leave"))
  }
}

//这里实现grpc的接口
#[tonic::async_trait]
impl PhoenixRiskcenter for ServerHandler {
  async fn phoenix_risk_check(&self, request: Request<PhoenixRiskCheckRequest>) -> Result<Response<PhoenixRiskCheckResponse>, Status> {
    //注意：以下代码仅作示例
    let ret = self.stub.phoenix_risk_check(&request.into_inner()).await;
    Ok(Response::new(ret.unwrap()))
  }

  async fn phoenix_risk_test(&self, request: Request<PhoenixRiskRequest>) -> Result<Response<PhoenixRiskResponse>, Status> {
    //注意：以下代码仅作示例
    let ret = self.stub.phoenix_risk_test(&request.into_inner()).await;
    Ok(Response::new(ret.unwrap()))
  }
}

// -----------监听配置文件的变化-------------
fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
  let (mut tx, rx) = channel(1);

  // Automatically select the best implementation for your platform.
  // You can also access each implementation directly e.g. INotifyWatcher.
  let watcher = RecommendedWatcher::new(
    move |res| {
      futures::executor::block_on(async {
        tx.send(res).await.unwrap();
      })
    },
    Config::default(),
  )?;

  Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(path: P, tx: tokio::sync::mpsc::Sender<Event>) -> notify::Result<()> {
  let (mut watcher, mut rx) = async_watcher()?;

  // Add a path to be watched. All files and directories at that path and
  // below will be monitored for changes.
  watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

  while let Some(res) = rx.next().await {
    match res {
      Ok(event) => {
        // log::info!("changed: {:?}", event);
        let _ = tx.send(event).await;
      }
      Err(e) => log::error!("watch error: {:?}", e),
    }
  }

  Ok(())
}
