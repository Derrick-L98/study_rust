use std::collections::HashMap;

use anyhow::{anyhow, Result};

use sled::Db;
use sled::{open, Batch};

use crate::protofiles::hqmsg::YsHqInfo;

pub struct SledClient {
    db: Db,
}

impl SledClient {
    pub fn new(path: &String) -> Self {
        let db = open(path).expect("打开文件失败");
        Self { db }
    }

    pub async fn read_tick(&self) -> Vec<YsHqInfo> {
        let mut vec: Vec<YsHqInfo> = Vec::new();
        log::info!("文件读取tick数据....");
        for val in self.db.iter() {
            let _ret = val.map(|(k, v)| {
                let _key = format!("{}", String::from_utf8(k.to_vec()).unwrap());

                let str = String::from_utf8(v.to_vec()).unwrap();
                let val: YsHqInfo = serde_json::from_str(&str).expect("解析行情失败...");
                vec.push(val);
            });
        }
        log::info!("文件读取{}条tick数据", vec.len());
        vec
    }

    pub async fn batch_insert(&self, ticks: &HashMap<String, YsHqInfo>) -> Result<()> {
        let mut batch = Batch::default();
        for (key, val) in ticks.iter() {
            let value = serde_json::to_string(&val).unwrap(); //序列化为json字符串
            batch.insert(key.as_str(), value.as_str());
        }

        if let Err(err) = self.db.apply_batch(batch) {
            return Err(anyhow!("{}", err));
        }
        Ok(())
    }
}
