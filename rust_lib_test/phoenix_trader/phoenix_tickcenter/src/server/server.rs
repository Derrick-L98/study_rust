use anyhow::Result;
use std::pin::Pin;
use std::sync::Arc;
use tonic::Response;
use std::sync::atomic::{AtomicBool, Ordering};

use tokio::sync::RwLock;
use tokio::sync::{broadcast, mpsc, oneshot};
use common::logclient::*;

use super::service::errors;
use super::service::prelude::TickService;

use crate::config::settings::Settings;
use super::controller::TickCenterController;
use crate::commonutil::commonutil::CommonUtil;

use crate::client::{
    sledclient::SledClient,
    tickmqclient::TickMqClient,
    cassandraclient::CassandraClient,
    marketdataclient::MarketDataclient,
};

use crate::protofiles::{LastPriceReq, LastPriceResp, TickReq, TickResp, YsHqInfo};

type StubType = Arc<TickCenterController>;
// type StubType = TickCenterController;
type ControllerAction = Box<dyn FnOnce(StubType) -> Pin<Box<dyn futures::Future<Output = ()> + Send>> + Send>;

pub struct TickServerHandler {
    stub: StubType,
    pub settings: Settings,
    task_dispacther: mpsc::Sender<ControllerAction>,
    set_close: Option<oneshot::Sender<()>>,
}

pub struct ServerLeave(mpsc::Sender<ControllerAction>, oneshot::Sender<()>);
impl ServerLeave {
    pub async fn leave(self) {
        self.1.send(()).unwrap();
        self.0.closed().await;
    }
}

impl TickServerHandler {
    pub async fn new(setting: &Settings) -> Result<TickServerHandler> {
        let mut persist_interval = tokio::time::interval(std::time::Duration::from_secs(setting.system.cassinterval as u64));

        //创建通道
        let (tx, mut rx) = mpsc::channel(32);
        let (tx_close, mut rx_close) = oneshot::channel(); //创建一次性通道,用于发送单个值

        let (tx_tick, mut rx_tick) = broadcast::channel::<YsHqInfo>(setting.system.channelcap as usize);
        let mut do_rx_tick = tx_tick.subscribe();

        // 1. 日志中心客户端初始化
        let logclient = LogClient::init(&setting.system.logserver, "phoenix_tickcenter").await.expect("init logclient error");
        LOGCLIENT.set(logclient).expect("set global logclient error");
        //2. 写入日志的办法
        // LogClient::get().push_error("error message is founded......").await;

        //创建cassandra
        let arc_cassandra_client: Option<Arc<CassandraClient>>;
        if setting.cassandra.addr.is_empty() {
            arc_cassandra_client = None;
        } else {
            let cassandra_client = CassandraClient::new(&setting.cassandra).await.expect("cassandra连接出错..");
            arc_cassandra_client = Some(Arc::new(cassandra_client));
        }

        log::info!("start to init local cache");
        let sledclient = SledClient::new(&setting.system.filepath);
        let ticks = sledclient.read_tick().await;
        let tick_ctl = Arc::new(TickService::new());
        tick_ctl.init_tick_cache(&ticks).await;
        drop(ticks);
        log::info!("local cache init successfully");

        let common = CommonUtil::new();
        common.init(&setting).await.expect("init time error");

        let arc_mq_client: Option<Arc<TickMqClient>>;
        if setting.rabbitmq.amqpaddr.is_empty() {
            arc_mq_client = None;
        } else {
            let mq_client = TickMqClient::new(
                &setting.rabbitmq.exchanger,
                &setting.rabbitmq.exchanger_delay, 
                &setting.rabbitmq.amqpaddr
            ).await;
            arc_mq_client = Some(Arc::new(mq_client));
        }

        let stub = TickCenterController {
            tick_ctl,
            common_util: Arc::new(common),
            cassandra_client: arc_cassandra_client,
            tick_mq: arc_mq_client,
            sledclient: Arc::new(sledclient),
            ticks_cache: Arc::new(RwLock::new(Vec::new())),
        };

        let stub = Arc::new(stub);
        let stub_for_dispatch = stub.clone();

        let ret = TickServerHandler {
            stub: stub.clone(),
            settings: setting.clone(),
            task_dispacther: tx,
            set_close: Some(tx_close),
        };

        let mut makclient = MarketDataclient::new(&setting.system.quotationserver, &setting.system.exchangeno, tx_tick).await;
        let mut retry_interval = tokio::time::interval(std::time::Duration::from_secs(3));
        tokio::spawn(async move {
            retry_interval.tick().await;
            loop{
                tokio::select! {
                    _ = retry_interval.tick() => {
                        if let Err(err) = makclient.do_subscribe_market_data().await {
                            log::error!("can't subscribe market data......{:?}", err);
                            let _ = makclient.retry_do_subscribe_market_data().await;
                        }
                    }
                }
            }
        });

        let mut atomic_tick = AtomicBool::new(false);
        tokio::spawn(async move {
            //在到达间隔中的下一个瞬间时完成。
            // fill_interval.tick().await;
            persist_interval.tick().await;
            loop {
                tokio::select! {
                    may_task = rx.recv() => {//接收信息
                        log::info!("task from client received...");
                        let task = may_task.expect("Server scheduler has unexpected exit");
                        task(stub_for_dispatch.clone()).await;
                    }
                    _ = persist_interval.tick() => {
                        if *atomic_tick.get_mut(){
                            let ret = stub.persist_ticks().await;
                            if ret.as_ref().is_ok(){
                                atomic_tick.store(false, Ordering::Relaxed);
                            }else{
                                log::error!("persist to cassandra error");
                            }
                            // atomic_tick.store(false, Ordering::Relaxed);
                        }
                    }
                    tick_task_cache = do_rx_tick.recv()=>{
                        if let Ok(tick) = tick_task_cache{
                            if let Err(e) = stub.insert_cache(&tick).await{
                                log::error!("insert cache error:{:?}", &e);
                            }
                        }
                    }
                    tick_task = rx_tick.recv() => {
                        if let Ok(mut tick) = tick_task{
                            // log::info!("从channel收到tick消息, code: {}, time: {}", tick.contract_no1, tick.tapidtstamp);
                            //板块行情直接推送MQ,不需要接下来的处理
                            if tick.exchange_id == "HS" || tick.exchange_id == "HK" || tick.exchange_id == "US" {
                                if let Err(err) = stub.send_tick_to_mq(&tick).await {
                                    log::error!("send to mq error {:?}", err);
                                }
                                // log::info!("板块行情推送完成: {}", &tick.contract_no1);
                                continue;
                            }
                            if let Err(_) =  stub.insert_ticks(&mut tick).await {
                                if !*atomic_tick.get_mut(){
                                    // log::error!("invalid ticks received... {:?}", err);
                                    atomic_tick.store(true, Ordering::Relaxed); //数据已经更新,可以保存到cassandra
                                }
                            } else {
                                //把tick数据放入mq
                                if let Err(err) = stub.send_tick_to_mq(&tick).await {
                                    log::error!("send to mq error {:?}", err);
                                }

                                if !*atomic_tick.get_mut(){
                                    atomic_tick.store(true, Ordering::Relaxed); //数据已经更新,可以保存到cassandra
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

            while let Some(task) = rx.recv().await {
                task(stub_for_dispatch.clone()).await;
                log::info!("drain unhandled task received");
            }

            log::warn!("Server scheduler has exited");
        });

        Ok(ret)
    }

    pub fn on_leave(&mut self) -> ServerLeave {
        ServerLeave(self.task_dispacther.clone(), self.set_close.take().expect("Do not call twice with on_leave"))
    }
}

#[tonic::async_trait]
impl crate::protofiles::phoenix_tick_center_server::PhoenixTickCenter for TickServerHandler {
    async fn get_tick_hq(
        &self,
        request: tonic::Request<TickReq>,
    ) -> Result<tonic::Response<TickResp>, tonic::Status> {
        let req = request.into_inner();
        match self.stub.get_tick_hq(&req).await {
            Ok(res) => Ok(Response::new(res)),
            Err(_err) => {
                let res = TickResp {
                    tick_hq_info: vec![],
                };
                log::info!("返回query reset 结果：{:?}", &res);
                Ok(Response::new(res))
            }
        }
    }

    async fn get_last_price(&self, request: tonic::Request<LastPriceReq>) -> Result<tonic::Response<LastPriceResp>, tonic::Status> {
        let req = request.into_inner();
        match self.stub.get_stock_last_price(&req).await {
            Ok(res) => Ok(Response::new(res)),
            Err(_err) => {
                let res = LastPriceResp {
                    err_msg: errors::get_error_code(errors::ErrorCode::CodeUnknown).0,
                    err_code: errors::get_error_code(errors::ErrorCode::CodeUnknown).1,
                    data: vec![],
                };
                log::info!("返回query reset 结果：{:?}", &res);
                Ok(Response::new(res))
            }
        }
    }
}
