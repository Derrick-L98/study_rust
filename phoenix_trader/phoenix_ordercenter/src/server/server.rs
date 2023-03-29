use anyhow::Result;
// use dashmap::DashMap;
use std::sync::Arc;
use std::{fmt::Debug, pin::Pin, time::Duration};
use tonic::{Response, Status};
use tokio::sync::{broadcast, mpsc, oneshot, RwLock};
use common::redisclient::redisclient::RedisClient;
use common::uidservice::UidgenService;
use common::akaclient::AkaClient;
use common::logclient::*;
use messagecenter::notificationclient::NotificationClient;
use messagecenter::protofiles::phoenixnotification::NotificationMessage;

use crate::config::settings::Settings;
use crate::common::common::OrderDetail;
use super::controller::{OrderCenterController, PersistData};
use crate::client::{AssetsCenterClient, DbClient, OrderRouterClient, RiskCenterClient, AccountRiskClient};
use crate::protofiles::{
  order_center_service_server::OrderCenterService, 
  CancelReq, ExecMsg, OrderReq, OrderResp, RouterMsg,
  ReplenishOrderReq, ReplenishOrderResp
};

type StubType = Arc<OrderCenterController>;
type ControllerAction = Box<dyn (FnOnce(StubType) -> Pin<Box<dyn futures::Future<Output = ()> + Send>>) + Send>;

// type StubType = Arc<RwLock<OrderCenterController>>;
// struct ControllerDispatch<OT>(ControllerAction, oneshot::Receiver<OT>);
// impl<OT: 'static + Debug + Send> ControllerDispatch<OT> {
//   fn new<T>(f: T) -> Self
//   where
//     T: for<'c> FnOnce(&'c mut OrderCenterController) -> Pin<Box<dyn futures::Future<Output = OT> + Send + 'c>>,
//     T: Send + 'static,
//   {
//     let (tx, rx) = oneshot::channel();

//     ControllerDispatch(
//       Box::new(move |ctrl: StubType| -> Pin<Box<dyn futures::Future<Output = ()> + Send + 'static>> {
//         Box::pin(async move {
//           let mut wg = ctrl.write().await;
//           if let Err(t) = tx.send(f(&mut wg).await) {
//             log::error!("Controller action can not be return: {:?}", t);
//           }
//         })
//       }),
//       rx,
//     )
//   }
// }

// fn map_dispatch_err<T: 'static>(_: mpsc::error::SendError<T>) -> tonic::Status {
//   tonic::Status::unknown("Server temporary unavaliable")
// }
// type ControllerRet<OT> = Result<OT, tonic::Status>;
// type ServerRet<OT> = Result<Response<OT>, tonic::Status>;

// fn map_dispatch_ret<OT: 'static>(recv_ret: Result<ControllerRet<OT>, oneshot::error::RecvError>) -> ServerRet<OT> {
//   match recv_ret {
//     Ok(ret) => {
//       log::info!("收到结果，开始分发结果返回客户端");
//       ret.map(Response::new)
//     }
//     Err(_) => Err(Status::unknown("Dispatch ret unreach")),
//   }
// }

pub struct ServerLeave(mpsc::Sender<ControllerAction>, oneshot::Sender<()>);
impl ServerLeave {
  pub async fn leave(self) {
    self.1.send(()).unwrap();
    self.0.closed().await;
  }
}
pub struct OrderServerHandler {
  stub: StubType,
  pub settings: Settings,
  task_dispacther: mpsc::Sender<ControllerAction>,
  set_close: Option<oneshot::Sender<()>>,
}

impl OrderServerHandler {
  pub async fn new(settings: &Settings) -> Result<Self> {
    let (tx, mut rx) = mpsc::channel(32); //多生产 单消费
    let (tx_close, mut rx_close) = oneshot::channel(); //单生产 单消费

    //报单channel: 订单 -> channel send -> 报单
    let (tx_order, mut _rx_order) = broadcast::channel::<RouterMsg>(4096);
    //回报channel：回报 -> channel send -> 处理
    let (tx_repay, mut rx_repay) = broadcast::channel::<RouterMsg>(4096);
    // let (tx_order, mut rx_order) = broadcast::channel::<RouterMsg>(128);
    // let (tx_order, mut rx_order) = broadcast::channel::<RouterMsg>(128);
    // let (tx_order, mut rx_order) = broadcast::channel::<RouterMsg>(128);
    let (tx_persist, mut rx_persist) = mpsc::channel::<PersistData>(4096);
    let (tx_cancel, mut rx_cancel) = mpsc::channel::<ExecMsg>(4096);
    let (tx_opponent, mut rx_opponent) = mpsc::channel::<OrderDetail>(4096);

    //订阅mq: 目前本项目中无用
    let (tx_mq, mut _rx_mq) = mpsc::channel::<NotificationMessage>(4096);

    // 1. 日志中心客户端初始化
    let logclient = LogClient::init(&settings.grpcserver.logserver, "phoenix_ordercenter").await.expect("init logclient error");
    LOGCLIENT.set(logclient).expect("set global logclient error");
    //2. 写入日志的办法
    // LogClient::get().push_error("error message is founded......").await;

    let db_client = DbClient::new(&settings.mysql.uri).await;
    let redis_client = RedisClient::new(&settings.redis.uri).await.expect("redis connect err");
    let uidgen = UidgenService::new(settings.application.machineid, settings.application.nodeid);
 
    let mut mq_client = NotificationClient::new(&settings.mq.exchanger, &settings.mq.queue, settings.mq.routingkey.to_owned(), &settings.mq.amqpaddr, tx_mq).await;
    let _ = mq_client.try_connect().await.expect("mq client connection error");

    let riskcenter_client = RiskCenterClient::new(&settings.grpcserver.riskcenter).await;
    let assetscenter_client = AssetsCenterClient::new(&settings.grpcserver.assetscenter).await;
    let aka_client = AkaClient::init(settings.grpcserver.akacenter.clone(), true, settings.system.cachelong as i64).await;
    let account_risk_client = AccountRiskClient::new(&settings.grpcserver.accountriskcenter).await;

    let stub = OrderCenterController {
      settings: Arc::new(settings.to_owned()),
      tx_persist,
      tx_order: tx_order.clone(),
      tx_cancel,
      tx_opponent,
      db_client: Arc::new(db_client),
      riskcenter_client: Arc::new(riskcenter_client),
      aka_client: Arc::new(aka_client),
      assetscenter_client: Arc::new(assetscenter_client),
      account_risk_client: Arc::new(account_risk_client),
      redis_client: Arc::new(redis_client),
      mq_client: Arc::new(mq_client),
      uidgen: Arc::new(RwLock::new(uidgen)),
    };

    let stub = Arc::new(stub);
    let stub_clone = stub.clone();
    // let stub_copy = stub.clone();
    let stub_for_dispatch = stub.clone();

    let ret = OrderServerHandler {
      stub,
      settings: settings.to_owned(),
      task_dispacther: tx,
      set_close: Some(tx_close),
    };

    if settings.grpcserver.orderrouter.to_owned().is_some() {
      let url = settings.grpcserver.orderrouter.as_ref().unwrap();
      let mut order_router_client = OrderRouterClient::new(&url, tx_order.clone(), tx_repay.clone()).await;
      let mut retry_interval = tokio::time::interval(Duration::from_secs(3));
      tokio::spawn(async move {
        retry_interval.tick().await;
        loop {
          tokio::select! {
              _ = retry_interval.tick() => {
                  if let Err(err) = order_router_client.order_routing().await {
                      log::error!("{:?}", err);
                      let _ = order_router_client.retry_order_routing().await;
                  }
              }
          }
        }
      });
    }

    // 以下用来测试订阅mq消息
    // let mut interval = tokio::time::interval(Duration::from_secs(3));
    // tokio::spawn(async move {
    //   loop {
    //     tokio::select! {
    //         _ = interval.tick() => {
    //           let _ = stub_copy.mq_client.try_consume().await;
    //         }
    //     }
    //   }
    // });

    // tokio::spawn(async move {
    //   loop {
    //     tokio::select! {

    //     }
    //   }
    // });    


    tokio::spawn(async move {
      loop {
        tokio::select! {
            may_task = rx.recv() => {
                log::info!("task from client received...");
                let task = may_task.expect("Server scheduler has unexpected exit");
                task(stub_for_dispatch.clone()).await;
            }
            // _ = persist_interval.tick() => {//定时任务
            // }
            order_task = rx_repay.recv() => {//订单回报channel
                if let Ok(orderresp) = order_task {
                    log::info!("订单回报: {:#?}", orderresp);
                    if let Err(err) = stub_clone.order_receipt(&orderresp).await {
                      log::error!("处理失败: {:?}", err);
                    }
                }
            }
            // mq_task = rx_mq.recv() => {//测试mq消息接收
            //     if let Some(msg) = mq_task {
            //       log::info!("==================={:#?}", msg);
            //     }
            // }
            // plase_task = rx_plase.recv() => {//下单channel
            //     if let Some(plase_order) = plase_task {
            //         log::info!("下单: {:#?}", plase_order);
            //     }
            // }
            // cancel_task = rx_cancel.recv() => {//撤单channel
            //     if let Some(cancel_order) = cancel_task {
            //         log::info!("撤单: {:#?}", cancel_order);
            //     }
            // }
            persist_task = rx_persist.recv() => {//持久化channel
                if let Some(persist_data) = persist_task {
                  if let Err(err) = stub_clone.persist_data(&persist_data).await {
                      log::error!("persist data error: {:#?}", err);
                  }
                }
            }
            cancel_task = rx_cancel.recv() => {
                if let Some(exec_msg) = cancel_task {
                  if let Err(err) = stub_clone.cancel_order_receipt(&exec_msg).await {
                      log::error!("cancel order error: {:#?}", err);
                  }
                }
            }
            opponent_task = rx_opponent.recv() => {
              if let Some(mut data) = opponent_task {
                log::info!("{:?}", data);
                if let Err(err) = stub_clone.generate_opponent_order(&mut data).await {
                  log::error!("generate_opponent_order error: {:#?}", err);
                }
              }
            }
            _ = &mut rx_close => {
                log::info!("Server scheduler is notified to close");
                rx.close();
                break;
            }
        }
      }

      while let Some(task) = rx.recv().await {
        task(stub_for_dispatch.clone()).await;
        log::info!("drain unhandled task received");
      }

      log::warn!("Server scheduler has exited");
    });

    Ok(ret)
    // Err(anyhow!("启动失败"))
  }

  pub fn on_leave(&mut self) -> ServerLeave {
    ServerLeave(self.task_dispacther.clone(), self.set_close.take().expect("Do not call twice with on_leave"))
  }
}

#[tonic::async_trait]
impl OrderCenterService for OrderServerHandler {
  async fn place_order(&self, request: tonic::Request<OrderReq>) -> Result<tonic::Response<OrderResp>, tonic::Status> {
    log::info!("place_order Client from {:?}\n", request.remote_addr());

    let req = request.into_inner();
    match self.stub.place_order(&req).await {
      Ok(val) => Ok(Response::new(val)),
      Err(err) => {
        log::error!("place order err: {:?}", err);
        Ok(Response::new(OrderResp::default()))
      }
    }
  }
  
  async fn cancel_order(&self, request: tonic::Request<CancelReq>) -> Result<tonic::Response<OrderResp>, tonic::Status> {
    log::info!("cancel_order Client from {:?}\n", request.remote_addr());

    let req = request.into_inner();
    match self.stub.cancel_order(&req).await {
      Ok(val) => Ok(Response::new(val)),
      Err(err) => {
        log::error!("cancel order err: {:?}", err);
        Ok(Response::new(OrderResp::default()))
      }
    }
  }

  async fn replenishment_order(&self, request: tonic::Request<ReplenishOrderReq>) -> Result<tonic::Response<ReplenishOrderResp>, tonic::Status> {
    log::info!("replenishment_order Client from {:?}\n", request.remote_addr());
    // request.set_timeout(Duration::from_secs(30));
    let req = request.into_inner();
    let mut resp = ReplenishOrderResp::default();
    match self.stub.replenishment_order(&req).await {
      Ok(_) => {
        resp.err_code = 0;
        resp.err_msg = format!("补单成功");
        Ok(Response::new(resp))
      },
      Err(err) => {
        log::error!("replenishment_order order err: {:?}", err);
        resp.err_code = 1;
        resp.err_msg = format!("{}", err);
        Ok(Response::new(resp))
      }
    }
  }
}
