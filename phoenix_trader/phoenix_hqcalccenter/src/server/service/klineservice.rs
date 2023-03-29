use anyhow::Result;
use std::sync::Arc;
use dashmap::DashMap;
use chrono::prelude::*;
use tokio::sync::mpsc::Sender;

use crate::protofiles::YsHqInfo;
use super::common::{KLineData, KLineType};
use utility::timeutil::{
    build_naive_date_time, 
    from_timestamp_to_naive_date_time
};
use crate::{
    commonutil::commonutil::CommonUtil, 
    server::controller::TickCenterController
};
// use crate::client::cassandraclient::CassandraClient;

pub const DAY_TOTAL_MINUTES: i64 = 1440;

//每个合约的最新处理时间，用于数据补全
//第一个为行情时间分钟,第二个为行情服务器时间戳
#[derive(Clone, Debug)]
pub struct TimePair{
    pub minute: i64, 
    pub timestamp: i64
}

#[derive(Clone, Debug)]
pub struct KLineService {
    kline_1: Arc<DashMap<String, KLineData>>,
    kline_5: Arc<DashMap<String, KLineData>>,
    kline_10: Arc<DashMap<String, KLineData>>,
    kline_30: Arc<DashMap<String, KLineData>>,
    kline_60: Arc<DashMap<String, KLineData>>,
    kline_day: Arc<DashMap<String, KLineData>>,
    tx_kline: Sender<KLineData>,
    pub contract_fill: Arc<DashMap<String, TimePair>>, //补数用
}

impl KLineService {
    pub fn new(tx_kline: Sender<KLineData>) -> Self {
        KLineService {
            kline_1: Arc::new(DashMap::new()),
            kline_5: Arc::new(DashMap::new()),
            kline_10: Arc::new(DashMap::new()),
            kline_30: Arc::new(DashMap::new()),
            kline_60: Arc::new(DashMap::new()),
            kline_day: Arc::new(DashMap::new()),
            contract_fill: Arc::new(DashMap::new()),
            tx_kline,
        }
    }
    pub async fn check_kline_is_none(&self, kline_type: KLineType, key: &String) -> bool {
        match kline_type {
            KLineType::Hq1Kline => self.kline_1.get(key).is_none(),
            KLineType::Hq5Kline => self.kline_5.get(key).is_none(),
            KLineType::Hq10Kline => self.kline_10.get(key).is_none(),
            KLineType::Hq30Kline => self.kline_30.get(key).is_none(),
            KLineType::Hq60Kline => self.kline_60.get(key).is_none(),
            KLineType::Hq24Kline => self.kline_day.get(key).is_none(),
        }
    }

    //插入新值到map中, prev_minutes根据tick时间来计算或直接给出
    async fn insert_kline_data(&self, tick: &YsHqInfo, kline_type: KLineType, common_util: &CommonUtil, prev_minutes: i32) -> Result<()> {
        let mut kline_data = KLineData::new().await;
        let curr_tick_time = build_naive_date_time(&tick.tapidtstamp);

        kline_data.tick_time = tick.tapidtstamp.clone();
        kline_data.stock_code = tick.contract_no1.clone();
        kline_data.period = kline_type as i32;

        kline_data.pre_close_price = tick.q_pre_closing_price; //昨收价
        kline_data.close_price = tick.q_last_price; //收盘价
        kline_data.last_price = tick.q_last_price; //最新价
        kline_data.average_price = tick.q_average_price; //均价

        // kline_data.open_price = tick.q_opening_price; //开盘价
        // kline_data.high_price = tick.q_high_price; //最高价
        // kline_data.low_price = tick.q_low_price; //最低价
        kline_data.current_period_volume += tick.q_last_qty;
        kline_data.current_period_turnover += tick.q_last_turnover;

        //交易开始分钟数
        let ret_begin_time = common_util.get_begin_time(&tick.exchange_id).await;
        if ret_begin_time.as_ref().is_err() {
            log::error!("{:?}", ret_begin_time);
            return Err(anyhow!("{:?}", ret_begin_time));
        }
        let begin_time = ret_begin_time.unwrap();
        kline_data.begin_minutes = (begin_time.hour() * 60 + begin_time.minute()) as i32;

        //交易结束分钟数
        let ret_end_time = common_util.get_close_time(&tick.exchange_id).await;
        if ret_end_time.as_ref().is_err() {
            log::error!("{:?}", &ret_end_time);
            return Err(anyhow!("{:?}", ret_end_time));
        }
        let end_time = ret_end_time.unwrap();
        kline_data.end_minutes = (end_time.hour() * 60 + end_time.minute()) as i32;

        if prev_minutes == -1 {
            //日K 或者9:30以后才有的行情
            if kline_type as i32 == 1440 {
                kline_data.prev_minutes = kline_data.begin_minutes; //最新已形成的周期
            } else {
                //09:30:00 or 13:00:00
                let ret_time = common_util.get_interval_begin_time(&tick.tapidtstamp[11..19].to_string(), &tick.exchange_id).await;
                if ret_time.as_ref().is_err() {
                    log::error!("{:?}", ret_time);
                    return Err(anyhow!("{:?}", ret_time));
                }
                let time = ret_time.unwrap();
                let mut begin_min = (time.hour() * 60 + time.minute()) as i32;
                if (kline_type as i32 == 60) && (begin_min == 13 * 60) && tick.exchange_id == "XHKG" {
                    begin_min -= 30;
                }
                //距离上一个周期的分钟数，如：10分钟K线，第15分钟此值为5
                let interval = ((curr_tick_time.hour() * 60 + curr_tick_time.minute()) as i32 - begin_min) % kline_type as i32;
                // log::info!("[insert_kline_data] {} kline 距离上一个周期的分钟数: {}", kline_data.period, interval);
                //上一个保存周期
                kline_data.prev_minutes = (curr_tick_time.hour() * 60 + curr_tick_time.minute()) as i32 - interval;
            }
        } else {
            kline_data.prev_minutes = prev_minutes;
        }

        if kline_type as i32 == 1440 {
            if tick.q_opening_price > 0.00 {
                //开盘价
                kline_data.open_price = tick.q_opening_price;
            } else {
                kline_data.open_price = tick.q_last_price;
            }
            if tick.q_high_price > 0.00 {
                //最高价
                kline_data.high_price = tick.q_high_price;
            } else {
                kline_data.high_price = tick.q_last_price;
            }
            if tick.q_low_price > 0.00 {
                //最低价
                kline_data.low_price = tick.q_low_price
            } else {
                kline_data.low_price = tick.q_last_price;
            }
        } else {
            //不是日K
            kline_data.open_price = tick.q_last_price;
            kline_data.high_price = tick.q_last_price;
            kline_data.low_price = tick.q_last_price;
        }

        if tick.q_last_qty == -1 as f64 {
            kline_data.is_fake = true;
            if kline_type as i64 != 1400 {
                kline_data.current_period_volume += 1.0;
            }
        } else {
            kline_data.is_fake = false;
        }

        match kline_type {
            KLineType::Hq1Kline => self.kline_1.insert(kline_data.stock_code.clone(), kline_data.to_owned()),
            KLineType::Hq5Kline => self.kline_5.insert(kline_data.stock_code.clone(), kline_data.to_owned()),
            KLineType::Hq10Kline => self.kline_10.insert(kline_data.stock_code.clone(), kline_data.to_owned()),
            KLineType::Hq30Kline => self.kline_30.insert(kline_data.stock_code.clone(), kline_data.to_owned()),
            KLineType::Hq60Kline => self.kline_60.insert(kline_data.stock_code.clone(), kline_data.to_owned()),
            KLineType::Hq24Kline => self.kline_day.insert(kline_data.stock_code.clone(), kline_data.to_owned()),
        };
        Ok(())
    }

    pub async fn updata_open_call_auction_kline_data(&self, kline_type: &KLineType, tick: &YsHqInfo) {
        // log::info!("update {} {}\n", &tick.contract_no1, &tick.tapidtstamp);
        let mut kline = match kline_type {
            KLineType::Hq1Kline => self.kline_1.get_mut(&tick.contract_no1).unwrap(),
            KLineType::Hq5Kline => self.kline_5.get_mut(&tick.contract_no1).unwrap(),
            KLineType::Hq10Kline => self.kline_10.get_mut(&tick.contract_no1).unwrap(),
            KLineType::Hq30Kline => self.kline_30.get_mut(&tick.contract_no1).unwrap(),
            KLineType::Hq60Kline => self.kline_60.get_mut(&tick.contract_no1).unwrap(),
            KLineType::Hq24Kline => self.kline_day.get_mut(&tick.contract_no1).unwrap(),
        };

        let kline_data = kline.value_mut();

        kline_data.tick_time = tick.tapidtstamp.clone();
        kline_data.close_price = tick.q_last_price;
        kline_data.last_price = tick.q_last_price;
        kline_data.average_price = tick.q_average_price;
        kline_data.pre_close_price = tick.q_pre_closing_price; //昨收价

        // kline_data.open_price = tick.q_opening_price; //开盘价
        // kline_data.high_price = tick.q_high_price; //最高价
        // kline_data.low_price = tick.q_low_price; //最低价

        kline_data.current_period_volume += tick.q_last_qty;
        kline_data.current_period_turnover += tick.q_last_turnover;

        if kline_data.current_period_volume == 0 as f64 {
            //成交量为0
            kline_data.open_price = tick.q_last_price;
            kline_data.high_price = tick.q_last_price;
            kline_data.low_price = tick.q_last_price;
        } else {
            if kline_data.high_price < tick.q_last_price {
                //K线 最高阶 < tick 最新价,则更新
                kline_data.high_price = tick.q_last_price;
            }
            if kline_data.low_price > tick.q_last_price {
                //K线 最低价 > tick 最新价, 更新
                kline_data.low_price = tick.q_last_price;
            }
        }
    }

    //更新K线过程
    async fn update_kline_data(&self, kline_data: &mut KLineData, tick: &YsHqInfo, is_same_period: bool) {
        // log::info!("update {} {}\n", &tick.contract_no1, &tick.tapidtstamp);
        kline_data.tick_time = tick.tapidtstamp.clone(); 
        kline_data.close_price = tick.q_last_price; //收盘价
        kline_data.last_price = tick.q_last_price; //最新价
        kline_data.average_price = tick.q_average_price; //均价
        kline_data.pre_close_price = tick.q_pre_closing_price; //昨收价

        // kline_data.open_price = tick.q_opening_price; //开盘价
        // kline_data.high_price = tick.q_high_price; //最高价
        // kline_data.low_price = tick.q_low_price; //最低价

        kline_data.current_period_volume += tick.q_last_qty;
        kline_data.current_period_turnover += tick.q_last_turnover;

        //不是1分钟, 是每天刚开始时候
        if is_same_period && (kline_data.period != 1) && (kline_data.prev_minutes == kline_data.begin_minutes) {
            if kline_data.is_fake {
                kline_data.open_price = tick.q_last_price;
                kline_data.high_price = tick.q_last_price;
                kline_data.low_price = tick.q_last_price;
                kline_data.is_fake = false;
            }
            if tick.q_opening_price > 0.00 {
                kline_data.open_price = tick.q_opening_price;
            }
            if tick.q_high_price > 0.00 {
                kline_data.high_price = tick.q_high_price;
            } else if kline_data.high_price < tick.q_last_price {
                kline_data.high_price = tick.q_last_price;
            }

            if tick.q_low_price > 0.00 {
                kline_data.low_price = tick.q_low_price;
            } else if kline_data.low_price > tick.q_last_price {
                kline_data.low_price = tick.q_last_price;
            }

            return;
        }

        if is_same_period {
            if kline_data.is_fake {
                kline_data.open_price = tick.q_last_price;
                kline_data.high_price = tick.q_last_price;
                kline_data.low_price = tick.q_last_price;
                kline_data.is_fake = false;
            } else {
                if kline_data.high_price < tick.q_last_price {
                    kline_data.high_price = tick.q_last_price;
                }
                if kline_data.low_price > tick.q_last_price {
                    kline_data.low_price = tick.q_last_price;
                }
            }
        } else {
            //跨周期了, 全部更新
            kline_data.prev_minutes = (kline_data.prev_minutes + kline_data.period) % 1440;

            kline_data.open_price = tick.q_last_price;
            kline_data.high_price = tick.q_last_price;
            kline_data.low_price = tick.q_last_price;
            if tick.q_last_qty == -1 as f64 {
                kline_data.is_fake = true;
                kline_data.current_period_volume += 1 as f64;
            } else {
                kline_data.is_fake = false;
            }
        }
    }

    pub async fn insert_or_update(&self, tick: &YsHqInfo, kline_type: KLineType, common_util: &CommonUtil) {
        if self.check_kline_is_none(kline_type, &tick.contract_no1).await {
            if kline_type as i64 == 1440 {
                self.insert_kline_data(tick, kline_type, common_util, -1).await.unwrap();
            } else {
                if kline_type as i64 == 1 {
                    // yyyy-mm-dd 09:29:59
                    self.insert_kline_data(tick, kline_type, common_util, 569).await.unwrap();
                } else {
                    // yyyy-mm-dd 09:30:00
                    self.insert_kline_data(tick, kline_type, common_util, 570).await.unwrap();
                }
            }
        } else {
            self.updata_open_call_auction_kline_data(&kline_type, tick).await;
        }
    }

    // 对TICK数据分段处理
    pub async fn deal_tick_range(&self, tick: &YsHqInfo, kline_type: KLineType, center: &TickCenterController) -> Result<()> {
        if let Some(mut kline) = match kline_type {
            KLineType::Hq1Kline => self.kline_1.get_mut(&tick.contract_no1),
            KLineType::Hq5Kline => self.kline_5.get_mut(&tick.contract_no1),
            KLineType::Hq10Kline => self.kline_10.get_mut(&tick.contract_no1),
            KLineType::Hq30Kline => self.kline_30.get_mut(&tick.contract_no1),
            KLineType::Hq60Kline => self.kline_60.get_mut(&tick.contract_no1),
            KLineType::Hq24Kline => self.kline_day.get_mut(&tick.contract_no1),
        } {
            // log::info!("prev tick: {} {}", &kline.stock_code, &kline.tick_time);
            // log::info!("current tick: {} {}\n", &tick.contract_no1, &tick.tapidtstamp);
            let kline_data = kline.value_mut(); //上一条K线数据

            //当前tick分钟
            let naive_time = build_naive_date_time(&tick.tapidtstamp);

            let current_tick_min = (naive_time.hour() * 60 + naive_time.minute()) as i32;

            //前一条数据时间
            // let prev_tick_time = kline_data.tick_time.clone();
            // let ret = build_naive_date_time(&prev_tick_time);
            // if ret.as_ref().is_err() {
            //     log::error!("error: {:?}", ret);
            //     return Err(anyhow!("error: {:?}", ret));
            // }
            // let prev_time = ret.unwrap();
            // let prev_tick_min = (prev_time.hour() * 60 + prev_time.minute()) as i32;
            // log::info!("current tick data minutes: {}, prev_tick data minutes: {}", current_tick_min, prev_tick_min);

            if kline_data.period == 1440 {
                self.update_kline_data(kline_data, tick, true).await;
            } else {
                //当前与上一周期的时间间隔
                let mut diff_minutes = if current_tick_min > kline_data.prev_minutes {
                    current_tick_min - kline_data.prev_minutes
                } else {
                    0
                };

                if diff_minutes < kline_data.period {
                    //[0,period)本周期内, 更新
                    self.update_kline_data(kline_data, tick, true).await;
                } else if diff_minutes == kline_data.period {
                    //跨1、5、10.。。。分钟了，整理上一周期的数据，存入K线数据中存入Cassandra
                    if let Err(err) = self.tx_kline.send(kline_data.to_owned()).await {
                        log::error!("send error: {:?}", err);
                        return Err(anyhow!("send error: {:?}", err));
                    }
                    kline_data.current_period_turnover = 0.0;
                    kline_data.current_period_volume = 0.0;
                    self.update_kline_data(kline_data, tick, false).await;
                } else {
                    //diff_minutes > kline_data.period
                    //如果相差N(N>1)个周期, 启用补数据逻辑, 最多补到tick_minutes
                    log::info!("[deal_tick_range] {} 启用补数逻辑: 时间间隔:{}, {} kline", kline_data.stock_code, diff_minutes, kline_data.period);
                    let mut min = kline_data.prev_minutes + kline_data.period;
                    while min < current_tick_min + 1 {
                        if let Err(err) = self.deal_fill(kline_data, center).await {
                            return Err(anyhow!("{:?}", err));
                        }
                        min += kline_data.period;
                        diff_minutes -= kline_data.period;
                    }

                    //补完之后用当前tick更新
                    // log::info!("新的：{:?}", &tick);
                    self.update_kline_data(kline_data, tick, true).await;
                }
            }
        } else {
            //直接插入第一条数据
            // self.insert_kline_data(tick, kline_type, common_util, -1).await.unwrap();
            self.insert_kline_data(tick, kline_type, &center.common_util, -1).await.unwrap();
        }
        Ok(())
    }

    pub async fn deal_fill(&self, kline_data: &mut KLineData, center: &TickCenterController) -> Result<()> {
        let tt = build_naive_date_time(&kline_data.tick_time);

        let tt_fake_tick = tt.timestamp() + (60 * kline_data.period) as i64;
        let mut st_fake_tick = from_timestamp_to_naive_date_time(tt_fake_tick);

        if let Some(mut fake_tick) = center.tick_ctl.query_tick(&kline_data.stock_code).await {
            //补的tick 的交易量为0
            fake_tick.q_last_qty = 0.00;
            fake_tick.q_last_turnover = 0.00;
            // fake_tick.tapidtstamp = format!("{}.000", st_fake_tick);
            st_fake_tick = st_fake_tick.with_second(0).unwrap(); //更改秒数为0
            fake_tick.tapidtstamp = format!("{}", st_fake_tick);
            // fake_tick.tapidtstamp = format!("{:04}-{:02}-{:02} {:02}:{:02}:00.000", st_fake_tick.year(), st_fake_tick.month(), st_fake_tick.day(), st_fake_tick.hour(), st_fake_tick.minute());
            // log::info!("补数: {} {} prev {:02}:{:02}:00", &fake_tick.contract_no1, &fake_tick.tapidtstamp, kline_data.prev_minutes/60, kline_data.prev_minutes%60);

            // log::info!("保存上一周期：{:#?}", &kline_data);//上一周期的先保存
            if let Err(err) = self.tx_kline.send(kline_data.to_owned()).await {
                log::error!("send error: {:?}", err);
                return Err(anyhow!("send error: {:?}", err));
            }
            fake_tick.q_last_qty = -1 as f64; //设置为-1 标识此条tick为伪造的
            kline_data.current_period_turnover = 0.0;
            kline_data.current_period_volume = 0.0;
            // log::info!("补发假的一条tick：{:?}", &fake_tick);
            self.update_kline_data(kline_data, &fake_tick, false).await;
        }
        Ok(())
    }
 
    //补全K线数据
    pub async fn fill_kline_data(&self, local_current_minutes: i64, center: &TickCenterController) ->Result<()>{
        // log::info!("fill_kline_data start, fill size {}", self.contract_fill.len());
        //遍历所有合约,是否需要补
        let del_code: DashMap<String, String> = DashMap::new();
        for val in center.exchange_contract.iter() {
            let stock_code = val.key();
            if let Some(mut timepair) = self.contract_fill.get_mut(stock_code) {
                let timepair = timepair.value_mut();
                let mut local_deal_minutes = timepair.minute;
                //如果本地处理分钟小于本地当前分钟, 本地处理时间一分一分的累加, 并判断是否补分时K线
                //contract_fill中本地处理时间决定补多少个，tick时间决定补哪些时间的
                for _ in local_deal_minutes..local_current_minutes {
                    log::info!("处理分钟: {}, 当前: {}", local_deal_minutes, local_current_minutes);
                    local_deal_minutes = (timepair.minute + 1) % DAY_TOTAL_MINUTES;
                    let tt = timepair.timestamp + 60;//加1分钟,如午休,此时为11:29的时间戳,改为11:30
                    timepair.minute = local_deal_minutes;
                    timepair.timestamp = tt;    
                    //相当于来了一条tick_time的tick, 此时tick_time已加了一分钟
                    let mut tick_tm = from_timestamp_to_naive_date_time(tt);//11:30

                    tick_tm = tick_tm.with_second(0).unwrap(); //更改秒数为0
                    log::info!("fill_kline_data {} falsify tick {}", stock_code, tick_tm);
    
                    let ret = center.common_util.check_hqtick_time(&tick_tm.time().to_string(), &val.value().to_string()).await;
                    if ret.as_ref().is_err() {
                        return Err(anyhow!("{:?}", ret));
                    }
                    let (_, mut flag) = ret.unwrap();
                    let mut delay = 0;
                    if val.value() == "XHKG" || val.value() == "HK" {
                        delay = 10;
                    } else if val.value() == "XASE" || val.value() == "XNYS" || val.value() == "XNAS" || val.value() == "US" {
                        delay = 15;
                    }
                    if (flag == 3) && (local_deal_minutes < ((tick_tm.hour() * 60 + tick_tm.minute() + delay) % 1440).into()) && 
                    (val.value() == "XHKG" || val.value() == "HK" || val.value() == "XASE" || val.value() == "XNYS" || val.value() == "XNAS" || val.value() == "US") {
                        //闭市
                        //HK 16:25:00之前不补 US 04:30:00之前不补 一般都是 16:15 04:15 触发这里
                        //16:24 04:30
                        //15:59:59 03:59:59
                        timepair.minute = local_deal_minutes + (delay as i64) - 1;
                        timepair.timestamp = tick_tm.timestamp() - 1;
                        log::info!("fill_kline_data skip, {} waiting for close time reach", stock_code);
                        continue;
                    }
                    if val.value() == "XASE" || val.value() == "XNYS" || val.value() == "XNAS" || val.value() == "US" {
                        if flag != 3 {//不是闭市时间
                            //假设来了一条闭市时间的tick
                            let tick_time: String;
                            if let Some(tick) = center.tick_ctl.query_tick(&stock_code).await {
                                tick_time = tick.tapidtstamp;
                            } else {
                                log::info!("fill_kline_data {} has no previous tick, do not fill", stock_code);
                                continue;
                            }
                            let ret = center.common_util.get_close_time(val.value()).await;
                            if ret.as_ref().is_err() {
                                return Err(anyhow!("{:?}", ret));
                            }
                            let close_time = format!("{}", ret.unwrap());
                            let naive_date_time = build_naive_date_time(&(tick_time[0..11].to_string() + &close_time));

                            let tt = naive_date_time.timestamp();
                            tick_tm = from_timestamp_to_naive_date_time(tt);

                            flag = 3;
                        }
                    } else {
                        //不在交易时间范围内且不正好是结束时间, 
                        if let Err(_err) = center.common_util.trade_area_index(&tick_tm.time().to_string(), &val.value().to_string()).await {
                            //不在交易时间
                            if flag != 2 && flag != 3 {
                                //不是结束时间, 不是午休时间
                                log::info!("fill_kline_data skip, {} falsify tick {}", stock_code, tick_tm);
                                continue;
                            }
                        }
                    }
    
                    // log::info!("fake_tick_tm: {}", tick_tm);
                    if let Err(err) = self.fill_contract_data(KLineType::Hq1Kline, center, &stock_code, &tick_tm).await {
                        return Err(anyhow!("{:?}", err));
                    }
                    if let Err(err) = self.fill_contract_data(KLineType::Hq5Kline, center, &stock_code, &tick_tm).await {
                        return Err(anyhow!("{:?}", err));
                    }
                    if let Err(err) = self.fill_contract_data(KLineType::Hq10Kline, center, &stock_code, &tick_tm).await {
                        return Err(anyhow!("{:?}", err));
                    }
                    if let Err(err) = self.fill_contract_data(KLineType::Hq30Kline, center, &stock_code, &tick_tm).await {
                        return Err(anyhow!("{:?}", err));
                    }
                    if let Err(err) = self.fill_contract_data(KLineType::Hq60Kline, center, &stock_code, &tick_tm).await {
                        return Err(anyhow!("{:?}", err));
                    }
                    // self.fill_contract_data(KLineType::Hq24Kline, center, &stock_code, &tick_tm).await;
    
                    if flag == 3 {
                        //交易日结束
                        if let Err(err) = self.save_trade_end_data(stock_code, val.value()).await{
                            log::info!("{:?}", err);
                            return Err(anyhow!("{:?}", err));
                        }
                        // // 当天分时数据存入Cassandra
                        // if let Err(err) = center.persist_fenshi(&stock_code).await {
                        //     log::info!("{:?}", err);
                        //     return Err(anyhow!("{:?}", err));
                        // }
                        del_code.insert(stock_code.to_string(), val.value().to_string());
                        // log::info!("========================交易结束========================");
                        break; //如果是结束时间, 后面无论是否还有要补的, 都不补了
                    } else if flag == 2 {
                        //午休
                        //相当于在13:00:00来了一条13:00:00的tick
                        //下一个时间段的开始时间 如沪深的13:00:00
                        // log::info!("===== : {}", tick_tm.time());
                        let ret = center.common_util.get_next_interval_begin_time(&tick_tm.time().to_string(), &val.value()).await;
                        if ret.as_ref().is_err() {
                            return Err(anyhow!("{:?}", ret));
                        }
                        let time = ret.unwrap();
                        let btime = format!("{}", time);
                        // log::info!("获取时间: {}", &btime);
                        //年月日不变时分秒变为下一个时间段开始时间, 如沪深的13:00:00
                        if let Ok(hour) = btime[0..2].parse::<u32>() {
                            tick_tm = tick_tm.with_hour(hour).unwrap();
                        } else {
                            log::error!("取小时转换失败");
                        }
                        if let Ok(min) = btime[3..5].parse::<u32>() {
                            tick_tm = tick_tm.with_minute(min).unwrap();
                        } else {
                            log::error!("取分钟转换失败");
                        }
                        tick_tm = tick_tm.with_second(0).unwrap();
    
                        //将<localtime,ticktime>都设置为下一个时间段的开始时间 如沪深的13:00:00
                        timepair.minute = (tick_tm.hour() * 60 + tick_tm.minute()) as i64;
                        timepair.timestamp = tick_tm.timestamp();

                        // log::info!("设置为下一开始时间: {}", tick_tm);
                        self.set_next_save_minute(&center.exchange_contract, KLineType::Hq1Kline, &stock_code, &tick_tm).await;
                        self.set_next_save_minute(&center.exchange_contract, KLineType::Hq5Kline, &stock_code, &tick_tm).await;
                        self.set_next_save_minute(&center.exchange_contract, KLineType::Hq10Kline, &stock_code, &tick_tm).await;
                        self.set_next_save_minute(&center.exchange_contract, KLineType::Hq30Kline, &stock_code, &tick_tm).await;
                        self.set_next_save_minute(&center.exchange_contract, KLineType::Hq60Kline, &stock_code, &tick_tm).await;
                        // self.set_next_save_minute(&center.exchange_contract, KLineType::Hq24Kline, &stock_code, &tick_tm).await;
    
                        log::info!("fill_kline_data set {} save minutes {}", stock_code, tick_tm.hour() * 60 + tick_tm.minute());
                        break;
                    }
                }
            } else {
                continue;
            } 
        }

            if !del_code.is_empty() {
                //交易结束清除缓存
                for val in del_code.iter() {
                    self.contract_fill.remove(&val.key().to_owned());
                    self.kline_1.remove(&val.key().to_owned());
                    self.kline_5.remove(&val.key().to_owned());
                    self.kline_10.remove(&val.key().to_owned());
                    self.kline_30.remove(&val.key().to_owned());
                    self.kline_60.remove(&val.key().to_owned());
                    self.kline_day.remove(&val.key().to_owned());
                }
            }
        // }
            
        log::info!("fill_kline_data finish, fill size {}", self.contract_fill.len());
        Ok(())
    }

    // 午休时设置KLineDataType里的pre_minutes为下一交易时间段的开始时间13:00
    pub async fn set_next_save_minute(&self, map: &Arc<DashMap<String, String>>, kline_type: KLineType, stock_code: &String, tick_st: &NaiveDateTime) {
        if let Some(mut kline) = match kline_type {
            KLineType::Hq1Kline => self.kline_1.get_mut(stock_code),
            KLineType::Hq5Kline => self.kline_5.get_mut(stock_code),
            KLineType::Hq10Kline => self.kline_10.get_mut(stock_code),
            KLineType::Hq30Kline => self.kline_30.get_mut(stock_code),
            KLineType::Hq60Kline => self.kline_60.get_mut(stock_code),
            KLineType::Hq24Kline => self.kline_day.get_mut(stock_code),
        } {
            let kline_data = kline.value_mut();

            kline_data.tick_time = format!("{}", tick_st);
            kline_data.prev_minutes = (tick_st.hour() * 60 + tick_st.minute()) as i32;

            if kline_data.period == 60 && map.get(stock_code).unwrap().value() == "XHKG" {
                kline_data.prev_minutes -= 30;
            }
        }
    }
    
    async fn fill_contract_data(&self, kline_type: KLineType, center: &TickCenterController, stock_code: &String, fake_tick_tm: &NaiveDateTime) ->Result<()> {
        if let Some(mut kline) = match kline_type {
            KLineType::Hq1Kline => self.kline_1.get_mut(stock_code),
            KLineType::Hq5Kline => self.kline_5.get_mut(stock_code),
            KLineType::Hq10Kline => self.kline_10.get_mut(stock_code),
            KLineType::Hq30Kline => self.kline_30.get_mut(stock_code),
            KLineType::Hq60Kline => self.kline_60.get_mut(stock_code),
            KLineType::Hq24Kline => self.kline_day.get_mut(stock_code),
        } {
            let kline_data = kline.value_mut();
            let tick_minutes = (fake_tick_tm.hour() * 60 + fake_tick_tm.minute()) as i32;
            let mut diff_minutes = if tick_minutes > kline_data.prev_minutes { tick_minutes - kline_data.prev_minutes } else { 0 };

            //如果相差N(N>1)个周期, 启用补数据逻辑, 最多补到tick_minutes - 1
            // log::info!("开始===================1: {}, {}", kline_data.period, diff_minutes);
            if diff_minutes > kline_data.period {
                let mut min = kline_data.prev_minutes +  kline_data.period;
                while min < tick_minutes {
                    if let Err(err) = self.deal_fill(kline_data, center).await {
                        log::error!("{:?}", err);
                        return Err(anyhow!("{:?}", err));
                    }
                    min +=  kline_data.period;
                    diff_minutes -=  kline_data.period;
                }
            }
            //剩下最多一个周期
            if diff_minutes ==  kline_data.period {
                let fake_time = format!("{}", fake_tick_tm);
                let tt = build_naive_date_time(&fake_time);

                let tt_fake_tick = tt.timestamp() - (60 * kline_data.period) as i64;
                let ret = from_timestamp_to_naive_date_time(tt_fake_tick);

                kline_data.tick_time = format!("{}", ret);
                if let Err(err) = self.deal_fill(kline_data, center).await {
                    log::error!("{:?}", err);
                    return Err(anyhow!("{:?}", err));
                }
            }
        } else {
            log::error!("fill_contract_data {} has no kline data", stock_code);
            return Err(anyhow!("fill_contract_data {} has no kline data", stock_code));
        }
        Ok(())
    }

    //交易结束保存交易日分时数据/日K数据,清理合约均价信息
    pub async fn save_trade_end_data(&self, stock_code: &String, exchange: &String) ->Result<()>{
        //特殊处理港股的时K
        if exchange == "XHKG" || exchange == "HK" || exchange == "XASE" || exchange == "XNYS" || exchange == "XNAS" || exchange == "US" {
            if let Some(val) = self.kline_60.get(stock_code) {
                let mut kline_data = val.value().clone();
                kline_data.prev_minutes -= 30;
                if let Err(err) = self.tx_kline.send(kline_data.to_owned()).await {
                    log::error!("send error: {:?}", err);
                    return Err(anyhow!("send error: {:?}", err));
                }
            } else {
                return Err(anyhow!("{} has no data in the 60 kline map", stock_code));
            }
        }

        //生成日K存入Cassandra
        if let Some(val) = self.kline_day.get(stock_code) {
            let kline_data = val.value();
            // log::info!("Save day K Line {} => {}", stock_code, kline_data.trade_day);
            if let Err(err) = self.tx_kline.send(kline_data.to_owned()).await {
                log::error!("send error: {:?}", err);
                return Err(anyhow!("send error: {:?}", err));
            }
        } else {
            return Err(anyhow!("{} has no data in the day kline map", stock_code));
        }

        //当天分时数据存入Cassandra
        Ok(())
    }

    pub async fn query_kline(&self, stock_code: &String, kline_type: i32) -> Option<KLineData> {
        let ret = match kline_type{
            1 => self.kline_1.get(stock_code),
            5 => self.kline_5.get(stock_code),
            10 => self.kline_10.get(stock_code),
            30 => self.kline_30.get(stock_code),
            60 => self.kline_60.get(stock_code),
            1440 => self.kline_day.get(stock_code),
            _ => None,
        };

        if ret.is_none() {
            log::error!("未找到数据: {}",stock_code);
            return None
        }
        Some(ret.unwrap().value().to_owned())
    }
}
