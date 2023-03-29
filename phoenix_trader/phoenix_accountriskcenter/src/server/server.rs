use super::controller::{PersistData, PhoenixController};
use super::service::accountassetsservice::PhoenixAccountAssetsService;
use super::service::accountstockpositionservice::PhoenixAccountStockPositionService;
use super::service::assetsservice::AssetsDataServie;
use super::service::basedataservice::BaseDataService;
use super::service::basiccacheservice::BasicCacheService;
use super::service::userassetsservice::UserAssetsService;
use super::service::userpositionservice::UserPositionService;
use crate::app::constdata;
use crate::config::settings::Settings;
use crate::dataservice::{dbsetup::DbConnection, entities::prelude::*};
use crate::protofiles::phoenixaccountriskcenter::account_risk_center_server::AccountRiskCenter;
use crate::protofiles::phoenixaccountriskcenter::{
  MarginRatioReq, MarginRatioResp, PhoenixAccountQueryRequest, PhoenixAccountResetRequest, PhoenixAssetsResponse, PhoenixStockPositionRequest, PhoenixStockPositionResponse, PhoenixTransferRequest, UserAssetsReq,
  UserAssetsResp, UserPositionReq, UserPositionResp, UserTotalAssetsResp,
};
use common::akaclient::AkaClient;
use dashmap::DashMap;
use messagecenter::notificationclient::NotificationClient;
use messagecenter::protofiles::hqmsg::YsHqInfo;
use messagecenter::protofiles::phoenixnotification::{NotificationMessage, NotificationType, OrderExecType};
use messagecenter::quotationclient::QuotationClient;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{pin::Pin, sync::Arc};
use tokio::sync::{mpsc, oneshot, RwLock};
use tonic::{self, Response};
use utility::errors::{self, ErrorCode};

// type StubType = Arc<RwLock<PhoenixController>>;
type StubType = Arc<PhoenixController>;
type ControllerAction = Box<dyn FnOnce(StubType) -> Pin<Box<dyn futures::Future<Output = ()> + Send>> + Send>;

pub struct ServerHandler {
  stub: StubType,
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

impl ServerHandler {
  pub async fn new(config: &Settings, dbconn: &DbConnection) -> Self {
    let mut persist_interval = tokio::time::interval(std::time::Duration::from_secs(config.system.persist_interval as u64));
    let mut position_interval = tokio::time::interval(std::time::Duration::from_secs(config.system.position_interval as u64));

    let (tx, mut rx) = mpsc::channel(1024);
    let (tx_persist, mut rx_persist) = mpsc::channel::<PersistData>(1024);
    let (tx_close, mut rx_close) = oneshot::channel();
    let (tx_quotation, mut rx_quotation) = tokio::sync::mpsc::channel::<YsHqInfo>(1024);
    let (tx_notification, mut rx_notification) = tokio::sync::mpsc::channel::<NotificationMessage>(1024);
    let (tx_assets, mut rx_assets) = tokio::sync::mpsc::channel::<i64>(16);

    let price_cache: Arc<DashMap<String, Decimal>> = Arc::new(DashMap::new());
    let mut atomic_last_price = AtomicBool::new(false);
    let mut atomic_assets_persist = AtomicBool::new(false);

    let assets_svc = AssetsDataServie::new().await;
    let basedata_svc = BaseDataService::new(config).await;
    let aka_svc = AkaClient::init(config.system.aka_server.to_string(), true, 200).await;
    let account_position_svc = PhoenixAccountStockPositionService::new();
    let account_assets_svc = PhoenixAccountAssetsService::new();
    let sys_info = PhoenixSysSystem::query(&dbconn).await.expect("query phoenix_sys_system info error");
    let ignore_accounts: Vec<i64> = config.system.ignore_user_account.split(',').map(|f| f.parse::<i64>().unwrap_or_default()).collect();
    let ignore_fee_accounts: Vec<i64> = config.system.ignore_fee_account.split(',').map(|f| f.parse::<i64>().unwrap_or_default()).collect();
    let userassetssvc = UserAssetsService::new();
    let userpositionsvc = UserPositionService::new();
    let basic_cache_svc = Arc::new(BasicCacheService::new());

    let stub = PhoenixController {
      dbconn: Arc::new(dbconn.to_owned()),
      assets_svc: Arc::new(assets_svc),
      basedata_svc: Arc::new(basedata_svc),
      account_position_svc: Arc::new(account_position_svc),
      account_assets_svc: Arc::new(account_assets_svc),
      aka_svc: Arc::new(aka_svc),
      setting: Arc::new(RwLock::new(config.clone())),
      sys_info,
      tx_persist,
      ignore_accounts,
      ignore_fee_accounts,
      user_assets_svc: Arc::new(userassetssvc),
      user_position_svc: Arc::new(userpositionsvc),
      basic_cache_svc,
      tx_assets,
    };

    //初始化数据
    stub.init().await.expect("init main controller error");

    //连接行情服务
    let routingkeys: HashMap<String, i32> = stub.get_stock_positions_quotation_key().await;
    // let mut routingkeys: HashMap<String, i32> = HashMap::new();
    // routingkeys.insert("stock.#".to_string(), 1);
    let quotation_queue_name = "phoenix_account_riskcenter_quotaion_queue";
    let quotation_client = QuotationClient::new(config.quotation.exchanger.as_str(), quotation_queue_name, routingkeys, &config.quotation.amqpaddr.as_str(), tx_quotation).await;
    //clone one, for rest if necessary
    let mut quotion_clone_client = quotation_client.clone();
    messagecenter::init::init_quotation_listen(quotation_client).await;

    //连接消息中心服务
    let queue_name = "phoenix_notification_client_queue";
    let notification_client = NotificationClient::new(&config.messagecenter.exchanger, queue_name, config.messagecenter.key.to_string(), &config.messagecenter.addr, tx_notification).await;
    // let mut notification_client_clone = notification_client.clone();
    messagecenter::init::init_notification_client(notification_client).await;

    let stub = Arc::new(stub);
    let stub_clone = stub.clone();
    let stub_for_dispatch = stub.clone();

    let ret = ServerHandler {
      task_dispacther: tx,
      set_close: Some(tx_close),
      stub,
    };

    tokio::spawn(async move {
      persist_interval.tick().await; //skip first tick
      position_interval.tick().await;
      loop {
        tokio::select! {
            may_task = rx.recv() => {
                log::info!("my task received...");
                if let Some(task) = may_task{
                    // log::info!("my task received...{:#?}",&task);
                    task(stub_for_dispatch.clone()).await;
                }
            }
            _ = position_interval.tick()=> {
              //定时计算浮动盈亏
              if *atomic_last_price.get_mut() {
                  //计算浮动盈亏，交易账户和分帐户
                  // log::info!("持仓最新价发生变化,计算持仓市值和盈亏等数据");
                  if let Ok(()) = stub_clone.update_assets_by_position().await{
                      atomic_last_price.store(false, Ordering::Relaxed);
                      if !*atomic_assets_persist.get_mut(){
                          atomic_assets_persist.store(true, Ordering::Relaxed);//可以保存到数据库
                      }
                  }
              }
            }
            _ = persist_interval.tick() => {
                 // 定时持久化数据,包括用户账户，分帐户的资产和持仓
                 if *atomic_assets_persist.get_mut() {
                    log::info!("持仓市值和盈亏等数据已经发生变化,定时保存到数据库");
                    if let Ok(()) = stub_clone.persist_data_interval().await{
                        atomic_assets_persist.store(false, Ordering::Relaxed);//数据持久化完成
                    }
                 }
            }
            rx_unitid = rx_assets.recv() =>{
              //重算用户资产信息
              if let Some(unit_id)=rx_unitid{
                atomic_assets_persist.store(false, Ordering::Relaxed);
                atomic_last_price.store(false, Ordering::Relaxed);
                let ret = stub_clone.re_calculate_user_assets(unit_id).await;
                if ret.is_ok() {
                  atomic_assets_persist.store(true, Ordering::Relaxed);
                  atomic_last_price.store(true, Ordering::Relaxed);
                }
              }

            }
            notification = rx_notification.recv() => {
              //包括：订单信息，资金调整消息
              // 1) 更新用户账户的资产数据和持仓数据
              // 2) 更新分账户的资产数据和持仓数据
              if let Some(message) = notification {
                if let Some(message_body) = message.msg_body.to_owned() {
                  match message.msg_type() {
                    NotificationType::AssetChanged => {
                      log::info!("资产变化（变化后的结果数据）");
                      if let Some(msg_asset) = &message_body.msg_asset {
                        // stub_clone.set_unit_assets(&msg_asset).await;
                        stub_clone.update_user_assets_by_notification(&msg_asset).await;
                      }
                    },
                    NotificationType::PositionChanged => {
                      log::info!("持仓变化");
                      if let Some(msg_position) = &message_body.msg_position {
                        // stub_clone.set_unit_positions(&msg_position).await;
                        stub_clone.update_user_position_by_notification(&msg_position).await;
                      }
                    },
                    NotificationType::OrderExecMsg  => {
                      log::info!("订单执行回报消息");
                      if let Some(order) = &message_body.msg_orderexec{
                        //更新资产和持仓信息
                        let ret = stub_clone.handle_assets_by_dealinfo(&order).await;
                        if order.exec_type == OrderExecType::NewOrder as i32 && ret.is_ok(){
                          //新订单，需要处理行情
                          let new_keys = stub_clone.get_stock_positions_quotation_key().await;
                          let _ = quotion_clone_client.update_bindings(&new_keys).await;
                        }
                      }
                    },
                    //收到：用户品种保证金比率变化消息，品种保证金变化消息，停牌消息，融资杠杆变化消息
                    //重新计算用户保证金变化
                    // NotificationType::用户品种保证金比率 =>{
                    //   stub_clone.check_margin_rate(unitid, stockid, channelid)
                    // }
                    _=>{log::error!("unhandled message type:{}", message.msg_type);}
                  }
                }else{
                  log::error!("message body is empty");
                }
              } else {
                log::error!("empty notification message...");
              }
            }
            quotation_task = rx_quotation.recv() => {
              // 接收到行情，需要处理：
              // 1)更新所有用户持仓的最新价，并计算浮动盈亏
              // 2)更新所有分账户持仓数据的最新价和浮动盈亏（主要是根据stockid和channelid）
              if let Some(quotation) = quotation_task {
                  // log::info!("1)接收到的行情信息, {} <==> 最新价:{}, 涨跌幅:{}, 行情时间:{:?} ,行情：{:?}", &quotation.contract_no1, quotation.q_last_price, quotation.q_change_rate, &quotation.tapidtstamp, &quotation);
                  let newval = Decimal::from_f64(quotation.q_last_price).unwrap_or_default();

                  let  mut val_price = Decimal::from(0 as i32);
                  let val_price_cache = price_cache.get(&quotation.contract_no1);
                  if val_price_cache.is_none(){
                      price_cache.insert((&quotation.contract_no1).clone(), newval);
                  }else{
                      val_price = val_price_cache.unwrap().value().clone();
                  }

                  if newval != val_price {
                    //计算用户账户的数据
                    let stockid = stub_clone.basic_cache_svc.get_stockid(&quotation.contract_no1,&stub_clone.aka_svc).await;
                    if stockid.is_ok(){
                    // stub_clone.set_stock_price(&quotation.commodity_no,&quotation.exchange_id,quotation.q_last_price).await;
                    //更新最新价，包括分帐户的用户账户
                    let _ = stub_clone.update_stock_positions_lastprice_by_quotation(stockid.unwrap(), &newval).await;
                    if !*atomic_last_price.get_mut(){
                        atomic_last_price.store(true, Ordering::Relaxed); //数据已经更新,可以重算资产
                    }
                  }
                  }
              }
            }
            persist_task = rx_persist.recv() => {
                log::info!("persist command received *************");
                if let Some(persist_data) = persist_task{
                    let _ = stub_clone.persist_data(&persist_data).await;
                }
            }
            _ = &mut rx_close => {
                log::info!("Server scheduler is notified to close");
                rx.close();
                break;
            }
        }
      }
      // drain unhandled task
      while let Some(task) = rx.recv().await {
        task(stub_for_dispatch.clone()).await;
        log::info!("drain unhandled task received");
      }

      log::warn!("Server scheduler has exited");
    });

    ret
  }
  pub fn on_leave(&mut self) -> ServerLeave {
    // self.stub.persist_to_database(constant::VALUE_ALL).await;
    ServerLeave(self.task_dispacther.clone(), self.set_close.take().expect("Do not call twice with on_leave"))
  }
}

//这里实现grpc的接口
#[tonic::async_trait]
impl AccountRiskCenter for ServerHandler {
  //查询用户风险率
  async fn query_margin_ratio(&self, request: tonic::Request<MarginRatioReq>) -> Result<tonic::Response<MarginRatioResp>, tonic::Status> {
    let req = request.into_inner();
    let ret = self.stub.query_margin_ratio(&req).await;
    let r = Response::new(ret.unwrap());
    Ok(r)
  }

  async fn query_user_positions(&self, request: tonic::Request<UserPositionReq>) -> Result<tonic::Response<UserPositionResp>, tonic::Status> {
    let req = request.into_inner();
    let ret = self.stub.query_user_positions(&req).await;
    let r = Response::new(ret.unwrap());
    Ok(r)
  }
  async fn query_user_toal_assets(&self, request: tonic::Request<UserAssetsReq>) -> Result<tonic::Response<UserTotalAssetsResp>, tonic::Status> {
    // let ControllerDispatch(act, rt) = ControllerDispatch::new(move |ctrl: &mut UnitServerController| Box::pin(async move { ctrl.query_channel_position(request.into_inner()).await }));
    // self.task_dispacther.send(act).await.map_err(map_dispatch_err)?;
    // map_dispatch_ret(rt.await)
    let result = UserTotalAssetsResp { ..Default::default() };
    let r = Response::new(result);
    Ok(r)
  }

  //查询用户资产
  async fn query_user_assets(&self, request: tonic::Request<UserAssetsReq>) -> Result<tonic::Response<UserAssetsResp>, tonic::Status> {
    let req = request.into_inner();
    let ret = self.stub.query_user_asset(&req).await;
    let r = Response::new(ret.unwrap());
    Ok(r)
  }

  async fn query_stock_positions(&self, request: tonic::Request<PhoenixStockPositionRequest>) -> Result<tonic::Response<PhoenixStockPositionResponse>, tonic::Status> {
    let req = request.into_inner();
    let ret = self.stub.query_stock_positions(&req).await;
    match ret {
      Ok(v) => {
        let res = PhoenixStockPositionResponse {
          ret_code: errors::get_error_code(ErrorCode::CodeOk).0,
          ret_msg: errors::get_error_code(ErrorCode::CodeOk).1,
          data: v,
          user_total_positions: 0,
        };
        log::info!("返回股票持仓信息:{:?}", &res);
        Ok(Response::new(res))
      }
      Err(e) => {
        let res = PhoenixStockPositionResponse {
          ret_code: errors::get_error_code(ErrorCode::CodeUnknown).0,
          ret_msg: e.to_string(),
          data: vec![],
          user_total_positions: 200,
        };
        log::info!("返回股票持仓信息:{:#?}", &res);
        Ok(Response::new(res))
      }
    }
  }

  async fn query_account_assets(&self, request: tonic::Request<PhoenixAccountQueryRequest>) -> Result<tonic::Response<PhoenixAssetsResponse>, tonic::Status> {
    let req = request.into_inner();
    log::info!("收到查找总账户风控信息的请求......{:?}", &req);
    // log::info!("query request: {:#?}", &req);
    let ret = self.stub.query_account_assets(&req).await;
    // log::info!("result of PhoenixAccountAssetsInfo:{:?}", &ret);
    match ret {
      Ok(v) => {
        let res = PhoenixAssetsResponse {
          ret_code: errors::get_error_code(ErrorCode::CodeOk).0,
          ret_msg: errors::get_error_code(ErrorCode::CodeOk).1,
          data: v,
        };
        log::info!("返回查找总账户资产的请求:{:#?}", &res);
        Ok(Response::new(res))
      }
      Err(e) => {
        let res = PhoenixAssetsResponse {
          ret_code: errors::get_error_code(ErrorCode::CodeUnknown).0,
          ret_msg: e.to_string(),
          data: vec![],
        };
        log::info!("返回查找总账户资产的请求:{:#?}", &res);
        Ok(Response::new(res))
      }
    }
  }

  ///资金划转
  async fn transfer_fund(&self, request: tonic::Request<PhoenixTransferRequest>) -> Result<tonic::Response<PhoenixAssetsResponse>, tonic::Status> {
    let req = request.into_inner();
    log::info!("收到资金划转的请求:{:#?}", &req);
    match self.stub.transfer_fund(&req).await {
      Ok(res) => Ok(Response::new(res)),
      Err(err) => {
        let res = PhoenixAssetsResponse {
          ret_code: errors::get_error_code(ErrorCode::CodeUnknown).0,
          ret_msg: err.to_string(),
          data: vec![],
        };
        Ok(Response::new(res))
      }
    }
  }

  ///分帐户reset动作
  async fn reset_profit(&self, request: tonic::Request<PhoenixAccountResetRequest>) -> Result<tonic::Response<PhoenixAssetsResponse>, tonic::Status> {
    let req = request.into_inner();

    log::info!("请求reset_profit的账户ID:{}", &req.account_id);

    match self.stub.reset_profit(req.account_id, &req).await {
      Ok(_) => {
        let res = PhoenixAssetsResponse {
          ret_code: errors::get_error_code(ErrorCode::CodeOk).0,
          ret_msg: errors::get_error_code(ErrorCode::CodeOk).1,
          data: vec![],
        };
        log::info!("返回reset结果:{:?}", &res);
        Ok(Response::new(res))
      }
      Err(e) => {
        let res = PhoenixAssetsResponse {
          ret_code: errors::get_error_code(ErrorCode::CodeUnknown).0,
          ret_msg: e.to_string(),
          data: vec![],
        };
        log::info!("返回reset结果:{:?}", &res);
        Ok(Response::new(res))
      }
    }
  }
}
