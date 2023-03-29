// extern crate rand;
use crate::protofiles::logcenter::log_center_service_client::LogCenterServiceClient;
use crate::protofiles::logcenter::*;
use chrono::Local;
use local_ip_address::local_ip;
use std::sync::Arc;
use tokio::sync::RwLock;
// use logcenter::{log_center_service_client::LogCenterServiceClient, OmsLogMessage};
use once_cell::sync::{Lazy, OnceCell};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::{mem::MaybeUninit, sync::Once};
use tonic::transport::Channel;
// mod logcenter {
//   include!("./protofiles/logcenter.rs");
// }

pub static LOGCLIENT: OnceCell<LogClient> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct LogInfo {
  typ: i32,        // 日志类型：1-系统监控日志、2-业务日志
  lvl: i32,        // 日志等级DBG(debug):0; INF(info):1; WRN(warning):2; ERR(error):3-9;
  content: String, // 内容, json/text
}

#[derive(Copy, Clone)]
pub enum LogLevel {
  Debug,
  Info,
  Warning,
  Error,
}

// static DEFAULT_LOG_CENTER_URL: &str = "http://139.217.232.110:50090";
// // Create an uninitialized static
// static mut INITFLAG: bool = false;
// static mut SINGLETON: MaybeUninit<LogClient> = MaybeUninit::uninit();

#[derive(Debug, Clone)]
pub struct LogClient {
  // url: String, // logcenter地址
  sid: String, // 应用id
  app: String, // 应用名称
  client: Arc<RwLock<LogCenterServiceClient<Channel>>>,
}
impl LogClient {
  pub fn get() -> &'static LogClient {
    LOGCLIENT.get().expect("logclient is not inited")
  }

  pub async fn init(url: &String, app: &str) -> Result<LogClient, std::io::Error> {
    let client = LogCenterServiceClient::connect(url.to_owned()).await.expect("init error");
    let my_local_ip = local_ip().expect("get local ip error");
    Ok(LogClient {
      // url,
      sid: my_local_ip.to_string(),
      app: app.to_string(),
      client: Arc::new(RwLock::new(client)),
    })
  }

  // pub fn get() -> &'static LogClient {
  //   unsafe {
  //     if !INITFLAG {
  //       let mut appname = "unknow".to_string();
  //       match std::env::current_exe() {
  //         Ok(apppath) => {
  //           _ = apppath.file_name().and_then(|filename| filename.to_str()).and_then(|fnamestr| {
  //             appname = fnamestr.to_owned();
  //             None::<String>
  //           })
  //         }
  //         Err(e) => println!("failed to get current exe path:{}", e),
  //       };
  //       Self::init(DEFAULT_LOG_CENTER_URL.to_owned(), appname);
  //     }
  //     SINGLETON.assume_init_ref()
  //   }
  // }

  // 日志类型：2-业务日志
  // lvl 日志等级DBG(debug):0; INF(info):1; WRN(warning):2; ERR(error):3-9;
  // log 内容, json/text
  pub async fn push_error(&self, log: &str) {
    let request = tonic::Request::new(OmsLogMessage {
      log_sid: self.sid.clone(),
      srv_name: self.app.to_string(),
      log_type: 2,
      log_level: LogLevel::Error as i32,
      log_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
      log_content: log.to_string(),
    });

    let mut client_wr = self.client.write().await;
    let response = client_wr.push_log(request).await;
    if response.as_ref().is_err() {
      log::error!("push log failed:{:?}", response.as_ref().unwrap_err());
    }
  }

  // 日志类型：2-业务日志
  // lvl 日志等级DBG(debug):0; INF(info):1; WRN(warning):2; ERR(error):3-9;
  // log 内容, json/text
  pub async fn push(&self, lvl: LogLevel, log: &str) {
    let request = tonic::Request::new(OmsLogMessage {
      log_sid: self.sid.clone(),
      srv_name: self.app.to_string(),
      log_type: 2,
      log_level: lvl as i32,
      log_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
      log_content: log.to_string(),
    });
    // println!("{:?}", request);
    // let client = LogCenterServiceClient::connect(self.url.clone()).await;
    // if client.as_ref().is_err() {
    //   log::error!("connect to logcenter failed");
    // }
    let mut client_wr = self.client.write().await;
    let response = client_wr.push_log(request).await;
    if response.as_ref().is_err() {
      log::error!("push log failed:{:?}", response.as_ref().unwrap_err());
      // return Err("push log failed".to_string());
    }
    // log::info!("{:?}", response.unwrap().into_inner());
    // Ok(())
  }

  pub async fn push_log(&self, log: &LogInfo) {
    let request = tonic::Request::new(OmsLogMessage {
      log_sid: self.sid.clone(),
      srv_name: self.app.to_string(),
      log_type: log.typ,
      log_level: log.lvl,
      log_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
      log_content: log.content.to_owned(),
    });
    log::info!("{:?}", request);
    let mut client_wr = self.client.write().await;
    // let client = LogCenterServiceClient::connect(self.url.clone()).await;
    // if client.as_ref().is_err() {
    //   log::error!("connect to logcenter failed");
    // }
    let response = client_wr.push_log(request).await;
    if response.as_ref().is_err() {
      log::error!("push log failed:{:?}", response.as_ref().unwrap_err());
      // return Err("push log failed".to_string());
    }
    // log::info!("{:?}", response.unwrap().into_inner());
    // Ok(())
  }
}
