pub use super::controller::*;

use crate::protofiles::assetscenter::{phoenixassetscenter_server::Phoenixassetscenter, PhoenixassetscenterQueryRequest, PhoenixassetscenterRequest, PhoenixassetscenterResponse};
use crate::{
  config::settings::{self, Settings},
  dataservice::dbsetup,
  service::{assetssvc::UnitAssetsService, stockpositionsvc::UnitPositionService},
};
use common::akaclient::AkaClient;
use common::constant;
use common::logclient::LogClient;
use common::{redisclient::redisclient::RedisClient, uidservice::UidgenService};
use utility::errors::{self, ErrorCode};
// use messagecenter::assetsclient::AssetsChangeClient;
use crate::dataservice::entities::prelude::PhoenixSysSystem;
use messagecenter::notificationclient::NotificationClient;
use messagecenter::protofiles::phoenixnotification::NotificationMessage;
use std::sync::Arc;
use std::{fmt::Debug, pin::Pin};
use tokio::sync::{mpsc, oneshot, RwLock};
use tonic::{self, Request, Response, Status};
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
    let mut persist_interval = tokio::time::interval(std::time::Duration::from_secs(5 * 60 * 60 as u64));
    let (tx, mut rx) = mpsc::channel(16);
    let (tx_close, mut rx_close) = oneshot::channel();

    let (tx_persist, mut rx_persist) = mpsc::channel::<PersistData>(1024);
    //
    let (tx_notification, mut rx_notification) = mpsc::channel::<NotificationData>(1024);

    let dbconn = dbsetup::DbConnection::new(&settings.database.uri).await;
    let redisclient: RedisClient = RedisClient::new(&settings.redis.uri).await.unwrap();
    let uidsvc = UidgenService::new(settings.application.machineid, settings.application.nodeid);

    let (tx_message, mut rx_message) = mpsc::channel::<NotificationMessage>(1024);
    let queue_name = "phoenix_assetscenter_queue";
    let msgclient = NotificationClient::new(&settings.msgcenter.assets_exchange, queue_name, settings.msgcenter.routing_key.clone(), &settings.msgcenter.amqpaddr, tx_message).await;
    let msgclient = Arc::new(msgclient);

    let akaclient = AkaClient::init(settings.system.akaserver.clone(), true, settings.system.cachetime).await;
    let sysinfo = PhoenixSysSystem::find(&dbconn).await.expect("find error").unwrap();

    let stub = ServerController {
      redis: Arc::new(redisclient),
      dbconn: Arc::new(dbconn),
      assetssvc: Arc::new(UnitAssetsService::new()),
      positionsvc: Arc::new(UnitPositionService::new()),
      uidgen: Arc::new(uidsvc),
      akasvc: Arc::new(akaclient),
      config: Arc::new(RwLock::new(settings.to_owned())),
      sysinfo: Arc::new(sysinfo),
      notify: msgclient,
      tx_persist,
      tx_notification,
    };
    //用户资产数据初始化
    stub.init().await;

    let stub = Arc::new(stub);
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
            }
            message = rx_message.recv() =>{
              log::info!("received message:{:?}",&message);
            }
            persist = rx_persist.recv() => {
              log::info!("receive persist data:{:?}",&persist);
              if let Some(persist_data)=persist{
                let _ = stub_clone.save_dbdata(&persist_data).await;
              }
            }
            notification = rx_notification.recv() =>{
              log::info!("received message:{:?}",&notification);
              if let Some(notification_data) = notification{
                  let _ = stub_clone.publish_data(&notification_data).await;
              }
            }
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
impl Phoenixassetscenter for ServerHandler {
  async fn phoenix_assets_change(&self, request: Request<PhoenixassetscenterRequest>) -> Result<Response<PhoenixassetscenterResponse>, Status> {
    // let stub = self.stub.read().await;
    // Ok(Response::new(stub.phoenix_place_order(request.into_inner())?))
    let req = request.into_inner();
    let ret = self.stub.phoenix_assets_change(&req).await;

    match ret {
      Ok(v) => Ok(Response::new(v)),
      Err(e) => {
        let res = PhoenixassetscenterResponse {
          ret_code: constant::ResultCode::ERROR as i32,
          ret_msg: e.to_string(),
          assetsinfo: vec![],
        };
        Ok(Response::new(res))
      }
    }
  }

  async fn phoenix_assets_query(&self, request: Request<PhoenixassetscenterQueryRequest>) -> Result<Response<PhoenixassetscenterResponse>, Status> {
    // let stub = self.stub.read().await;
    // Ok(Response::new(stub.phoenix_place_order(request.into_inner())?))
    // let ControllerDispatch(act, rt) = ControllerDispatch::new(move |ctrl: &mut ServerController| Box::pin(async move { ctrl.phoenix_assets_query(request.into_inner()).await }));
    // self.task_dispacther.send(act).await.map_err(map_dispatch_err)?;
    // map_dispatch_ret(rt.await)
    let req = request.into_inner();
    let ret = self.stub.phoenix_assets_query(&req).await;

    match ret {
      Ok(v) => Ok(Response::new(v)),
      Err(e) => {
        let res = PhoenixassetscenterResponse {
          ret_code: constant::ResultCode::ERROR as i32,
          ret_msg: e.to_string(),
          assetsinfo: vec![],
        };
        Ok(Response::new(res))
      }
    }
  }
}
