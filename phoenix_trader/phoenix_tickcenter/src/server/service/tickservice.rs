use anyhow::Result;
use dashmap::DashMap;
use chrono::{Datelike, Timelike};
use std::sync::Arc;


use utility::timeutil::current_naive_time;
use crate::{commonutil::commonutil::CommonUtil, protofiles::YsHqInfo};

#[derive(Clone, Debug)]
pub struct TickService {
    ticks_cache: Arc<DashMap<String, YsHqInfo>>,
}

impl TickService {
    pub fn new() -> Self {
        Self {
            ticks_cache: Arc::new(DashMap::new()),
        }
    }

    pub async fn init_tick_cache(&self, ticks: &Vec<YsHqInfo>) {
        for hqinfo in ticks.iter() {
            self.ticks_cache.insert(hqinfo.contract_no1.to_owned(), hqinfo.to_owned());
        }

        // ticks.iter().map(|&x| self.ticks_cache.insert(x.contract_no1.clone(), x));
    }

    pub async fn insert_ticks(&self, hqinfo: &mut YsHqInfo, common_util: &CommonUtil) -> Result<()> {
        //这里要判断是否需要处理，比如交易时间，行情时间等的判断
        //如果不需要处理，则返回error，处理成功，则返回ok
        // log::info!("insert_ticks begin...");
        // let now = Instant::now();
        let hq_time = hqinfo.tapidtstamp[11..19].to_owned();
        let ret = common_util.check_hqtick_time(&hq_time, &hqinfo.exchange_id).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("检查时间错误： {:?}", ret));
        }
        let (_ret_time, flag) = ret.unwrap();

        // let new_time = hqinfo.tapidtstamp[0..11].to_owned() + &ret_time; //yyyy-mm-dd 09:30:00
        let new_hq_time = hqinfo.tapidtstamp[11..19].to_owned();
        if flag < 0 {
            if (common_util.trade_area_index(&new_hq_time, &hqinfo.exchange_id).await).is_err() {
                return Err(anyhow!("该条tick不在交易时间内,不处理: code: {}, time: {}", hqinfo.contract_no1, hqinfo.tapidtstamp));
            }
        }
        self.ticks_cache.insert(hqinfo.contract_no1.to_owned(), hqinfo.to_owned());

        // log::info!("保存tick到缓存完成,用时: {:?}", now.elapsed());
        Ok(())
    }

    pub async fn query_ticks(&self, key: &Vec<String>) -> Vec<YsHqInfo> {
        let mut ret: Vec<YsHqInfo> = Vec::new();
        for val in key.iter() {
            let hqinfo = self.ticks_cache.get(val);
            if hqinfo.is_none() {
                continue;
            }
            ret.push(hqinfo.unwrap().value().clone());
        }
        ret
    }
    pub async fn query_tick(&self, key: &String) -> Option<YsHqInfo> {
        let hqinfo = self.ticks_cache.get(key);
        if hqinfo.is_none() {
            return None
        }
        Some(hqinfo.unwrap().value().to_owned())           
    }

    pub async fn insert_tick_cache(&self, hqinfo: &YsHqInfo) {
        self.ticks_cache.insert(hqinfo.contract_no1.to_owned(), hqinfo.to_owned());
    }

    pub async fn get_ticks(&self, stock_code: &String) -> Vec<YsHqInfo> {
        let mut ticks = Vec::new();
        let ticks_cache_clone = self.ticks_cache.clone();

        if stock_code.is_empty() {//获取所有
            for tick in ticks_cache_clone.iter() {
                ticks.push(tick.value().to_owned());
            }
        } else {
            if stock_code.contains(",") {
                let codes: Vec<&str> = stock_code.split(",").collect();
                for code in codes {
                    let tick = ticks_cache_clone.get(code);
                    if tick.is_none() {
                        log::error!("找不到{}的tick数据", code);
                        continue;
                    }
                    ticks.push(tick.unwrap().value().to_owned());
                } 
            } else {
                let tick = ticks_cache_clone.get(stock_code);
                if tick.is_none() {
                    log::error!("找不到{}的tick数据", &stock_code);
                } else {
                    ticks.push(tick.unwrap().value().to_owned());
                }
            }
        }

        ticks
    }

    pub async fn deadline(exchange: &String) -> i64 {
        let mut st = current_naive_time();

        if exchange == "US" || exchange == "XASE" || exchange == "XNYS" || exchange == "XNAS" {
            //美股
            if st.hour() >= 21 {
                //明天的4:10
                st = st.with_day(st.day() + 1).unwrap().with_hour(5).unwrap().with_minute(10).unwrap().with_second(0).unwrap()
            } else {
                st = st.with_hour(5).unwrap().with_minute(10).unwrap().with_second(0).unwrap()
            }
        } else if exchange == "HK" || exchange == "XHKG" {
            //港股
            st = st.with_hour(16).unwrap().with_minute(25).unwrap().with_second(0).unwrap()
        } else if exchange == "HS" || exchange == "XSHE" || exchange == "XSHG" {
            //沪深
            st = st.with_hour(15).unwrap().with_minute(2).unwrap().with_second(0).unwrap()
        }
        st.timestamp()
    }
}
