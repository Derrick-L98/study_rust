use std::collections::HashMap;

use crate::setting::Settings;

pub struct TimePair(pub String, pub String);

pub struct Config {
    hs: HashMap<String, Vec<TimePair>>,
    hk: HashMap<String, Vec<TimePair>>,
    us: HashMap<String, Vec<TimePair>>
}

impl Config {
    pub async fn new() -> Self{
        Config {
            hs: HashMap::new(),
            hk: HashMap::new(),
            us: HashMap::new(),
        }
    }

    pub async fn deal(time: Vec<String>) -> Vec<TimePair>{
        let mut vec = Vec::new();
        for i in time.iter() {
            let tmp: Vec<&str> = i.split("-").map(|s| s).collect();
            vec.push(TimePair(tmp[0].to_string(), tmp[1].to_string()))
        }
        vec
    }

    async fn m(key: &String, setting: &Settings) -> HashMap<String, Vec<String>> {
        if let Some(s) = setting.tradetime.get(key) {
            return s.to_owned();
        } else {
            println!("配置
            错误...");
            return HashMap::new();
        }
    }

    pub async fn config(&self, setting: &Settings) {
        let hs = Config::m(&"hs".to_string(), setting).await;
        let hk = Config::m(&"hk".to_string(), setting).await;
        let us = Config::m(&"us".to_string(), setting).await;

        println!("{:#?}", hs);
        println!("{:#?}", hk);
        println!("{:#?}", us);


    }

    
}