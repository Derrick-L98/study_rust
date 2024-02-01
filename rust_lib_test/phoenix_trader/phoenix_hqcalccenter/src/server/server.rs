use anyhow::Result;
use tonic::Response;
use tokio::sync::RwLock;
use tokio::sync::{broadcast, mpsc, oneshot};

use std::pin::Pin;
use std::sync::Arc;
// use std::time::Instant;
use std::sync::atomic::{AtomicBool, Ordering};
use common::logclient::*;
use super::controller::TickCenterController;
use crate::commonutil::commonutil::CommonUtil;
use super::service::{
    common::KLineData,
    prelude::TickService,
    klineservice::KLineService,
};
use crate::client::{
    sledclient::SledClient,
    cassandraclient::CassandraClient,
    marketdataclient::MarketDataclient,
};
use crate::{
    config::settings::Settings, 
    protofiles::{
        hqmsg::YsHqInfo, 
        phoenixklinecenter::{
            FenShiResp, KLineHqRequest, 
            KLineDataResp,
        }, 
    }
};

type StubType = Arc<TickCenterController>;
// type StubType = TickCenterController;
type ControllerAction = Box<dyn FnOnce(StubType) -> Pin<Box<dyn futures::Future<Output = ()> + Send>> + Send>;

pub struct CalcServerHandler {
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

impl CalcServerHandler {
    pub async fn new(setting: &Settings) -> Result<CalcServerHandler> {
        //创建新的时间间隔，该时间间隔随时间间隔而变化,第一次滴答声立即结束
        let mut fill_interval = tokio::time::interval(std::time::Duration::from_secs(1_u64));
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(setting.system.cassinterval as u64));

        //创建通道
        let (tx, mut rx) = mpsc::channel(32);
        let (tx_close, mut rx_close) = oneshot::channel(); //创建一次性通道,用于发送单个值

        let (tx_tick, mut rx_tick) = broadcast::channel::<YsHqInfo>(setting.system.channelcap as usize);

        let (tx_kline, mut rx_kline) = mpsc::channel::<KLineData>(204800);

        // 1. 日志中心客户端初始化
        let logclient = LogClient::init(&setting.system.logserver, "phoenix_hqcalccenter").await.expect("init logclient error");
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
        let tick_ctl = Arc::new(TickService::new());
        let common = CommonUtil::new();
        common.init(&setting).await.expect("init time error");

        //K线相关
        let kline = KLineService::new(tx_kline);
        let kline_ctl = Arc::new(kline);

        let fenshis = sledclient.read_fenshi().await;
        let stub = TickCenterController {
            tick_ctl,
            common_util: common,
            cassandra_client: arc_cassandra_client,
            sledclient: Arc::new(sledclient),
            //以下是K线相关
            fenshi: Arc::new(dashmap::DashMap::new()),
            kline_ctl,
            klines_cache: Arc::new(RwLock::new(Vec::new())),
            exchange_contract: Arc::new(dashmap::DashMap::new()),
        };
        stub.init_center_cache(&fenshis).await;
        log::info!("local cache init successfully");
        drop(fenshis);
        let stub = Arc::new(stub);
        let stub_for_dispatch = stub.clone();

        let stub_clone = stub.clone();

        let ret = CalcServerHandler {
            stub: stub.clone(),
            settings: setting.clone(),
            task_dispacther: tx,
            set_close: Some(tx_close),
        };
        
        let mut makclient = MarketDataclient::new(
            &setting.system.quotationserver, 
            &setting.system.exchangeno, 
            tx_tick
        ).await;
        let mut retry_interval = tokio::time::interval(std::time::Duration::from_secs(3));
        tokio::spawn(async move {
            retry_interval.tick().await; //系统启动, 等时间间隔后执行(不加, 先执行)
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

        let mut atomic_kline = AtomicBool::new(false);
        tokio::spawn(async move {
            interval.tick().await;
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if *atomic_kline.get_mut() {
                            let ret = stub_clone.persist_klines().await;
                            if ret.as_ref().is_ok(){
                                atomic_kline.store(false, Ordering::Relaxed);
                            }else{
                                log::error!("persist to cassandra error");
                            }
                        }
                    }
                    kline_task = rx_kline.recv() => {
                        if let Some(kline) = kline_task {
                            if kline.period == 1 {//缓存分时数据
                                if let Err(e) = stub_clone.generate_time_line(&kline, true).await {
                                    log::error!("cache error:{:?}", &e);
                                }
                            }
                            if let Err(e) = stub_clone.insert_cache_klines(&kline).await {
                                log::error!("insert cache error:{:?}", &e);
                            }
                            if !*atomic_kline.get_mut() {
                                atomic_kline.store(true, Ordering::Relaxed);
                            }
                        }
                    }
                }
            }
        });

        tokio::spawn(async move {
            //在到达间隔中的下一个瞬间时完成。
            fill_interval.tick().await;
            loop {
                tokio::select! {
                    may_task = rx.recv() => {//接收信息
                        log::info!("task from client received...");
                        let task = may_task.expect("Server scheduler has unexpected exit");
                        task(stub_for_dispatch.clone()).await;
                    }
                    tick_task = rx_tick.recv() => {
                        if let Ok(mut tick) = tick_task{
                            if tick.exchange_id == "HS" || tick.exchange_id == "HK" || tick.exchange_id == "US" {
                                //不用处理
                                continue;
                            }
                            if let Err(_) =  stub.insert_ticks(&mut tick).await {
                                // log::error!("invalid ticks received... {:?}", err);
                            } else {
                                if let Err(err) = stub.deal_tick_range(&tick).await {
                                    log::error!("{:?}", err);
                                }
                            }
                        }
                    }
                    _ = fill_interval.tick() => {//定时任务(每秒执行一次)
                        let _ = stub.fill().await;
                        if let Err(err) = stub.persist_fenshi().await {
                            log::error!("{:?}", err);
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
impl crate::protofiles::phoenix_kline_center_server::PhoenixKlineCenter for CalcServerHandler {
    async fn get_last_kline_data(
        &self,
        request: tonic::Request<KLineHqRequest>,
    ) -> Result<tonic::Response<KLineDataResp>, tonic::Status> {
        log::info!("client connected from:{:?}", &request.remote_addr());
        let req = request.into_inner();
        match self.stub.get_last_kline_data(&req).await {
            Ok(res) => Ok(Response::new(res)),
            Err(_err) => {
                let res = KLineDataResp::default();
                log::info!("query reset：{:?}", &res);
                Ok(Response::new(res))
            }
        }
    }
    async fn get_generate_fenshi_hq(
        &self,
        request: tonic::Request<KLineHqRequest>,
    ) -> Result<tonic::Response<FenShiResp>, tonic::Status> {
        log::info!("client connected from:{:?}", &request.remote_addr());
        let req = request.into_inner();
        match self.stub.get_generate_fenshi_hq(&req).await {
            Ok(res) => Ok(Response::new(res)),
            Err(_err) => {
                let res = FenShiResp::default();
                log::info!("query reset：{:?}", &res);
                Ok(Response::new(res))
            }
        }
    }
    
}

