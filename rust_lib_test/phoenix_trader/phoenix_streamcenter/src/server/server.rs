// use anyhow::Result;
use futures::Stream;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, oneshot};

use common::logclient::*;
use super::controller::StreamCenterController;
use crate::client::marketdataclient::MarketDataclient;

use crate::config::settings::Settings;
use crate::protofiles::hqmsg::YsHqInfo;

use crate::protofiles::{CommodityMsg, ContractMsg, QryContractMsg};
use std::{pin::Pin, time::Duration};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

type YsHqStream = Pin<Box<dyn Stream<Item = Result<YsHqInfo, Status>> + Send>>;
type QryStream = Pin<Box<dyn Stream<Item = Result<QryContractMsg, Status>> + Send>>;

type StubType = Arc<StreamCenterController>;
type ControllerAction = Box<dyn FnOnce(StubType) -> Pin<Box<dyn futures::Future<Output = ()> + Send>> + Send>;

// #[derive(Copy)]
pub struct StreamServerHandler {
    stub: StubType,
    pub settings: Settings,
    task_dispacther: mpsc::Sender<ControllerAction>,
    set_close: Option<oneshot::Sender<()>>,
    tx_tick: broadcast::Sender<YsHqInfo>,
}

pub struct ServerLeave(mpsc::Sender<ControllerAction>, oneshot::Sender<()>);
impl ServerLeave {
    pub async fn leave(self) {
        self.1.send(()).unwrap();
        self.0.closed().await;
    }
}

impl StreamServerHandler {
    pub async fn new(setting: &Settings) -> StreamServerHandler {
        //创建新的时间间隔，该时间间隔随时间间隔而变化,第一次滴答声立即结束
        // let mut persist_interval = tokio::time::interval(std::time::Duration::from_secs(setting.system.cassinterval as u64));

        //创建通道
        let (tx, mut rx) = mpsc::channel(32);
        let (tx_close, mut rx_close) = oneshot::channel(); //创建一次性通道,用于发送单个值

        let (tx_tick, mut rx_tick) = broadcast::channel::<YsHqInfo>(setting.system.channelcap as usize);

        let mut mktclient = MarketDataclient::new(&setting.system.quotationserver, &setting.system.exchangeno, tx_tick.clone()).await;

        // 1. 日志中心客户端初始化
        let logclient = LogClient::init(&setting.system.logserver, "phoenix_tickcenter").await.expect("init logclient error");
        LOGCLIENT.set(logclient).expect("set global logclient error");
        //2. 写入日志的办法
        // LogClient::get().push_error("error message is founded......").await;
        let stub = StreamCenterController {};

        let stub = Arc::new(stub);
        let stub_for_dispatch = stub.clone();
        // let (tick_sender, tick_receiver) = mpsc::channel::<Result<YsHqInfo, Status>>(1280);

        let ret = StreamServerHandler {
            stub,
            settings: setting.clone(),
            task_dispacther: tx,
            set_close: Some(tx_close),
            tx_tick,
        };

        let mut retry_interval = tokio::time::interval(Duration::from_secs(3));
        tokio::spawn(async move {
            retry_interval.tick().await;
            loop{
                tokio::select! {
                    _ = retry_interval.tick() => {
                        if let Err(err) = mktclient.do_subscribe_market_data().await {
                            log::error!("can't subscribe market data......{:?}", err);
                            let _ = mktclient.retry_do_subscribe_market_data().await;
                        }
                    }
                }
            }
        });

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
                    _tick_task = rx_tick.recv() => {
                        //do nothing......
                        // log::info!("************** Ticks:{:?}",&tick_task);
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

        ret
    }

    pub fn on_leave(&mut self) -> ServerLeave {
        ServerLeave(self.task_dispacther.clone(), self.set_close.take().expect("Do not call twice with on_leave"))
    }
}

#[tonic::async_trait]
impl crate::protofiles::market_data_servers_server::MarketDataServers for StreamServerHandler {
    type SubscribeMarketDataStream = YsHqStream;
    async fn subscribe_market_data(&self, request: Request<Streaming<ContractMsg>>) -> Result<Response<Self::SubscribeMarketDataStream>, tonic::Status> {
        log::info!("client connected from:{:?}", &request.remote_addr());

        let (tx, rx) = mpsc::channel(204800);

        let mut tick_clone = self.tx_tick.subscribe();
        tokio::spawn(async move {
            while let Ok(tick) = tick_clone.recv().await {
                // log::info!("received ticks code: {},  time: {}", &tick.contract_no1, &tick.tapidtstamp);
                let ret = tx.send(Result::<YsHqInfo, Status>::Ok(tick.to_owned())).await;
                if ret.as_ref().is_err() {
                    log::error!("send error:{}", ret.as_ref().err().unwrap().to_string());
                    break;
                }
            }
        });

        let out_stream = ReceiverStream::new(rx);

        Ok(Response::new(Box::pin(out_stream) as Self::SubscribeMarketDataStream))
    }

    type QueryCommodityDataStream = QryStream;
    async fn query_commodity_data(&self, _request: Request<CommodityMsg>) -> Result<Response<Self::QueryCommodityDataStream>, tonic::Status> {
        // creating infinite stream with requested message
        let repeat = std::iter::repeat(QryContractMsg::default());
        let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(200)));

        // spawn and channel are required if you want handle "disconnect" functionality
        // the `out_stream` will not be polled after client disconnect
        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {
                        // item (server response) was queued to be send to client
                    }
                    Err(_item) => {
                        // output_stream was build from rx and both are dropped
                        break;
                    }
                }
            }
        });

        let output_stream = ReceiverStream::new(rx);

        Ok(Response::new(Box::pin(output_stream) as Self::QueryCommodityDataStream))
    }
}
