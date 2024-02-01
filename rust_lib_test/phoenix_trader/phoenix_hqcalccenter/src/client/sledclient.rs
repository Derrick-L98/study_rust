
use sled::Db;
// use sled::Batch;
// use std::str::FromStr;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub struct SledClient {
    pub db: Db,
}

impl SledClient {
    pub fn new(path: &String) -> Self {
        let db = sled::open(path).expect("打开文件失败");
        // let cats = db.open_tree(b"cats")?;
        // let dogs = db.open_tree(b"dogs")?;
        Self { db }
    }

    pub async fn read_fenshi(&self) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        log::info!("文件读取分时数据....");
        for val in self.db.iter() {
            let _ret = val.map(|(k, v)| {
                let key = format!("{}", String::from_utf8(k.to_vec()).unwrap()).trim().to_string();
                let str = format!("{}", String::from_utf8(v.to_vec()).unwrap()).trim().to_string();
                // log::info!("{}-----------{}", key, str);
                map.insert(key, str)
            });
        }
        // let _ = self.db.clear();
        log::info!("文件读取{}条分时数据", map.len());
        map
    }

    pub async fn batch_insert_fs(&self, fenshis: &HashMap<String, String>) -> Result<()> {
        let mut batch = sled::Batch::default();
        for (key, val) in fenshis.iter() {
            // log::info!("key: {}, val: {}", &key, &val);
            // let value = serde_json::to_string(&val).unwrap(); //序列化为json字符串
            batch.insert(key.as_str(), val.as_str());
        }
        if let Err(err) = self.db.apply_batch(batch) {
            return Err(anyhow!("{}", err));
        }
        Ok(())
    }
}
