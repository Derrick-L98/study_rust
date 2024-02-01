use std::vec;


use anyhow::{anyhow, Result};
use chrono::{offset::TimeZone, DateTime, Local, NaiveDateTime};
use common::akaclient::AkaClient;
use common::constant::{self, OperateType};
use common::protofiles::phoenixakacenter::MarketCloseInfo;
use dashmap::DashMap;
use moka::future::{Cache, CacheBuilder};
#[derive(Clone, Debug)] //市场临时休市时间缓存
pub struct CloseTimeCached {
  pub close_time_cache: DashMap<i64, CloseTimeTemp>,
  // server:String,
}
#[derive(Debug, Clone, Default)]
pub struct CloseTimeTemp {
  // pub id: i64, // id
  pub market_id: i64,
  pub start_time: DateTime<Local>, //休市开始时间
  pub end_time: DateTime<Local>,   //休市结束时间
  pub close_type: i32,             //休市类型，一般用于陆股通，1:表示陆股通，2:表示QFII
}
impl CloseTimeCached {
  pub fn new() -> Self {
    CloseTimeCached {
      close_time_cache: DashMap::new(),
      // server: server.to_string(),
    }
  }
  ////qfii:0(否) 1:是 closetype：1:陆股通 2:qfii
  pub async fn check_currenttime_is_closetime(&self, aka: &AkaClient, market_id: i64, market_type: i32, qfii: i32) -> bool {
    let current_time = Local::now();
    if let Ok(ret) = aka.query_market_close_info().await {
      for info in ret {
        let tm = CloseTimeCached::convert_to_closetime(&info);
        self.close_time_cache.insert(tm.market_id, tm);
      }
      log::info!("所有的临时休市时间信息:{:#?}", &self.close_time_cache);
      true;
    } else {
      log::error!("获取临时休市信息错误");
      false;
    }
    log::info!(
      "current time: {:?}, marketid: {}, qfii: {}, current close times: {:?}",
      &current_time,
      &market_id,
      qfii,
      &self.close_time_cache
    );
    //验证当前时间是否在休市时间，需要用到的参数：1)market 2:通道的qfii
    //如果market type是3的话，则需要判断QFII属性、//qfii:0(否) 1:是    closetype：1: 陆股通 2:qfii
    let close_type = qfii + 1;
    let closetimes: Vec<CloseTimeTemp> = self
      .close_time_cache
      .iter()
      .filter(|x| {
        if market_type == constant::MarketType::HS as i32 {
          x.market_id == market_id && x.close_type == close_type
        } else {
          x.market_id == market_id
        }
      })
      .map(|y| y.value().to_owned())
      .collect();
    log::info!("marketid:{},close times :{:?}", &market_id, &closetimes);
    if closetimes.len() <= 0 {
      return false;
    }

    let ret = closetimes.iter().find(|x| x.start_time < current_time && current_time < x.end_time);
    if ret.is_some() {
      log::info!("Find close time: {:?}", &ret.unwrap());
      return true;
    }

    false
  }
  fn convert_to_closetime(close_detail: &MarketCloseInfo) -> CloseTimeTemp {
    let mut ret = CloseTimeTemp::default();
    ret.market_id = close_detail.market_id;
    let start_naive = NaiveDateTime::parse_from_str(&close_detail.start_time, "%Y-%m-%dT%H:%M:%S").unwrap();
    let start_time: DateTime<Local> = Local.from_local_datetime(&start_naive).unwrap();
    let end_naive = NaiveDateTime::parse_from_str(&close_detail.end_time, "%Y-%m-%dT%H:%M:%S").unwrap();
    let end_time: DateTime<Local> = Local.from_local_datetime(&end_naive).unwrap();
    ret.start_time = start_time;
    ret.end_time = end_time;
    ret.close_type = close_detail.close_type;
    return ret;
  }

  pub fn update_closetime_from_notification(
    &mut self,
    marketid: i32,
    closetype: i32,
    notification: &str, //时间格式为 2020-12-01 09:30:00|2020-12-01 15:30:00,2020-12-02 09:30:00|2020-12-02 15:30:00
    operate: i32,
  ) -> Result<()> {
    let closetimes: Vec<&str> = notification.split(',').collect();
    if closetimes.len() <= 0 {
      return Ok(());
    }
    log::info!("Close times: {:?}", &closetimes);

    for val in closetimes {
      let st_end_time: Vec<&str> = val.split('|').collect();
      if st_end_time.len() != 2 {
        continue;
      }

      let str_splitter = "|";

      let mut str_datetime: String = marketid.to_string();
      str_datetime.push_str(str_splitter.clone());
      str_datetime.push_str(str_splitter.clone());
      str_datetime.push_str(st_end_time[0]);
      str_datetime.push_str(str_splitter.clone());
      str_datetime.push_str(st_end_time[1]);
      str_datetime.push_str(str_splitter.clone());
      str_datetime.push_str(closetype.to_string().as_str());

      let _ = self.update_closetime(str_datetime.as_str(), operate);
    }
    Ok(())
  }

  //string format: "49|101|2021-12-31 00:00:00|2022-01-01 00:00:00|1"
  pub fn update_closetime(&mut self, str: &str, operate: i32) -> Result<()> {
    //更新
    log::info!("checked close times: {:?},operate type: {}", &str, operate);
    let closetime = self.convert_string_to_closetime(&str);
    if closetime.is_err() {
      return Err(anyhow!("传入用于拆分的字符串{}不对,不能拆分成休市时间", &str));
    }
    let closetime = closetime.unwrap();
    if operate == OperateType::Insert as i32 {
      log::info!("insert new close time:{:?}", &closetime);
      self.close_time_cache.insert(closetime.market_id, closetime);
    } else if operate == OperateType::Edit as i32 {
      //do modify
      log::info!("edit current close time: {:?}", &closetime);
      for mut p in self.close_time_cache.iter_mut() {
        if p.market_id == closetime.market_id {
          p.start_time = closetime.start_time;
          p.end_time = closetime.end_time;
          p.close_type = closetime.close_type;
        }
      }
    } else if operate == OperateType::Delete as i32 {
      //do remove
      log::info!("delete close time: {:?}", &closetime);
      if let Some(mut val) = self.close_time_cache.iter().position(|x| x.market_id == closetime.market_id) {
        let k = val as i64;
        self.close_time_cache.remove(&k);
      }
    }

    log::info!("Updated close time: {:?}", &self.close_time_cache);

    Ok(())
  }
  //string format: "49|101|2021-12-31 00:00:00|2022-01-01 00:00:00|1"
  fn convert_string_to_closetime(&self, str: &str) -> Result<CloseTimeTemp> {
    let split_strings: Vec<&str> = str.split('|').collect();

    if split_strings.len() != 5 {
      log::error!("传入用于拆分的字符串{}不对,不能拆分成休市时间", &str);
      return Err(anyhow!("传入用于拆分的字符串{}不对,不能拆分成休市时间", &str));
    }

    let mut tm = CloseTimeTemp {
      market_id: 0,
      start_time: Local::now(),
      end_time: Local::now(),
      close_type: 0,
    };

    // if let Ok(ret) = split_strings[0].parse::<i64>() {
    //     tm.market_id = ret;
    // } else {
    //     log::error!("不能处理ID:{}", &split_strings[0]);
    //     return Err(anyhow!("不能处理ID:{}", &split_strings[0]));
    // }

    if let Ok(ret) = split_strings[1].parse::<i64>() {
      tm.market_id = ret;
    } else {
      log::error!("不能处理市场ID:{}", &split_strings[1]);
      return Err(anyhow!("不能处理市场ID:{}", &split_strings[1]));
    }
    if let Ok(ret) = Local.datetime_from_str(split_strings[2], "%Y-%m-%d %H:%M:%S") {
      tm.start_time = ret;
    } else {
      log::error!("不能处理休市开始时间:{}", &split_strings[2]);
      return Err(anyhow!("不能处理休市开始时间:{}", &split_strings[2]));
    }
    if let Ok(ret) = Local.datetime_from_str(split_strings[3], "%Y-%m-%d %H:%M:%S") {
      tm.end_time = ret;
    } else {
      log::error!("不能处理休市结束时间:{}", &split_strings[3]);
      return Err(anyhow!("不能处理休市结束时间:{}", &split_strings[3]));
    }

    if let Ok(ret) = split_strings[4].parse::<i32>() {
      tm.close_type = ret;
    } else {
      log::error!("不能处理休市时间类型:{}", &split_strings[4]);
      return Err(anyhow!("不能处理休市时间类型:{}", &split_strings[4]));
    }

    Ok(tm)
  }
}
