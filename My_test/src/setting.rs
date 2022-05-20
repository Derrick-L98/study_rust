use std::collections::HashMap;

use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub tradetime: HashMap<String, HashMap<String, Vec<String>>>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("default"))
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