use crate::protofiles::{RegisterReq, RouterMsg};
use crate::{config::settings::Settings, protofiles::MsgContent};
// use crossbeam::channel::{Receiver, Sender};
use async_channel::{Receiver, Sender};
use std::pin::Pin;
// use tokio::sync::{mpsc, oneshot};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Request, Response, Status, Streaming};
use utility::timeutil::current_timestamp;
//#[derive(Clone)]
pub struct OrderRouterController {
  // pub mktclient: Arc<MarketDataclient>,
  //pub settings: Settings,
  // pub order_sender: Sender<RouterMsg>,
  // pub order_receiver: Receiver<RouterMsg>,
}

impl OrderRouterController {
  // async fn get_hq(&self, in_stream: Streaming<ContractMsg>) -> Result<YsHqInfo> {}

  pub async fn order_routing(&mut self, _req: Streaming<RouterMsg>) -> Result<RouterMsg, Status> {
    //  let result = RouterMsg {
    //      msg_type: 1,
    //      msg: None,
    //      msg_id: 1,
    //      msg_time:current_timestamp(),
    //  };
    //  let (tx, rx) = mpsc::channel(128);
    //
    // // type OrderRoutingStream = Pin<Box<dyn Stream<Item = Result<result, Status>> + Send>>;
    //  log::info!("收请求");
    //  tx.send(Result::<RouterMsg, Status>::Ok(result)).await;
    //  let out_stream = ReceiverStream::new(rx);
    // // Ok(Response::new(Box::pin(out_stream) as OrderRoutingStream))
    // Ok(rx)
    todo!()
  }

  pub async fn order_transfer(&self, _req: RouterMsg) -> Result<RouterMsg, Status> {
    todo!()
  }
}
