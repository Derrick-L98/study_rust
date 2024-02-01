use anyhow::{anyhow, Result};
use common::akaclient::AkaClient;
use dashmap::DashMap;

pub struct BasicCacheService {
  stockcodes: DashMap<String, i64>,             //String:600000_XSHG, i64:stockid
  user_stock_margin_rate: DashMap<String, f64>, //用户品种的保证金比例
}

impl BasicCacheService {
  pub fn new() -> Self {
    BasicCacheService {
      stockcodes: DashMap::new(),
      user_stock_margin_rate: DashMap::new(),
    }
  }

  pub async fn get_stockid(&self, code: &str, akasvc: &AkaClient) -> Result<i64> {
    let ret = self.stockcodes.get(code);
    if ret.as_ref().is_none() {
      let vec_key: Vec<&str> = code.clone().split("_").collect();
      if vec_key.len() != 2 {
        return Err(anyhow!("code error: {}", &code));
      }
      let akaret = akasvc.query_stock_info_by_code(vec_key[0], vec_key[1]).await;
      if akaret.is_none() {
        return Err(anyhow!("query from akaserver error"));
      }
      let stockinfo = akaret.unwrap();

      self.stockcodes.insert(code.to_string(), stockinfo.stock_id);
      return Ok(stockinfo.stock_id);
    }

    Ok(*ret.unwrap().value())
  }

  pub async fn set_user_stock_margin_rate(&self, unitid: i64, stockid: i64, mrate: f64) -> Result<()> {
    Ok(())
  }
}
