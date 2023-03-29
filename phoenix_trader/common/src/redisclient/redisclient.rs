use anyhow::{anyhow, Result};
use redis::RedisError;
use redis_cluster_async::Client;
use utility::timeutil;
// use rust_decimal::{prelude::*, Decimal};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct RedisClient {
  pool: Client,
}

impl RedisClient {
  pub async fn new(uri: &str) -> Result<Self> {
    let nodes: Vec<&str> = uri.split(",").map(|s| s).collect();
    if nodes.len() <= 0 {
      log::error!("配置文件{}有误,不能连接redis集群", &uri);
      return Err(anyhow!("配置文件{}有误,不能连接redis集群", &uri));
    }
    log::info!("Redis nodes: {:?}", &nodes);
    if let Ok(pool) = Client::open(nodes) {
      Ok(RedisClient { pool })
    } else {
      log::error!("redis初始化失败");
      Err(anyhow!("不能连接redis集群"))
    }
  }

  pub async fn get_value_by_hgetall(&self, key: &str) -> BTreeMap<String, String> {
    let mut ret: BTreeMap<String, String> = BTreeMap::new();

    if let Ok(mut conn) = self.pool.get_connection().await {
      let query: Result<BTreeMap<String, String>, RedisError> = redis::cmd("HGETALL").arg(key).query_async(&mut conn).await;

      if let Ok(query_ret) = query {
        ret = query_ret;
      }
    }

    ret
  }

  pub async fn get_value_by_get(&self, key: &str) -> String {
    let mut ret: String = String::from(""); //

    if let Ok(mut conn) = self.pool.get_connection().await {
      let query: Result<String, RedisError> = redis::cmd("GET").arg(key).query_async(&mut conn).await;
      if let Ok(query_ret) = query {
        ret = query_ret;
      }
    }
    ret
  }

  async fn get_value_by_hget(&self, key: &str, field: &str) -> String {
    let mut ret: String = String::from(""); // = BTreeMap::new();
    if let Ok(mut conn) = self.pool.get_connection().await {
      let query: Result<String, RedisError> = redis::cmd("HGET").arg(key).arg(field).query_async(&mut conn).await;
      if let Ok(query_ret) = query {
        ret = query_ret;
      }
    }
    ret
  }

  pub async fn get_values_by_lrange(&self, key: &str) -> Result<Vec<String>> {
    match self.pool.get_connection().await {
      Ok(mut conn) => {
        let query: Result<Vec<String>, RedisError> = redis::cmd("lrange").arg(key).arg(0).arg(-1).query_async(&mut conn).await;
        match query {
          Ok(ret) => Ok(ret),
          Err(e) => Err(anyhow!("Error: {:?}", e)),
        }
      }
      Err(e) => Err(anyhow!("Error: {:?}", e)),
    }
  }

  pub async fn set_str_value(&self, key: &str, timeout: i64, value: &str) -> Result<()> {
    let conn = self.pool.get_connection().await;
    match conn {
      Ok(mut c) => {
        let query: Result<String, RedisError> = redis::cmd("SETEX").arg(key).arg(timeout).arg(value).query_async(&mut c).await;
        if let Ok(_r) = query {
        } else {
          log::info!("set_str_value失败,{:?}", query);
          return Err(anyhow!("redis写入失败"));
        }
      }
      Err(e) => {
        log::info!("set_str_value失败：{:?}", e);
        return Err(anyhow!("redis写入失败"));
      }
    }
    return Result::Ok(());
  }

  pub async fn lock(&self, key: &str, ex: i32) -> i32 {
    let mut ret: i32 = 0;
    let conn = self.pool.get_connection().await;
    match conn {
      Ok(mut c) => {
        let query: Result<String, RedisError> = redis::cmd("SET").arg(key).arg(1).arg("ex").arg(ex).arg("nx").query_async(&mut c).await;
        if let Ok(_r) = query {
          ret = 1;
        } else {
          log::info!("redis加锁失败,{:?},{}", query, timeutil::current_timestamp());
        }
      }
      Err(e) => {
        log::info!("获取redis链接失败：{:?}", e);
      }
    }
    ret
  }

  pub async fn delele(&self, key: &str) {
    println!("{}", key);
    let conn = self.pool.get_connection().await;
    match conn {
      Ok(mut c) => {
        let query: Result<(), RedisError> = redis::cmd("DEL").arg(key).query_async(&mut c).await;
        if let Ok(_r) = query {
        } else {
          log::info!("redis删除失败,{:?}", query);
        }
      }
      Err(e) => {
        log::info!("获取redis链接失败：{:?}", e);
      }
    }
  }

  pub async fn get_values_by_lindex(&self, key: &str, index: i32) -> String{
    log::info!("{}", key);
    let mut ret: String = String::from("");
    let conn = self.pool.get_connection().await;
    match conn {
        Ok(mut c) => {
          let query: Result<String, RedisError> = redis::cmd("LINDEX").arg(key).arg(index).query_async(&mut c).await;
          if let Ok(query_ret) = query {
            ret = query_ret;
          }
        },
        Err(err) => {
          log::info!("获取redis链接失败：{:?}", err);
        },
    }
    ret
  }
}
