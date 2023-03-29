use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Application {
    pub ip: String,
    pub port: i32,
    pub machineid: i32, 
    pub nodeid: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MySql {
    pub uri: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Redis {
    pub uri: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RabbitMQ {
    pub amqpaddr: String,
    pub exchanger: String,
    pub queue: String,
    pub routingkey: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GrpcServer {
    pub riskcenter: String,
    pub orderrouter: Option<String>,
    pub assetscenter: String,
    pub akacenter: String,
    pub logserver: String,
    pub accountriskcenter: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct System {
    pub cachelong: i64,
    pub oppaccount: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub application: Application,
    pub mysql: MySql,
    pub redis: Redis,
    pub mq: RabbitMQ,
    pub system: System,
    pub grpcserver: GrpcServer,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            //配置文件路径
            .add_source(File::with_name("config/ordercenter"))
            .build()
            .expect("build config file error");

        config.try_deserialize()
    }
}