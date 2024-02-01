use anyhow::{Ok, Result};
// use chrono::{DateTime, Local, NaiveTime, Offset, Timelike, format::format};
use chrono::prelude::*;
use dashmap::DashMap;
use std::sync::Arc;

use crate::config::settings::Settings;

// pub struct TimeSlot {
//     // exchangeno: String,
//     starttime: String,
//     endtime: String,
// }
#[derive(Clone, Debug)]
pub struct TimeSlot {
    pub sttime: NaiveTime,
    pub endtime: NaiveTime,
}

#[derive(Clone, Debug)]
pub struct CommonUtil {
    trade_times: Arc<DashMap<String, Vec<TimeSlot>>>,
    // call_auction_area: Arc<DashMap<String, Vec<TimeSlot>>>,
    // break_area: Arc<DashMap<String, Vec<TimeSlot>>>,
}

impl CommonUtil {
    pub fn new() -> Self {
        Self {
            trade_times: Arc::new(DashMap::new()),
        }
    }

    pub async fn init(&self, qtsetting: &Settings) -> Result<()> {
        let quotation_times = qtsetting.quotaiontime.get("exchange").unwrap();
        for (key, val) in quotation_times.iter() {
            //
            let mut time_slots: Vec<TimeSlot> = Vec::new();
            for qtime in val.iter() {
                let xshg: Vec<&str> = qtime.split('-').collect(); //"09:15:00","11:31:00"
                if xshg.len() != 2 {
                    log::error!("配置时间有错误:{}", qtime);
                    continue;
                }
                let sttime = utility::timeutil::build_naive_time(&xshg[0].to_string());
                let endtime = utility::timeutil::build_naive_time(&xshg[1].to_string());
                let timeslot = TimeSlot { sttime, endtime };
                time_slots.push(timeslot);
            }
            self.trade_times.insert(key.clone(), time_slots);
        }
        log::info!("初始化后的时间:{:#?}", &self.trade_times);
        Ok(())
    }

    ///根据市场判断是否是特殊时间的tick 开市集合竞价|午休|闭市集合竞价时期(tick交易时间, tick市场)
    pub async fn check_hqtick_time(&self, time: &String, exchangeno: &String) -> Result<(String, i32)> {
        // let vec_ret = self.get_interval(exchangeno).await;
        // if vec_ret.is_none() {
        //     log::error!("找不到市场时间：{}",exchangeno);
        //     return Err(anyhow!("找不到市场时间：{}",exchangeno));
        // }
        // let vec = vec_ret.unwrap();
        let tick_time = utility::timeutil::build_naive_time(time);

        let min = tick_time.hour() * 60 + tick_time.minute(); //tick分钟
        {
            if exchangeno == "XSHG" || exchangeno == "XSHE" || exchangeno == "HS" {
                //开盘
                if min >= 555 && min < 570 {
                    //A股 9:15:00 - 9:30:00
                    return Ok(("09:30:00".to_string(), 1));
                }
                //午休
                if min >= 690 && min < 692 {
                    //A股  11:30:00 - 11:32:00
                    return Ok(("11:30:00".to_string(), 2));
                }
                // 闭市
                if min >= 900 && min < 902 {
                    //A股  15:00:00 - 15:02:00
                    return Ok(("15:00:00".to_string(), 3));
                }
            }
            if exchangeno == "XHKG" || exchangeno == "HK" {
                //开盘
                if min >= 540 && min < 570 {
                    //9:00:00 - 9:30:00之间的
                    return Ok(("09:30:00".to_string(), 1));
                }
                //午休
                if min >= 720 && min < 721 {
                    //港股 12:00:00 - 12:01:00
                    return Ok(("12:00:00".to_string(), 2));
                }
                // 闭市
                if min >= 960 && min < 970 {
                    //港股 [16:00:00 - 16:10:00)
                    return Ok(("16:00:00".to_string(), 3));
                }
            }
        }
        Ok((String::new(), -1))
    }

    // 给定时间判断所在交易区间(特殊时间外)
    pub async fn trade_area_index(&self, trade_time: &String, exchangeno: &String) -> Result<usize> {
        let trade_time = utility::timeutil::build_naive_time(trade_time);

        if let Some(time_area) = self.trade_times.get(exchangeno) {
            for (mut count, area) in time_area.iter().enumerate() {
                if trade_time >= area.sttime && trade_time < area.endtime {
                    //大于等于开始时间, 小于结束时间
                    // log::info!("{}", count);
                    return Ok(count); // 1早上, 2下午
                }
                count += 1;
            }
        }
        // Ok(-1)//=============================???
        return Err(anyhow!("给定时间不在交易时间内"));
    }

    // pub async fn get_trade_area(&self, exchangeno: &String) -> Vec<TimeSlot> {
    //     log::info!("[get_trade_area] exchangeno: {}", exchangeno);
    //     // match self.trade_times.get(exchangeno) {
    //     //     Some(v) => v.to_vec(),
    //     //     None => vec![],
    //     // }

    //     if let Some(v) = self.trade_times.get(exchangeno) {
    //         return v.value().to_owned();
    //     }
    //     vec![]
    // }

    pub async fn get_interval(&self, exchangeno: &String) -> Option<Vec<TimeSlot>> {
        if let Some(ret) = self.trade_times.get(exchangeno) {
            return Some(ret.to_vec());
        } else {
            log::error!("找不到市场时间：{}", exchangeno);
            return None;
        }
    }

    pub async fn get_break_interval(&self, exchangeno: &String) -> (String, String) {
        if let Some(time_slot) = self.trade_times.get(exchangeno) {
            let vec = time_slot.to_vec();
            let btime = format!("{}", vec[0].sttime.clone());
            let etime = format!("{}", vec[0].endtime.clone());
            return (btime, etime);
        }
        (String::new(), String::new())
    }

    pub async fn get_begin_time(&self, exchangeno: &String) -> Result<NaiveTime> {
        let ret = self.trade_times.get(exchangeno);
        if ret.is_none() {
            log::error!("未找到开始时间");
            return Err(anyhow!("未找到开始时间"));
        }
        let times = ret.unwrap();
        Ok(times[0].sttime)
    }
    
    pub async fn get_close_time(&self, exchangeno: &String) -> Result<NaiveTime> {
        // let mut close_time = String::default();
        // if exchangeno == "XSHG" || exchangeno == "XSHE" || exchangeno == "HS" {
        //     close_time = "15:00:00";
        // } else if exchangeno == "XHKG" || exchangeno == "HK" {
        //     close_time = "16:10:00";
        // } else if exchangeno == "XASE" || exchangeno == "XNYS" || exchangeno == "XNAS" || exchangeno == "US" {
        //     close_time = "16:10:00";
        // }

        let ret = self.trade_times.get(exchangeno);
        if ret.is_none() {
            log::error!("未找到结束时间");
            return Err(anyhow!("未找到结束时间"));
        }
        let times = ret.unwrap();
        let size = times.len();
        Ok(times[size - 1].endtime)
    }

    pub async fn get_interval_begin_time(&self, time: &String, exchangeno: &String) -> Result<NaiveTime> {
        let ret = self.trade_area_index(time, exchangeno).await;
        if ret.as_ref().is_err() {
            log::error!("不在交易时间内");
            return Err(anyhow!("不在交易时间内"));
        }
        let index = ret.unwrap();

        let ret = self.trade_times.get(exchangeno);
        if ret.is_none() {
            log::error!("未找到时间区间");
            return Err(anyhow!("未找到时间区间"));
        }
        let times = ret.unwrap();
        Ok(times[index].sttime)
    }

    //获取下一个间隔开始时间 09:30:00  13:00:(午休调用)
    pub async fn get_next_interval_begin_time(&self, time: &String, exchangeno: &String) -> Result<NaiveTime> {
        if let Some(value) = self.trade_times.get(exchangeno) {
            let size = value.len();
            for val in value.value().iter() {
                let trade_time = format!("{}", val.endtime);//11:31:00 -> 13:00:00
                if trade_time[0..5] == time[0..5] {
                    return Ok(value.value()[size-1].sttime.clone());//13:00:00
                }
            }
            Ok(value.value()[0].sttime.clone())//09:30:00
        } else {
            log::error!("未找到时间区间");
            return Err(anyhow!("未找到时间区间"));
        }
    }
}
