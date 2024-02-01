// use std::thread;
// use crossbeam_utils::thread;
// use anyhow::Result;
use dashmap::DashMap;
use futures::Stream;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, oneshot, RwLock};
use utility::timeutil::{current_date, current_timestamp};
//use crate::server::controller::OrderRouterController;
use crate::config::settings::Settings;
use lazy_static::lazy_static;
// use crate::protofiles::hqmsg::YsHqInfo;
use crate::protofiles::phoenixordermsg::RouterMsg;
use crate::protofiles::{
  MsgContent,
  MsgType::{self, *},
  RegisterReq,
};
// use crossbeam::channel::unbounded;
// use async_channel::unbounded;
use async_channel::{Receiver as async_receiver, Sender as async_sender,unbounded};
use std::{pin::Pin, sync::Mutex, time::Duration,error::Error, io::ErrorKind};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{IntoRequest, Request, Response, Status, Streaming};
use common::uidservice;
use common::logclient::*;
//use std::fmt::Debug;

use crate::server::controller::OrderRouterController;
type OrderStream = Pin<Box<dyn Stream<Item = Result<RouterMsg, Status>> + Send>>;
type StubType = Arc<OrderRouterController>;
type ControllerAction = Box<dyn FnOnce(StubType) -> Pin<Box<dyn futures::Future<Output = ()> + Send>> + Send>;
//type OrderChannel = (Sender<RouterMsg>, mut Receiver<RouterMsg>);
//const bp_cache :DashMap<i64, (Sender<Result<RouterMsg, Status>>, Receiver<Result<RouterMsg, Status>>)> =  DashMap::new();
//static  bp_cache:DashMap<i32, Sender<Result<RouterMsg, Status>>> =
// const bp_cache:DashMap<i32, Sender<Result<RouterMsg, Status>>> ={
//     let map:DashMap<i32, Sender<Result<RouterMsg, Status>>> =  DashMap::new();
// };
lazy_static! {
    //static ref BP_CACHE:DashMap<i64, mpsc::Sender<Result<RouterMsg, Status>>> = DashMap::new();
    static ref BP_CACHE:DashMap<i64, MapSender> = DashMap::new();
}
//static mut order_channel:(Sender<RouterMsg>, Receiver<RouterMsg>) = mpsc::channel::<RouterMsg>(32);

// #[derive(Copy)]
pub struct RouterServerHandler {
  stub: StubType,
  pub settings: Settings,
  task_dispacther: mpsc::Sender<ControllerAction>,
  set_close: Option<oneshot::Sender<()>>,
  order_sender: async_sender<RouterMsg>,
  order_receiver: async_receiver<RouterMsg>,
 // log_client:LogClient,
  //order_cache:DashMap<i64, Sender<T>, Receiver<T>>,
  // bp_cache:DashMap<i64, (Sender<Result<RouterMsg, Status>>, Receiver<Result<RouterMsg, Status>>)>,
}
#[derive(Clone)]
pub struct MapSender {
  sender_id :i64,
  sender_copy : Sender<Result<RouterMsg, Status>>,
}
pub struct ServerLeave(mpsc::Sender<ControllerAction>, oneshot::Sender<()>);
impl ServerLeave {
  pub async fn leave(self) {
    self.1.send(()).unwrap();
    self.0.closed().await;
  }
}
impl RouterServerHandler {
  pub async fn new(settings: &Settings) -> Self {
    // let mut persist_interval = tokio::time::interval(std::time::Duration::from_secs(setting.system.cassinterval as u64));

    //创建通道
    let (tx, mut rx) = mpsc::channel(32);
    let (tx_close, mut rx_close) = oneshot::channel(); //创建一次性通道,用于发送单个值

    // let (tx_tick, mut rx_tick) = broadcast::channel::<YsHqInfo>(setting.system.channelcap as usize);
    // let (order_tx, mut order_rx) = mpsc::channel::<RouterMsg>(32);

    let (tx_s, rx_s) = unbounded::<RouterMsg>();
    // ringbuffer

    let stub = OrderRouterController {
      // settings: settings.clone(),

    };

    let stub = Arc::new(stub);
    let stub_for_dispatch = stub.clone();
    //let log_c = LogClient::get().expect("get log_client error");
    let ret = RouterServerHandler {
      stub,
      settings: settings.clone(),
      task_dispacther: tx,
      set_close: Some(tx_close),
      order_sender: tx_s,
      order_receiver: rx_s,
      //log_client:*log_c,
      // order_channel : mpsc::channel::<RouterMsg>(32),
      // order_cache: DashMap::new(),
      // bp_cache: DashMap::new(),
    };

    // let mut retry_interval = tokio::time::interval(Duration::from_secs(3));
    // tokio::spawn(async move {
    //     loop{
    //         tokio::select! {
    //             _ = retry_interval.tick() => {
    //                 if let Err(err) = mktclient.do_subscribe_market_data().await {
    //                     log::error!("can't subscribe market data......{:?}", err);
    //                     let _ = mktclient.retry_do_subscribe_market_data().await;
    //                 }
    //             }
    //         }
    //     }
    // });

    tokio::spawn(async move {
      // persist_interval.tick().await;
      loop {
        tokio::select! {
            may_task = rx.recv() => {
                log::info!("task from client received...");
                let task = may_task.expect("Server scheduler has unexpected exit");
                task(stub_for_dispatch.clone()).await;
            }
            // _ = persist_interval.tick() => {
            // }
            // _ = rx_tick.recv() => {
            //     //do nothing......
            //     // log::info!("************** Ticks:{:?}",&tick_task);
            // }
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

    ret
  }

  pub fn on_leave(&mut self) -> ServerLeave {
    ServerLeave(self.task_dispacther.clone(), self.set_close.take().expect("Do not call twice with on_leave"))
  }
}

#[tonic::async_trait]
impl crate::protofiles::order_router_service_server::OrderRouterService for RouterServerHandler {
  type OrderRoutingStream = OrderStream;

  async fn order_routing(&self, request: Request<Streaming<RouterMsg>>) -> Result<Response<Self::OrderRoutingStream>, Status> {
    log::info!("client connected from:{:?}", &request.remote_addr());
    let mut in_stream = request.into_inner();
    let (tx, rx) = mpsc::channel(128);
    // let con = self.stub.clone();
    // let mut order_c = self.stub.order_sender.subscribe();
    let order_c = self.order_receiver.clone();
    // let order_handle = self.clone();
    let tx_c = tx.clone();
    let order_sender_copy = self.order_sender.clone();
    tokio::spawn(async move {
      while let Some(result) = in_stream.next().await {
        match result {
          Ok(val) => {
            log::info!("{:#?}", val);
            let router_msg = val.clone();
            if val.msg_type == (Order as i32) {
              //下单
              if let Some(msg) = val.msg_content {
                if let Some(orderMsg) = msg.order_msg {
                  if BP_CACHE.contains_key(&orderMsg.channel_id) {
                    let ret = BP_CACHE.get(&orderMsg.channel_id).expect("get channel_id value error").sender_copy.send(Result::<RouterMsg, Status>::Ok(router_msg)).await;
                    if ret.as_ref().is_err() {
                      log::error!("send bp error:{:#?}", ret.err().unwrap().to_string());
                    }
                  } else {
                    log::error!("can't find channel:{:?}", orderMsg.channel_id);
                  }
                  let send_err = tx
                    .send(Result::<RouterMsg, Status>::Ok(RouterMsg {
                      msg_type: Response as i32,
                      msg_id: val.msg_id,
                      msg_content: None,
                      msg_time: current_timestamp(),
                    }))
                    .await;
                  if send_err.as_ref().is_err() {
                    log::error!("send bp error:{:#?}", send_err.err().unwrap().to_string());
                  }
                }
              }
            } else {
              log::info!("order type error :{:#?}", val);
            }
          }
          Err(err) => {
            // match tx.send(Err(err)).await {
            //   Ok(_) => (),
            //   Err(_err) => break, // response was droped
            // }
            if let Some(io_err) = match_for_io_error(&err) {
              if io_err.kind() == ErrorKind::BrokenPipe {
                // here you can handle special case when client
                // disconnected in unexpected way
                eprintln!("\tclient disconnected: broken pipe");
                break ;
              }
            }

            match tx.send(Err(err)).await {
              Ok(_) => (),
              Err(_err) => break, // response was droped
            }

          }
        }
      }
      log::info!("\tstream ended");
      return;
    });

    tokio::spawn(async move {
      loop {
        tokio::select! {
          order_msg=order_c.recv()=>{
            log::info!("received msg:{:?}",&order_msg);
            if let Ok(msg) = order_msg{
              let order_msg_copy = msg.clone();
            let ret =  tx_c.send(Result::<RouterMsg, Status>::Ok(msg)).await;
            if ret.as_ref().is_err(){
              //send to log center
              log::error!("send to client error:{}",ret.as_ref().err().unwrap().to_string());
               //消息发送失败后,重新去推送
               let ret = order_sender_copy.send(order_msg_copy).await;
               if ret.as_ref().is_err() {
                 log::error!("send bp channel error:{:#?}", ret.err().unwrap().to_string());
               }
               return ;
            }
            }
          }
        }
      }
    });
    //let ret =  tx_c.send(Result::<RouterMsg, Status>::Ok(msg)).await;
    // std::thread::spawn(move || {
    //   loop {
    //     // tokio::select! {
    //     // match self.stub.order_receiver.recv() {
    //     match order_c.recv() {
    //       Ok(v) => {
    //         log::info!("client received...{:#?}", v);
    //         let ret = tx_c.send(Result::<RouterMsg, Status>::Ok(v));
    //       }
    //       Err(e) => {
    //         log::error!("error:{}", e);
    //       } // if let Ok(msg) = order_msg{
    //         //
    //         //    if ret.as_ref().is_err(){
    //         //     log::error!("send bp error:{:#?}",ret.err().unwrap().to_string());
    //         //    }
    //         //    log::info!("send succeed");
    //         // }
    //     }
    //     // }
    //   }
    // });
    let out_stream = ReceiverStream::new(rx);
    Ok(Response::new(Box::pin(out_stream) as Self::OrderTransferStream))
  }

  type OrderTransferStream = OrderStream;
  async fn order_transfer(&self, request: tonic::Request<tonic::Streaming<RouterMsg>>) -> Result<tonic::Response<Self::OrderTransferStream>, tonic::Status> {
    log::info!("client connected from:{:?}", &request.remote_addr());

    //let mut stub = self.stub.write().await;

    let mut in_stream = request.into_inner();

    let (tx, rx) = mpsc::channel(128);
    let order_sender_copy = self.order_sender.clone();
    let mut snow =  uidservice::UidgenService::new(1, 1);
    let uid = snow.get_uid();
    let sender_value = MapSender{sender_id:uid,sender_copy:tx.clone()};
    let sender_value_copy = sender_value.clone();
    //let oc = self.order_channel.clone();
    tokio::spawn(async move {
      while let Some(result) = in_stream.next().await {
        match result {
          Ok(val) => {
            log::info!("{:#?}", val);
            let router_msg = val.clone();
            if val.msg_type == (Register as i32) {
              //注册
              if let Some(msg) = val.msg_content {
                if let Some(reg) = msg.register_req {
                  // if let Some (channel_ist) = {
                  for i in &reg.channel_id {
                    // log::info!("{:#?}", i);

                    BP_CACHE.entry(*i).or_insert(sender_value.clone());
                    let send_err = tx
                      .send(Result::<RouterMsg, Status>::Ok(RouterMsg {
                        msg_type: Response as i32,
                        msg_id: val.msg_id,
                        msg_content: None,
                        msg_time: current_timestamp(),
                      }))
                      .await;
                    if send_err.as_ref().is_err() {
                      log::error!("send bp error:{:#?}", send_err.err().unwrap().to_string());
                    }
                  }
                  //}
                }
              }
            } else if val.msg_type == (Exec as i32) {
              //订单执行回报
              log::info!("order exec :{:#?}", router_msg);
              let ret = order_sender_copy.send(router_msg).await;
              if ret.as_ref().is_err() {
                log::error!("send bp error:{:#?}", ret.err().unwrap().to_string());
              }
              //    let send_err =  tx.send(Result::<RouterMsg, Status>::
              //     Ok(RouterMsg {
              //     msg_type: Response as i32,
              //     msg_id:val.msg_id,
              //     msg:None,
              //     msg_time: current_timestamp()
              // })).await;
              // if send_err.as_ref().is_err(){
              //     log::error!("send bp error:{:#?}",send_err.err().unwrap().to_string());
              //    }
              // order_array.write().await.push(router_msg);

              // order_channel.0.send(router_msg).await;
            }
            // let ret = con.order_transfer(router_msg).await;
            // tx.send(ret).await.expect("tx error")
          }
          Err(err) => {
            // if let Some(io_err) = match_for_io_error(&err) {
            //     if io_err.kind() == ErrorKind::BrokenPipe {
            //         eprintln!("client disconnected: broken pipe");
            //         break;
            //     }
            // }
            if let Some(io_err) = match_for_io_error(&err) {
              if io_err.kind() == ErrorKind::BrokenPipe {
                // here you can handle special case when client
                // disconnected in unexpected way
                eprintln!("\tclient disconnected: broken pipe");
                for value in BP_CACHE.iter() {
                  log::info!("=>key:{:#?}", value.key());
                  if value.value().sender_id == sender_value_copy.sender_id{
                      //删除map中的值
                      BP_CACHE.remove(value.key());
                  }
              }

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
      log::info!("\tstream ended");
      return;
    });

    let out_stream = ReceiverStream::new(rx);
    Ok(Response::new(Box::pin(out_stream) as Self::OrderTransferStream))
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
