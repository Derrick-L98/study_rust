pub use super::controller::*;

use crate::config::settings::Settings;
// use futures;
use tonic::{self, Request, Response, Status};
// use utility::constant::MessageType;

// use msgcontroller::messageclient::MessageClient;
// use msgcontroller::protofiles::notification::NotificationMessage;

// use crate::protofiles::phoenixriskcenter::phoenix_riskcenter_server::PhoenixRiskcenter;
use crate::protofiles::phoenixriskcenter::{phoenix_riskcenter_server::PhoenixRiskcenter, PhoenixRiskCheckRequest, PhoenixRiskCheckResponse, PhoenixRiskRequest, PhoenixRiskResponse};

use std::fmt::Debug;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::{mpsc, oneshot, RwLock};
type StubType = Arc<RwLock<ServerController>>;
type ControllerAction = Box<dyn FnOnce(StubType) -> Pin<Box<dyn futures::Future<Output = ()> + Send>> + Send>;

pub struct ServerHandler {
    stub: StubType,
    task_dispacther: mpsc::Sender<ControllerAction>,
    // order_dispacther: mpsc::Sender<PhoenixRiskCheckInfo>,
    set_close: Option<oneshot::Sender<()>>,
    // mqclient: QuotationClient,
}
struct ControllerDispatch<OT>(ControllerAction, oneshot::Receiver<OT>);

impl<OT: 'static + Debug + Send> ControllerDispatch<OT> {
    fn new<T>(f: T) -> Self
    where
        T: for<'c> FnOnce(&'c mut ServerController) -> Pin<Box<dyn futures::Future<Output = OT> + Send + 'c>>,
        T: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();

        ControllerDispatch(
            Box::new(move |ctrl: StubType| -> Pin<Box<dyn futures::Future<Output = ()> + Send + 'static>> {
                Box::pin(async move {
                    let mut wg = ctrl.write().await;
                    if let Err(t) = tx.send(f(&mut wg).await) {
                        log::error!("Controller action can not be return: {:?}", t);
                    }
                })
            }),
            rx,
        )
    }
}

fn map_dispatch_err<T: 'static>(_: mpsc::error::SendError<T>) -> tonic::Status {
    tonic::Status::unknown("Server temporary unavaliable")
}
type ControllerRet<OT> = Result<OT, tonic::Status>;
type ServerRet<OT> = Result<Response<OT>, tonic::Status>;

fn map_dispatch_ret<OT: 'static>(recv_ret: Result<ControllerRet<OT>, oneshot::error::RecvError>) -> ServerRet<OT> {
    match recv_ret {
        Ok(ret) => {
            log::info!("收到结果，开始分发结果返回客户端");
            ret.map(Response::new)
        }
        Err(_) => Err(Status::unknown("Dispatch ret unreach")),
    }
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

        let stub = ServerController {
            settings: settings.clone(), // rbcon: rb.to_owned(),
        };

        let stub = Arc::new(RwLock::new(stub));

        let (tx, mut rx) = mpsc::channel(16);
        let (tx_close, mut rx_close) = oneshot::channel();

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
        // let stub = self.stub.read().await;
        // Ok(Response::new(stub.phoenix_place_order(request.into_inner())?))
        let ControllerDispatch(act, rt) = ControllerDispatch::new(move |ctrl: &mut ServerController| Box::pin(async move { ctrl.phoenix_risk_check(request.into_inner()).await }));

        self.task_dispacther.send(act).await.map_err(map_dispatch_err)?;
        map_dispatch_ret(rt.await)
    }

    async fn phoenix_risk_test(&self, request: Request<PhoenixRiskRequest>) -> Result<Response<PhoenixRiskResponse>, Status> {
        // let stub = self.stub.read().await;
        // Ok(Response::new(stub.phoenix_place_order(request.into_inner())?))
        let ControllerDispatch(act, rt) = ControllerDispatch::new(move |ctrl: &mut ServerController| Box::pin(async move { ctrl.phoenix_risk_test(request.into_inner()).await }));

        self.task_dispacther.send(act).await.map_err(map_dispatch_err)?;
        map_dispatch_ret(rt.await)
    }
}
