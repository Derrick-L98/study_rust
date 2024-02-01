use anyhow::Result;
use chrono::prelude::*;
use chrono::Timelike;
use dashmap::DashMap;
use std::fmt::Write;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

// use time::strptime;
use super::service::{
    common::{KLineData, KLineType},
    klineservice::KLineService,
    persistservice::PersistService,
    prelude::*,
};
use crate::client::{
    cassandraclient::CassandraClient,
    sledclient::SledClient,
};
use crate::commonutil::commonutil::CommonUtil;
use crate::protofiles::{FenShiResp, KLineDataResp, KLineHqRequest, YsHqInfo};
use utility::timeutil::{build_naive_date_time, current_naive_time, from_timestamp_to_naive_date_time};

//一天总分钟数
pub const DAY_TOTAL_MINUTES: i64 = 1440;
pub const DAY_TOTAL_SECONDS: i64 = 86400;
enum Week {
    Monday = 0,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
//0: 星期一
//1: 星期二
//2: 星期三
//3: 星期四
//4: 星期五
//5: 星期六
//6: 星期日
pub const WEEK_SATURDAY: i32 = 1 << 5;
pub const WEEK_TRADE_BEGIN: i32 = 1 << 0; //周一
pub const WEEK_SUNDAY: i32 = 1 << 6; //周日
                                     //非交易日
pub const WEEK_OUT_TRADE: i32 = WEEK_SATURDAY | WEEK_SUNDAY;

#[derive(Clone)]
pub struct TickCenterController {
    pub common_util: CommonUtil,
    pub tick_ctl: Arc<TickService>,
    pub sledclient: Arc<SledClient>,
    pub cassandra_client: Option<Arc<CassandraClient>>,

    //以下是K线相关
    pub fenshi: Arc<DashMap<String, String>>, //<key, val>
    pub kline_ctl: Arc<KLineService>,
    pub klines_cache: Arc<RwLock<Vec<KLineData>>>,
    pub exchange_contract: Arc<dashmap::DashMap<String, String>>,
}

impl TickCenterController {
    //文件存储分时数据
    pub async fn insert_local_db_fs(&self) -> Result<()> {
        let mut map: HashMap<String, String> = HashMap::new();
        let dash_map = self.fenshi.clone();
        for val in dash_map.iter() {
            map.insert(val.key().to_owned(), val.value().to_owned());
        }
        let now = std::time::Instant::now();
        let ret = self.sledclient.batch_insert_fs(&map).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("{}", ret.as_ref().err().unwrap().to_string()));
        }
        log::info!("分时数据写入本地缓存完成,数据量:{},用时:{:?}", map.len(), now.elapsed());
        Ok(())
    }

    //本地缓存
    pub async fn insert_ticks(&self, tick: &mut YsHqInfo) -> Result<()> {
        self.exchange_contract.insert(tick.contract_no1.clone(), tick.exchange_id.clone());
        self.tick_ctl.insert_ticks(tick, &self.common_util, &self.kline_ctl).await

        // Ok(())
    }

    ///用于批量cassandra存储K线
    pub async fn insert_cache_klines(&self, kline: &KLineData) -> Result<()> {
        let mut klines_write = self.klines_cache.write().await;
        klines_write.push(kline.clone());
        Ok(())
    }

    ///初始化缓存, 防止重启后数据丢失
    pub async fn init_center_cache(&self, fenshi: &HashMap<String, String>) {
        let st = current_naive_time();
        //20220823
        let current_date = format!("{:04}{:02}{:02}", st.year(), st.month(), st.day());
        for (key, val) in fenshi.iter() {
            //STOCK_FS:600000_XSHG|20220823
            let str: Vec<&str> = key.split("|").collect(); //[STOCK_FS:600000_XSHG, 20220823]
            if current_date <= str[1].to_string() {
                self.fenshi.insert(key.to_owned(), val.to_owned());
            } else {
                let _ = self.sledclient.db.remove(key).expect("remove db error");
            }
            let contract_no: Vec<&str> = str[0].split(":").collect();
            if contract_no[1].contains("XSHG") {
                self.exchange_contract.insert(contract_no[1].to_owned(), "XSHG".to_string());
            } else if contract_no[1].contains("XSHE") {
                self.exchange_contract.insert(contract_no[1].to_owned(), "XSHE".to_string());
            } else if contract_no[1].contains("XHKG") {
                self.exchange_contract.insert(contract_no[1].to_owned(), "XHKG".to_string());
            }
        }
    }

    pub async fn persist_klines(&self) -> Result<()> {
        let ret = self.insert_local_db_fs().await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("{}", ret.as_ref().err().unwrap().to_string()));
        }
        let mut klines = self.klines_cache.write().await;
        if klines.len() > 0 {
            let klines_clone = klines.clone();
            klines.clear();
            drop(klines);

            // log::info!("缓存的kline数据:{}", klines_clone.len());
            if self.cassandra_client.is_some() {
                let now = std::time::Instant::now();
                let ret = PersistService::insert_klines_into_cassandra(&klines_clone, &self.cassandra_client.as_ref().unwrap()).await;
                if ret.as_ref().is_err() {
                    log::error!("{:?}", ret);
                    return Err(anyhow!("{}", ret.as_ref().err().unwrap().to_string()));
                }
                log::info!("kline数据写入Cassandra完成,数据量:{},用时:{:?}", klines_clone.len(), now.elapsed());
            }
        }

        Ok(())
    }

    ///交易日结束时持久化
    pub async fn persist_fenshi(&self) -> Result<()> {
        let st = current_naive_time();
        let time = format!("{:02}:{:02}:{:02}", st.hour(), st.minute(), st.second());
        if time == "16:20:00" {//保存分时数据
            log::info!("{} 保存分时数据...", &time);
            let fenshis = self.sledclient.read_fenshi().await;

            if self.cassandra_client.is_some() {
                let now = std::time::Instant::now();
                let ret = PersistService::insert_fenshi_into_cassandra(&fenshis, &self.cassandra_client.as_ref().unwrap()).await;
                if ret.as_ref().is_err() {
                    log::error!("{:?}", &ret);
                    return Err(anyhow!("{}", &ret.as_ref().err().unwrap().to_string()));
                }
                log::info!("分时数据写入Cassandra完成,数据量:{}, 用时:{:?}", fenshis.len(), now.elapsed());
            }
        }
        Ok(())
    }

    pub async fn deal_tick_range(&self, hqinfo: &YsHqInfo) -> Result<()> {
        if let Err(err) = self.kline_ctl.deal_tick_range(&hqinfo, KLineType::Hq1Kline, self).await {
            log::error!("{:?}", err);
            return Err(anyhow!("{:?}", err));
        }
        if let Err(err) = self.kline_ctl.deal_tick_range(&hqinfo, KLineType::Hq5Kline, self).await {
            log::error!("{:?}", err);
            return Err(anyhow!("{:?}", err));
        }
        if let Err(err) = self.kline_ctl.deal_tick_range(&hqinfo, KLineType::Hq10Kline, self).await {
            log::error!("{:?}", err);
            return Err(anyhow!("{:?}", err));
        }
        if let Err(err) = self.kline_ctl.deal_tick_range(&hqinfo, KLineType::Hq30Kline, self).await {
            log::error!("{:?}", err);
            return Err(anyhow!("{:?}", err));
        }
        if let Err(err) = self.kline_ctl.deal_tick_range(&hqinfo, KLineType::Hq60Kline, self).await {
            log::error!("{:?}", err);
            return Err(anyhow!("{:?}", err));
        }
        if let Err(err) = self.kline_ctl.deal_tick_range(&hqinfo, KLineType::Hq24Kline, self).await {
            log::error!("{:?}", err);
            return Err(anyhow!("{:?}", err));
        }
        Ok(())
    }

    pub async fn fill(&self) {
        let naive_time = current_naive_time();
        let mut local_current_minutes = (naive_time.hour() * 60 + naive_time.minute()) as i64; //当前时间分钟数
        if local_current_minutes == 0 {
            local_current_minutes = 24 * 60;
        }

        if naive_time.second() == 52 {
            log::info!("Start fill {} minutes, current time {}", local_current_minutes, naive_time);
            if let Err(err) = self.kline_ctl.fill_kline_data(local_current_minutes, self).await {
                log::error!("{}", err);
            }
            log::info!("Finish fill {} minutes, current time {}\n", local_current_minutes, naive_time);
        }
    }

    //生成分时
    pub async fn generate_time_line(&self, kline_data: &KLineData, append: bool) -> Result<()> {
        let prefix = "STOCK_FS";
        let minutes = (kline_data.prev_minutes + kline_data.period) % 1440;
        let st = build_naive_date_time(&kline_data.tick_time);
        //年月日时分
        let time = format!("{:04}{:02}{:02}{:02}{:02}00", &st.year(), &st.month(), &st.day(), minutes / 60, minutes % 60);

        let mut key = format!("{}:{}|{:04}{:02}{:02}", prefix, kline_data.stock_code, &st.year(), &st.month(), &st.day());
        key = key.to_uppercase();
        let mut val = self.get_value(&key).await;

        if append && !val.is_empty() {
            let delta_data = format!("{:<.3}|{:<.3}|{}|{}+", kline_data.average_price, kline_data.last_price, kline_data.current_period_volume, time);
            let _ = write!(val, "{}", delta_data);
        } else {
            val = format!(
                "{:<.03}|{:<.03}|{:<.03}|{}|{}+",
                kline_data.pre_close_price, kline_data.average_price, kline_data.last_price, kline_data.current_period_volume, time
            );
        }
        // log::info!("缓存分时数据:{}, {}", &key, &val)
        //清除缓存中前一天的数据
        let mut naive_time = current_naive_time();
        let tt = naive_time.timestamp() - (60 * 60 * 24);
        naive_time = from_timestamp_to_naive_date_time(tt);

        let mut prev_key = format!("{}:{}|{:04}{:02}{:02}", prefix, kline_data.stock_code, &naive_time.year(), &naive_time.month(), &naive_time.day());
        prev_key = prev_key.to_uppercase();
        // log::info!("prev key: {}", &prev_key);
        if let Some(_) = self.fenshi.get(&prev_key) {
            let _ = self.fenshi.remove(&prev_key);
            if let Ok(_) = self.sledclient.db.get(&prev_key) {
                let _ = self.sledclient.db.remove(&prev_key);
            }
        }

        self.fenshi.insert(key, val);

        Ok(())
    }

    pub async fn get_value(&self, key: &String) -> String {
        let val = self.fenshi.get(key);
        if val.is_none() {
            log::error!("查询key fenshi value None: {}", key);
            return "".to_string();
        }
        val.unwrap().value().to_owned()
    }

    pub async fn get_contract_fen_shi_key(&self, code: &String) -> Result<String> {
        let mut st = current_naive_time();
        let ret = self.exchange_contract.get(code);
        if ret.as_ref().is_none() {
            log::error!("没有证券: {}", code);
            return Err(anyhow!("{:?}", &ret));
        }
        let exchange = ret.unwrap().value().to_owned();
        let ret = self.adjust_trade_day(&exchange).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", &ret);
            return Err(anyhow!("{:?}", &ret.as_ref().is_err().to_string()));
        }
        //20220823
        let mut trade_day = ret.unwrap();
        let year = trade_day[0..4].parse::<i32>().unwrap_or_default();
        let month = trade_day[4..6].parse::<i32>().unwrap_or_default();
        let day = trade_day[6..8].parse::<i32>().unwrap_or_default();
        let trade_week = 1 << TickCenterController::calculate_week_day(year, month, day);
        log::info!("trade_week: {} , {}", trade_week, trade_week & WEEK_OUT_TRADE);

        //周末修正到周五
        if (trade_week & WEEK_OUT_TRADE) != 0 {
            let mut add_day = trade_week / WEEK_SATURDAY;
            if day as u32 != st.day() {
                //已修正到昨天, 当前时间要多修正一天
                add_day += 1;
            }
            let mut tt = st.timestamp();
            tt -= add_day as i64 * DAY_TOTAL_SECONDS;
            st = from_timestamp_to_naive_date_time(tt);

            trade_day = format!("{:04}{:02}{:02}", &st.year(), &st.month(), &st.day());
            log::info!("获取的日期为: {}", &trade_day);
        }
        let key = format!("STOCK_FS:{}|{}", code, &trade_day);
        Ok(key.to_uppercase())
    }

    //根据时间判断合约正确交易日
    pub async fn adjust_trade_day(&self, exchange: &String) -> Result<String> {
        let mut st = current_naive_time();
        let ret = self.common_util.get_begin_time(exchange).await;
        if ret.as_ref().is_err() {
            log::error!("{}", ret.as_ref().is_err().to_string());
            return Err(anyhow!("error: {:?}", &ret.as_ref().is_err().to_string()));
        }
        let begintime = ret.unwrap().format("%H:%M:%S").to_string(); //09:30:00
        let current_time = format!("{:02}:{:02}:{:02}", st.hour(), st.minute(), st.second());
        log::info!("current_time: {}, begin_time: {}", &current_time, &begintime);
        let mut trade_day = String::default();
        //开盘前
        if current_time < begintime {
            let mut tt = st.timestamp();
            tt -= DAY_TOTAL_SECONDS; //修正到昨天
            st = from_timestamp_to_naive_date_time(tt);
            
            trade_day = format!("{:04}{:02}{:02}", &st.year(), &st.month(), &st.day());
        } else {
            //大于等于开始时间，不用修正
            trade_day = format!("{:04}{:02}{:02}", &st.year(), &st.month(), &st.day());
            //当前yyyymmdd
        }
        log::info!("查询日期: {}", &trade_day);
        Ok(trade_day)
    }

    pub fn calculate_week_day(y: i32, m: i32, d: i32) -> i32 {
        let mut y = y;
        let mut m = m;
        if m < 3 {
            //把一月和二月换算成上一年的十三月和十四月
            m += 12;
            y -= 1;
        }
        //0: 星期一
        //1: 星期二
        //2: 星期三
        //3: 星期四
        //4: 星期五
        //5: 星期六
        //6: 星期日
        (d + 2 * m + 3 * (m + 1) / 5 + y + y / 4 - y / 100 + y / 400) % 7
    }

    pub async fn get_last_kline_data(&self, req: &KLineHqRequest) -> Result<KLineDataResp> {
        log::info!("code: {}, type: {}", &req.contract_no, &req.kline_type);
        let mut kline_type: i32 = req.kline_type.parse().unwrap_or_default();
        if kline_type == 24 {
            kline_type = KLineType::Hq24Kline as i32; //1440
        }
        // 判断是否是交易时间
        // {}
        log::info!("获取{}最新{}分钟K线数据", &req.contract_no, kline_type);
        let ret = self.kline_ctl.query_kline(&req.contract_no, kline_type).await;
        if ret.is_none() {
            log::error!("未找到数据: {}", &req.contract_no);
            return Ok(KLineDataResp::default());
        }
        let kline_data = ret.unwrap();
        let current_time = current_naive_time();
        let curr_time = format!("{}", current_time);
        if kline_data.tick_time > curr_time {//午休: 已将当前K线时间修改为下午开盘时间
            log::error!("超前时间: {}, {}", &req.contract_no, &kline_data.tick_time);
            return Ok(KLineDataResp::default());
        }

        let kline_info = KLineDataResp {
            stock_code: kline_data.stock_code,
            tick_time: kline_data.tick_time,
            open_price: kline_data.open_price,
            close_price: kline_data.close_price,
            high_price: kline_data.high_price,
            low_price: kline_data.low_price,
            last_price: kline_data.last_price,
            average_price: kline_data.average_price,
            pre_close_price: kline_data.pre_close_price,
            current_period_volume: kline_data.current_period_volume,
            current_period_turnover: kline_data.current_period_turnover,
            period: kline_data.period,
            prev_minutes: kline_data.prev_minutes,
        };
        log::info!("get_last_kline_data {:?}", &kline_info);
        Ok(kline_info)
    }

    pub async fn get_generate_fenshi_hq(&self, req: &KLineHqRequest) -> Result<FenShiResp> {
        log::info!("获取{}分时数据", &req.contract_no);
        let ret = self.get_contract_fen_shi_key(&req.contract_no).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", &ret);
            return Err(anyhow!("error: {:?}", ret));
        }
        let key = ret.unwrap();
        log::info!("key: {}", &key);
        let val = self.get_value(&key).await;

        log::info!("get_generate_fenshi_hq val: {}", &val);
        Ok(FenShiResp { fenshi_hq: val })
    }
}
