use std::collections::HashMap;
use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Application {
    pub apphost: String,
    pub appport: i32,
}

// #[derive(Debug, Clone, Deserialize)]
// pub struct Database {
//     pub uri: String,
// }
#[derive(Debug, Clone, Deserialize)]
pub struct CassandraConfig {
    pub addr: String,      //地址：data.metestsvr.com:11092
    pub username: String,  //用户名：cassdbuser
    pub password: String,  //密码：cassdbuser4321
    pub namespace: String, //
}

// #[derive(Debug, Clone, Deserialize)]
// pub struct RedisConfig {
//     pub password: String,//密码
//     pub redisaddrs: String,
//     pub redisdb: i32,
// }

// #[derive(Debug, Clone, Deserialize)]
// pub struct RabbitMQ {
//     pub amqpaddr: String,
//     pub exchanger: String,
// }

#[derive(Debug, Clone, Deserialize)]
pub struct System {
    pub exchangeno: String,
    pub quotationserver: String,
    pub logserver: String,
    pub cassinterval: i64,
    pub channelcap: i64,
    pub filepath: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub application: Application,
    pub system: System,
    pub cassandra: CassandraConfig,
    pub quotaiontime: HashMap<String, HashMap<String, Vec<String>>>,
    // pub database: Database,
    // pub rabbitmq: RabbitMQ,
    // pub redis: RedisConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/hqcalccenter"))
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

        s.try_deserialize()
    }
}
