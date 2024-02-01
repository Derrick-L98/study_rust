pub use super::controller::*;

use crate::{
  config::settings::Settings,
  dataservice::dbinit::DbConnection,
  protofiles::{
    phoenixexchanger::phoenix_exchanger_server::PhoenixExchanger,
    phoenixordermsg::{OrderMsg, RouterMsg},
  },
};
use anyhow::Result;
use async_channel::unbounded;
use dashmap::DashMap;
use futures::Stream;
use messagecenter::{protofiles::hqmsg::YsHqInfo, quotationclient::QuotationClient};
use std::{collections::HashMap, sync::Arc};
use std::{error::Error, io::ErrorKind, pin::Pin};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{self, Request, Response, Status};

use tokio::sync::{mpsc, oneshot, RwLock};
// type StubType = Arc<RwLock<ServerController>>;
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

    let (tx_order, rx_order) = unbounded::<OrderMsg>();
    let (tx_persist, rx_persist) = unbounded::<OrderMsg>();
    let (tx, mut rx) = mpsc::channel(16);
    let (tx_close, mut rx_close) = oneshot::channel();

    //消息中心
    // let (tx_notification, mut rx_notification) = tokio::sync::mpsc::channel::<NotificationMessage>(128);
    // let message_key = "notification.datetime.#";
    // let notification_client = MessageClient::new(
    //   &settings.notification.exchanger.as_str(),
    //   "phoenix_exchanger_message_queue",
    //   message_key.to_string(),
    //   &settings.notification.amqpaddr.as_str(),
    //   tx_notification,
    // )
    // .await;
    // // 监听消息中心
    // tokio::spawn(async move {
    //   messagecenter::init::init_notification_listen(notification_client).await;
    // });

    //行情连接。。。。。。
    let (tx_quotation, mut rx_quotation) = tokio::sync::mpsc::channel::<YsHqInfo>(1024);
    let routing_key: HashMap<String, i32> = HashMap::new();
    let quotation_client = QuotationClient::new(
      &settings.notification.exchanger.as_str(),
      "phoenix_exchanger_quotation_queue",
      routing_key,
      &settings.notification.amqpaddr.as_str(),
      tx_quotation,
    )
    .await;
    // 监听行情
    tokio::spawn(async move {
      messagecenter::init::init_quotation_listen(quotation_client).await;
    });

    let order_cache = Arc::new(DashMap::new());
    let db_conn = DbConnection::new(&settings.database.uri).await;

    let stub = ServerController {
      settings: Arc::new(settings.to_owned()),
      order_receiver: rx_order.clone(),
      order_sender: tx_order.clone(),
      order_cache,
      db_conn: Arc::new(db_conn),
    };

    // let stub = Arc::new(RwLock::new(stub));
    let stub = Arc::new(stub);
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
            order_msg = rx_order.recv()=>{
                log::info!("收到订单消息......:{:?}",order_msg);
                if let Ok(order_msg) = order_msg {
                  //把订单信息存入缓存，同时发送消息，保存到数据库(可以通过channel的方式)
                  log::info!("order message is:{:?}",&order_msg);
                  let _ = stub_for_dispatch.new_order(&order_msg).await;
                  let _ = tx_persist.send(order_msg).await;
                }
            }
            persist_msg = rx_persist.recv()=>{
                log::info!("收到持久化信息...:{:?}",persist_msg);
                if let Ok(order_msg) = persist_msg {
                  //把订单信息保存到数据库
                  log::info!("order message is:{:?}",&order_msg);
                  let _ = stub_for_dispatch.persist_database().await;
                }
            }
            quotation_msg = rx_quotation.recv() => {
                //收到行情，判断处理成交，如何保证成交处理按顺序执行？
                //如果通过mpsc管道来处理，是否可以不用加锁？
                if let Some(quotation) = &quotation_msg{
                    //receive quotation
                    log::info!("receive quotation from quotation center: {:?}", &quotation);
                    //如果收到行情，处理成交
                    let ret = stub_for_dispatch.deal_order(&quotation).await;
                    //发送给订单路由
                    if ret.as_ref().is_ok(){
                      let ret  = tx_order.send(OrderMsg{..Default::default()}).await;
                      log::info!("send to 订单channel:{:?}",&ret);
                    }
                }
            }
            // notification = rx_notification.recv() => {
            //     //收到消息通知，需要判断是否交易时间
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
impl PhoenixExchanger for ServerHandler {
  type PhoenixExchangerOrderStream = Pin<Box<dyn Stream<Item = Result<RouterMsg, Status>> + Send + 'static>>;

  async fn phoenix_exchanger_order(&self, request: Request<tonic::Streaming<RouterMsg>>) -> Result<Response<Self::PhoenixExchangerOrderStream>, Status> {
    // let stub = self.stub.read().await;
    // Ok(Response::new(stub.phoenix_place_order(request.into_inner())?))
    // let ControllerDispatch(act, rt) = ControllerDispatch::new(move |ctrl: &mut ServerController| Box::pin(async move { ctrl.phoenix_place_order(request.into_inner()).await }));
    // self.task_dispacther.send(act).await.map_err(map_dispatch_err)?;
    // map_dispatch_ret(rt.await)

    let mut in_stream = request.into_inner();
    let (tx, rx) = mpsc::channel(128);

    // this spawn here is required if you want to handle connection error.
    // If we just map `in_stream` and write it back as `out_stream` the `out_stream`
    // will be drooped when connection error occurs and error will never be propagated
    // to mapped version of `in_stream`.
    let tx_clone = tx.clone();
    let order_tx_clone = self.stub.order_sender.clone();
    tokio::spawn(async move {
      while let Some(result) = in_stream.next().await {
        match result {
          Ok(v) => {
            //接收成功收，发送订单信息到管道处理缓存
            log::info!("received in stream:{:?}", &v);
            if let Some(msg_content) = v.msg_content {
              if let Some(order_msg) = msg_content.order_msg {
                let ret = order_tx_clone.send(order_msg).await;
                log::info!("send order to order channel result:{:?}", &ret);
              } else {
                log::error!("无效订单信息:{:?}", &msg_content)
              }
            } else {
              log::error!("无效订单信息")
            }
          }
          Err(err) => {
            if let Some(io_err) = match_for_io_error(&err) {
              if io_err.kind() == ErrorKind::BrokenPipe {
                // here you can handle special case when client
                // disconnected in unexpected way
                eprintln!("\tclient disconnected: broken pipe");
                break;
              }
            }

            match tx.send(Err(err)).await {
              Ok(_) => (),
              Err(_err) => break, // response was droped
            }
          }
        }
      }
      println!("\tstream ended");
    });

    let receiver_clone = self.stub.order_receiver.clone();
    tokio::spawn(async move {
      loop {
        tokio::select! {
            order_msg = receiver_clone.recv()=>{
                log::info!("receive order:{:?}",&order_msg);
                if let Ok(order) = order_msg {
                    log::info!("start to send order to client:{:?}",&order);
                    let _ = tx_clone.send(Ok(RouterMsg{..Default::default()}));
                }
            }
        }
      }
    });

    // echo just write the same data that was received
    let out_stream = ReceiverStream::new(rx);

    Ok(Response::new(Box::pin(out_stream) as Self::PhoenixExchangerOrderStream))

    // Ok(result)
    // Ok(Response::new(Box::pin(output) as Self::phoenix_place_orderStream))
  }
}

fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
  let mut err: &(dyn Error + 'static) = err_status;

  loop {
    if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
      return Some(io_err);
    }

    // h2::Error do not expose std::io::Error with `source()`
    // https://github.com/hyperium/h2/pull/462
    if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
      if let Some(io_err) = h2_err.get_io() {
        return Some(io_err);
      }
    }

    err = match err.source() {
      Some(err) => err,
      None => return None,
    };
  }
}
