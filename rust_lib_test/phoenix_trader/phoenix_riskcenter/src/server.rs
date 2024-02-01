pub use super::controller::*;
use futures;

use tonic::{self, Request, Response, Status};
use crate::protofiles::{account_risk_center_client::AccountRiskCenterClient};
use crate::protofiles::phoenix_riskcenter_server::PhoenixRiskcenter;
use crate::protofiles::{PhoenixRiskCheckResponse, PhoenixRiskRequest, PhoenixRiskResponse};

use crate::{config::settings::Settings, protofiles::PhoenixRiskCheckRequest};
use messagecenter::notificationclient::NotificationClient;
use messagecenter::protofiles::phoenixnotification::NotificationMessage;
use crate::service::ordercontroller::OrderController;
use std::fmt::Debug;
use std::pin::Pin;
use std::sync::Arc;
use crate::client::{ AccountRiskClient,HqCenterClient};
use tokio::sync::{mpsc, oneshot, RwLock};
use utility::{errors,errors::ErrorCode};
type StubType = Arc<RwLock<RiskCenterController>>;
use crate::basicdata::marketclosetimecache::CloseTimeCached;
use common::constant::MessageType;
use common::akaclient::AkaClient;
// type StubType = Arc<Controller>;
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
            T: for<'c> FnOnce(&'c mut RiskCenterController) -> Pin<Box<dyn futures::Future<Output = OT> + Send + 'c>>,
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

        let (tx_notification, mut rx_notification) = tokio::sync::mpsc::channel::<NotificationMessage>(128);

        let message_key = "notification.datetime.#";
        let notification_client = NotificationClient::new(
            &settings.notification.exchanger.as_str(),
            "",
            message_key.to_string(),
            &settings.notification.amqpaddr.as_str(),
            tx_notification,
        )
            .await;
        messagecenter::init::init_notification_client(notification_client).await;

        // let tmp_s = settings.system.assetsserver.to_owned();
        // let account_risk_server: &'static str = Box::leak(tmp_s.into_boxed_str());
        let account_risk_client =AccountRiskClient::new(&settings.system.accountassetsserver).await;
        let akacenterconn = AkaClient::init(settings.system.akacenterserver.to_string(), false,settings.system.cachelong as i64).await;
        let hq_center_client = HqCenterClient::new(&settings.system.hqcenterserver).await;
        // let redisclient = RedisClient::new(settings.redis.uri.as_str()).await.expect("init redis cluster error");
        let mut order_controller = OrderController::new(settings, settings.system.cachelong);
        let close_time_cache = CloseTimeCached::new();
        // let _ = order_controller.channels_controller.init_channels(&basic_client).await.expect("init all channels error");
        // let mut datetime_controller = CloseTimeCached::new();
        // let _ =datetime_controller.init_market_closetime(&basic_client).await.expect("init close time error");
        // let mut commidity_controller= CommodityCached::new();
        // let _ = commidity_controller.init_commodity(&basic_client).await.expect("init commidity error");
        // let mut market_controller = MarketsCached::new();
        // let _ = market_controller.init_markets(&basic_client).await.expect("init market error");
        let stub = RiskCenterController {
            order_controller,
            close_time_cache,
            settings: settings.clone(), // rbcon: rb.to_owned(),
            account_risk_client,
            akacenterconn,
            hq_center_client,
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
                        // let task = may_task.expect("Server scheduler has unexpected exit");
                        // log::info!("启动 controller task。====================== thread:{:?}", std::thread::current());
                        if let Some(task) = may_task{
                            task(stub_for_dispatch.clone()).await;
                        }
                    }
                    _ = persist_interval.tick() => {
                        // let stub_rd = stub_for_dispatch.read().await;
                        log::info!("Start a time interval task (persist, computing):{:?}",std::thread::current());
                        //
                        // let msg_content = NotificationDatetime { operation_type: OperateType::Insert as i32, datetime_type: 0, id: 66, market_id: 103, msg_time: "2022-02-10 13:51:00|2022-02-10 23:00:00".to_string(), msg_type: 1 };
                        // if let Ok(_) = stub_for_dispatch.write().await.datetime_controller.update_closetime_from_notification(msg_content.id, msg_content.market_id, msg_content.msg_type, msg_content.msg_time.as_str(), msg_content.operation_type){
                        // }else{
                        //     log::error!("update close time error:{:?}", &msg_content);
                        // }
                    }
                    notification = rx_notification.recv() => {
                        //收到消息通知
                        if let Some(message) = &notification{
                            //receive message
                            log::info!("receive message from message center: {:?}", &message);
                            if message.msg_type == MessageType::DateTimeMsg as i32 {
                                //update date time cache
                                if let Some(msg_body) = &message.msg_body {
                                    if let Some(msg_content) = &msg_body.msg_datetime{
                                        //handle date time message
                                        log::info!("date time message:{:?}", &msg_content);
                                        // stub_for_dispatch.write().await.datetime_controller.update_closetime(str, operate)
                                        if let Ok(_) = stub_for_dispatch.write().await.close_time_cache.update_closetime_from_notification(msg_content.market_id as i32, msg_content.msg_type, msg_content.msg_time.as_str(), msg_content.operation_type){

                                        }else{
                                            log::error!("update close time error:{:?}", &msg_content);
                                        }
                                    }
                                }
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

#[tonic::async_trait]
impl PhoenixRiskcenter for ServerHandler {
    async fn phoenix_risk_check(&self, request: Request<PhoenixRiskCheckRequest>) -> Result<Response<PhoenixRiskCheckResponse>, Status> {
        // log::info!("收到获取订单通道的请求...");

        // let stub = self.stub.read().await;
        let mut stub = self.stub.write().await;

        // let queryinfo = request.into_inner().queryinfo;
        let mut order = match request.into_inner().queryinfo {
            Some(v) => v,
            None => {
                let result = PhoenixRiskCheckResponse {
                    ret_code: errors::get_error_code(ErrorCode::CodeSystemErrRequest).0,
                    ret_msg: errors::get_error_code(ErrorCode::CodeSystemErrRequest).1,
                    retinfo: Vec::new(),
                };
                return Ok(Response::new(result));
            }
        };
        // let _send_ret = self.order_dispacther.send(order.clone()).await;

        let ret = stub.phoenix_risk_check(&mut order);
        // let ret = stub.phoenix_risk_check(request.into_inner());
        if let Ok(res_ret) = ret.await {
            Ok(Response::new(res_ret))
        } else {
            // Ok(Response::new(ret.await.unwrap()))
            let result = PhoenixRiskCheckResponse {
                ret_code: errors::get_error_code(ErrorCode::CodeSystemErrRequest).0,
                ret_msg: errors::get_error_code(ErrorCode::CodeSystemErrRequest).1,
                retinfo: Vec::new(),
            };
            return Ok(Response::new(result));
        }
        // 以下方法是单线程进行（涉及到数据的写)
        // let ControllerDispatch(act, rt) =
        //     ControllerDispatch::new(move |ctrl: &mut Controller| Box::pin(async move { ctrl.phoenix_risk_check(request.into_inner()) }));
        // log::info!("分发任务,发送内容到controller...");
        // self.task_dispacther.send(act).await.map_err(map_dispatch_err)?;
        // log::info!("等待计算结果...");
        // map_dispatch_ret(rt.await)
    }

    async fn phoenix_risk_test(&self, request: Request<PhoenixRiskRequest>) -> Result<Response<PhoenixRiskResponse>, Status> {
        // let stub = self.stub.read().await;
        // Ok(Response::new(stub.phoenix_place_order(request.into_inner())?))
        let ControllerDispatch(act, rt) = ControllerDispatch::new(move |ctrl: &mut RiskCenterController| Box::pin(async move { ctrl.phoenix_risk_test(request.into_inner()).await }));

        self.task_dispacther.send(act).await.map_err(map_dispatch_err)?;
        map_dispatch_ret(rt.await)
    }
}
