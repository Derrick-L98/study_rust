use config::{Config, ConfigError, File};
// use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;
// use std::sync::mpsc::channel;
// use std::time::Duration;
// use std::env;
#[derive(Debug, Clone, Deserialize)]
pub struct Application {
  pub apphost: String,
  pub appport: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
  pub uri: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisUri {
  pub uri: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Notification {
  pub amqpaddr: String,
  pub exchanger: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct System {
  pub adminserver: String,
  pub wtserver: String,
  pub cachelong: u64,
  pub cacheshort: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
  pub application: Application,
  pub database: Database,
  pub system: System,
  pub notification: Notification,
  pub redis: RedisUri,
  // pub orders: Orders, /*订单的MQ*/
}

// lazy_static! {
//     pub static ref SETTINGS: Settings = Settings::new().unwrap();
// }

impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
    // let config_file = "config/riskcenter";
    let s = Config::builder()
      // Start off by merging in the "default" configuration file
      .add_source(File::with_name("config/riskcenter"))
      // Add in the current environment file
      // Default to 'development' env
      // Note that this file is _optional_
      //.add_source(File::with_name(&format!("examples/hierarchical-env/config/{}", run_mode)).required(false))
      // Add in a local configuration file
      // This file shouldn't be checked in to git
      //.add_source(File::with_name("examples/hierarchical-env/config/local").required(false))
      // Add in settings from the environment (with a prefix of APP)
      // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
      //.add_source(Environment::with_prefix("app"))
      // You may also programmatically change settings
      //.set_override("database.url", "postgres://")?
      .build()
      .expect("build configuration error");

    // You can deserialize (and thus freeze) the entire configuration as
    s.try_deserialize()
  }

  // pub fn watch(&mut self) {
  //     // Create a channel to receive the events.
  //     let (tx, rx) = channel();
  //     // Automatically select the best implementation for your platform.
  //     // You can also access each implementation directly e.g. INotifyWatcher.
  //     let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(3)).expect("create configuration watch error");

  //     // Add a path to be watched. All files and directories at that path and
  //     // below will be monitored for changes.
  //     watcher
  //         .watch("config/riskcenter", RecursiveMode::NonRecursive)
  //         .expect("watch error");

  //     // This is a simple loop, but you may want to use more complex logic here,
  //     // for example to handle I/O.
  //     loop {
  //         match rx.recv() {
  //             Ok(DebouncedEvent::Write(_)) => {
  //                 println!(" * configuration written; refreshing configuration ...");
  //                 self.refresh().unwrap();
  //                 // show();
  //             }

  //             Err(e) => println!("watch error: {:?}", e),

  //             _ => {
  //                 // Ignore event
  //             }
  //         }
  //     }
  // }
}
