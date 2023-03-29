use config::{Config, ConfigError, File};
// use lazy_static::lazy_static;
use serde::Deserialize;
// use crate::dbsetup::{self, DbConnection};
#[derive(Debug, Clone, Deserialize)]
pub struct Database {
  pub uri: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Application {
  pub apphost: String,
  pub appport: String,
  pub machineid: i32,
  pub nodeid: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct System {
  pub akaserver: String,
  pub cachetime: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Redis {
  pub uri: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MessageCenter {
  pub amqpaddr: String,
  pub assets_exchange: String,
  pub routing_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
  pub database: Database,
  pub application: Application,
  pub redis: Redis,
  pub system: System,
  pub msgcenter: MessageCenter,
}

impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
    println!("数据初始化...");
    let s = Config::builder().add_source(File::with_name("config/assetscenter.toml")).build().expect("build configuration error");
    let ret = s.try_deserialize();
    return ret;
  }
}

// lazy_static! {
//   pub static ref GLOBAL_CONFIG: Settings = {
//     let global_config = Settings::new();
//     match global_config {
//       Err(err) => {
//         println!("{}配置读取错误", err);
//         return Settings {
//           database: Database { uri: "".to_string() },
//           application: ApplicationConfig { ..Default::default() },
//           redis: RedisConfig { uri: "".to_string() },
//           uidgen: UidgenConfig { machineid: 0, nodeid: 0 },
//           assetsmsg: AssetsmsgConfig { ..Default::default() },
//           akacenter: AkacenterConfig { ..Default::default() },
//         };
//       }
//       Ok(config) => config,
//     }
//   };
// }
