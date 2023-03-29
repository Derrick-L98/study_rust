use config::{Config, ConfigError, File};
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
  pub database: Database,
  pub application: Application,
  pub system: System,
  pub quotation: Quotation,
  pub messagecenter: Messagecenter,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
  pub uri: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Application {
  pub apphost: String,
  pub appport: i32,
  pub httpport: i32,
  pub machineid: i32,
  pub nodeid: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct System {
  pub persist_interval: u64,
  pub position_interval: u64,
  pub account_total: String,
  pub ignore_user_account: String, //过滤的用户账户
  pub ignore_fee_account: String,  //过滤费用的账户
  pub risk_restore: f64,           //风险率低于预警线多少点，恢复正常交易状态
  pub aka_server: String,
  pub assets_server: String,
  pub log_server: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Quotation {
  pub exchanger: String,
  pub amqpaddr: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Messagecenter {
  pub addr: String,      //消息中心的mq
  pub exchanger: String, //
  pub key: String,
}

impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
    let s = Config::builder()
      // Start off by merging in the "default" configuration file
      .add_source(File::with_name("config/accountriskcenter"))
      // // Add in the current environment file
      // // Default to 'development' env
      // // Note that this file is _optional_
      // .add_source(File::with_name(&format!("examples/hierarchical-env/config/{}", run_mode)).required(false))
      // // Add in a local configuration file
      // // This file shouldn't be checked in to git
      // .add_source(File::with_name("examples/hierarchical-env/config/local").required(false))
      // // Add in settings from the environment (with a prefix of APP)
      // // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
      // .add_source(Environment::with_prefix("app"))
      // // You may also programmatically change settings
      // .set_override("database.url", "postgres://")?
      .build()
      .expect("build config file error");

    // Now that we're done, let's access our configuration
    // println!("debug: {:?}", s.get_bool("debug"));
    // println!("database: {:?}", s.get::<String>("database.url"));

    // You can deserialize (and thus freeze) the entire configuration as
    // log::info!("configuration:{:?}", &s);
    // You can deserialize (and thus freeze) the entire configuration as
    s.try_deserialize()
  }
}
