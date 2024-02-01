use anyhow::Result;
use dashmap::DashMap;
use chrono::Timelike;
use std::sync::Arc;

use super::klineservice::KLineService;
use crate::{
    protofiles::YsHqInfo, 
    commonutil::commonutil::CommonUtil, 
    server::service::klineservice::TimePair,
    // client::cassandraclient::CassandraClient, 
};
use utility::timeutil::{
    build_naive_date_time, 
    from_timestamp_to_naive_date_time
};

//主要功能：
//1)保存到tick缓存
//2)持久化tick到kv存储

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

    // pub async fn init_tick_cache(&self, ticks: &Vec<YsHqInfo>) {
    //     for hqinfo in ticks.iter() {
    //         self.ticks_cache.insert(hqinfo.contract_no1.to_owned(), hqinfo.to_owned());
    //     }

    //    // ticks.iter().map(|&x| self.ticks_cache.insert(x.contract_no1.clone(), x));
    // }

    pub async fn insert_ticks(&self, hqinfo: &mut YsHqInfo, common_util: &CommonUtil, kline_ctl: &KLineService) -> Result<()> {
        let hq_time = hqinfo.tapidtstamp[11..19].to_owned();
        let ret = common_util.check_hqtick_time(&hq_time, &hqinfo.exchange_id).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("检查时间错误： {:?}", ret));
        }
        let (ret_time, flag) = ret.unwrap();

        let new_time = hqinfo.tapidtstamp[0..11].to_owned() + &ret_time; //yyyy-mm-dd 09:30:00
        let new_hq_time = hqinfo.tapidtstamp[11..19].to_owned();
        if flag < 0 {
            if (common_util.trade_area_index(&new_hq_time, &hqinfo.exchange_id).await).is_err() {
                return Err(anyhow!("该条tick不在交易时间内,不处理: code: {}, time: {}", hqinfo.contract_no1, hqinfo.tapidtstamp));
            }
        }
        //K线相关
        if flag == 1 {
            //开盘集合竞价
            self.ticks_cache.insert(hqinfo.contract_no1.clone(), hqinfo.to_owned());
            let mut st = build_naive_date_time(&new_time);
            let mut tt = st.timestamp();
            tt -= 30;
            st = from_timestamp_to_naive_date_time(tt);
            let time_pair = TimePair {
                minute: (st.hour() * 60 + st.minute()).into(),
                timestamp: tt
            };
            kline_ctl.contract_fill.insert(hqinfo.contract_no1.clone(), time_pair);
            TickService::before_disc_data(hqinfo, common_util, kline_ctl).await;
            return Err(anyhow!("开盘集合竞价数据不发送MQ..."));
        } else if flag == 2 || flag == 3 {
            //午休tick //闭市集合竞价tick
            let mut st = build_naive_date_time(&new_time);
            //闭市把时间戳处理成14:59:59.999, 15:59:59.999
            //午休11:29:59
            let mut tt = st.timestamp();
            tt -= 1; //yyyy-mm-dd 14:59:59, yyyy-mm-dd 11:29:59
            st = from_timestamp_to_naive_date_time(tt); 
            let tapidtstamp = format!("{}.999", st);
            hqinfo.tapidtstamp = tapidtstamp;

            let mut time_pair = TimePair{
                minute: 0,
                timestamp: tt,
            };
            if flag == 2 {
                if hqinfo.exchange_id == "HS" {// ???
                    //沪深指数有可能11:31分还推有效数据 此处要做特殊处理
                    time_pair.minute = (st.hour() * 60 + st.minute() + 2).into(); //11:31
                } else {
                    time_pair.minute = (st.hour() * 60 + st.minute() + 1).into(); //11:30
                }
                kline_ctl.contract_fill.insert(hqinfo.contract_no1.clone(), time_pair);
            } else if flag == 3 {
                if hqinfo.exchange_id == "XSHG" || hqinfo.exchange_id == "XSHE" {
                    time_pair.minute = (st.hour() * 60 + st.minute() + 1).into(); //yyyy-mm-dd 15:00
                } else if hqinfo.exchange_id == "HS" {
                    time_pair.minute = (st.hour() * 60 + st.minute() + 2).into(); //yyyy-mm-dd 15:01
                } else if hqinfo.exchange_id == "XHKG" || hqinfo.exchange_id == "HK" {
                    //防止超过一分钟没有tick造成补数据开始补16:00:00的数据
                    time_pair.minute = (st.hour() * 60 + st.minute() + 10).into();
                } else if hqinfo.exchange_id == "US" || hqinfo.exchange_id == "XASE" || hqinfo.exchange_id == "XNYS" || hqinfo.exchange_id == "XNAS" {
                    time_pair.minute = (st.hour() * 60 + st.minute() + 1 - 12 * 60 + 30).into();
                    //04:30
                }
                kline_ctl.contract_fill.insert(hqinfo.contract_no1.clone(), time_pair);
            }
        }
        self.ticks_cache.insert(hqinfo.contract_no1.to_owned(), hqinfo.to_owned());
        Ok(())
    }

    async fn before_disc_data(hqinfo: &YsHqInfo, common_util: &CommonUtil, kline_ctl: &KLineService) {
        kline_ctl.insert_or_update(hqinfo, crate::server::service::common::KLineType::Hq1Kline, common_util).await;
        kline_ctl.insert_or_update(hqinfo, crate::server::service::common::KLineType::Hq5Kline, common_util).await;
        kline_ctl.insert_or_update(hqinfo, crate::server::service::common::KLineType::Hq10Kline, common_util).await;
        kline_ctl.insert_or_update(hqinfo, crate::server::service::common::KLineType::Hq30Kline, common_util).await;
        kline_ctl.insert_or_update(hqinfo, crate::server::service::common::KLineType::Hq60Kline, common_util).await;
        kline_ctl.insert_or_update(hqinfo, crate::server::service::common::KLineType::Hq24Kline, common_util).await;
    }

    pub async fn query_tick(&self, key: &String) -> Option<YsHqInfo> {
        let hqinfo = self.ticks_cache.get(key);
        if hqinfo.is_none() {
            return None
        }
        Some(hqinfo.unwrap().value().to_owned())           
    }
}


