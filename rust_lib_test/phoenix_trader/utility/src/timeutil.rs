// use anyhow::{anyhow, Result};
// use chrono::{naive, DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};
// use std::time::{Duration, SystemTime, UNIX_EPOCH};

// pub fn system_time_to_timestamp(t: SystemTime) -> f64 {
//     t.duration_since(UNIX_EPOCH).unwrap().as_micros() as f64 / 1_000_000_f64
// }

// pub fn timestamp_to_system_time(t: f64) -> SystemTime {
//     UNIX_EPOCH + Duration::from_secs_f64(t)
// }

// pub fn current_system_time() -> SystemTime {
//     SystemTime::now()
// }

// pub fn current_timestamp() -> i64 {
//     // system_time_to_timestamp(current_system_time())
//     SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
// }

// //return date with 20220101
// pub fn current_date() -> i32 {
//     let dnow = Local::today();
//     // let dtstr = format!("{}{}{}", dnow.year(), dnow.month(), dnow.day());
//     let dtstr = dnow.format("%Y%m%d");
//     // SystemTime::now().
//     // let dtstr = dtstr.to_string();
//     dtstr.to_string().parse::<i32>().unwrap_or_default()
// }

// pub fn current_timestamp_mills() -> u128 {
//     SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
// }

// pub fn current_naive_time() -> NaiveDateTime {
//     chrono::Local::now().naive_local()
// }

// //2022-05-05 14:21:48.000
// pub fn build_naive_date_time(data_time: &String) -> Result<NaiveDateTime> {
//     //1 ==> 多8个小时
//     // let time: DateTime<Local> = (str + "Z").parse().unwrap();
//     // let data = time.naive_local();
//     // data

//     //2
//     let ret = NaiveDateTime::parse_from_str(&data_time, "%Y-%m-%d %H:%M:%S%.3f");
//     if ret.as_ref().is_err() {
//         return Err(anyhow!("%Y-%m-%d %H:%M:%S%.3f: {}", ret.as_ref().err().unwrap().to_string()));
//     }
//     Ok(ret.unwrap())
// }

// pub fn from_timestamp_to_naive_date_time(timestamp: i64) -> Result<NaiveDateTime> {
//     let ret = NaiveDateTime::parse_from_str(&(timestamp.to_string()), "%s");
//     if ret.as_ref().is_err() {
//         return Err(anyhow!("时间戳: {}", ret.as_ref().err().unwrap().to_string()));
//     }
//     Ok(ret.unwrap())
// }

// pub fn convert_datetime_to_timestamp(dt: &String) -> i64 {
//     let ret = build_naive_date_time(dt);
//     if ret.as_ref().is_err() {
//         return 0;
//     }
//     let naivedt = ret.unwrap();
//     naivedt.timestamp_millis()
// }

// //2022-05-05
// pub fn build_naive_date(date: &String) -> Result<NaiveDate> {
//     let ret = NaiveDate::parse_from_str(&data, "%Y-%m-%d");
//     if ret.as_ref().is_err() {
//         return Err(anyhow!("解析日期错误:{}", ret.as_ref().err().unwrap().to_string()));
//     }
//     Ok(ret.unwrap())
// }

// //14:21:48
// pub fn build_naive_time(time: &String) -> Result<NaiveTime> {
//     let ret = NaiveTime::parse_from_str(&time, "%H:%M:%S");
//     if ret.as_ref().is_err() {
//         return Err(anyhow!("解析时间错误:{}", ret.as_ref().err().unwrap().to_string()));
//     }
//     Ok(ret.unwrap())
// }

// //str: %H:%M:%S
// pub fn calc_minute_counts(str: &String) -> Result<i32> {
//     let naive_time = build_naive_time(str);
//     if naive_time.as_ref().is_err() {
//         return Err(anyhow!("处理时间错误:{}", naive_time.as_ref().err().unwrap().to_string()));
//     }
//     let naive_time = naive_time.unwrap();
//     Ok((naive_time.hour() * 60 + naive_time.minute()) as i32)
// }

// pub struct FTimestamp(pub f64);

// impl From<FTimestamp> for f64 {
//     fn from(f: FTimestamp) -> f64 {
//         f.0
//     }
// }

// impl From<&f64> for FTimestamp {
//     fn from(f: &f64) -> FTimestamp {
//         FTimestamp(*f)
//     }
// }

// impl From<FTimestamp> for NaiveDateTime {
//     fn from(f: FTimestamp) -> NaiveDateTime {
//         NaiveDateTime::from_timestamp(f.0 as i64, ((f.0 - f.0 as i64 as f64) * 1e9) as u32)
//     }
// }

// impl From<&NaiveDateTime> for FTimestamp {
//     fn from(f: &NaiveDateTime) -> FTimestamp {
//         FTimestamp(f.timestamp_nanos() as f64 / 1e9)
//     }
// }

// impl From<&DateTime<Utc>> for FTimestamp {
//     fn from(f: &DateTime<Utc>) -> FTimestamp {
//         FTimestamp(f.timestamp() as f64)
//     }
// }

// impl From<FTimestamp> for DateTime<Utc> {
//     fn from(f: FTimestamp) -> DateTime<Utc> {
//         DateTime::<Utc>::from_utc(f.into(), Utc)
//     }
// }






// use anyhow::{anyhow, Result};
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn system_time_to_timestamp(t: SystemTime) -> f64 {
    t.duration_since(UNIX_EPOCH).unwrap().as_micros() as f64 / 1_000_000_f64
}

pub fn timestamp_to_system_time(t: f64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs_f64(t)
}

pub fn current_system_time() -> SystemTime {
    SystemTime::now()
}

pub fn current_timestamp() -> i64 {
    // system_time_to_timestamp(current_system_time())
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}

//return date with 20220101
pub fn current_date() -> i32 {
    let dnow = Local::today();
    // let dtstr = format!("{}{}{}", dnow.year(), dnow.month(), dnow.day());
    let dtstr = dnow.format("%Y%m%d");
    // SystemTime::now().
    // let dtstr = dtstr.to_string();
    dtstr.to_string().parse::<i32>().unwrap_or_default()
}

pub fn current_timestamp_mills() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

pub fn current_naive_time() -> NaiveDateTime {
    chrono::Local::now().naive_local()
}

/// 2022-05-05 14:21:48.000
pub fn build_naive_date_time(data_time: &str) -> NaiveDateTime {
    //1 ==> 多8个小时
    // let time: DateTime<Local> = (str + "Z").parse().unwrap();
    // let data = time.naive_local();
    // data

    //2
    let ret = NaiveDateTime::parse_from_str(data_time, "%Y-%m-%d %H:%M:%S%.3f");
    if ret.as_ref().is_err() {
        log::error!("%Y-%m-%d %H:%M:%S%.3f: {}, {}", ret.as_ref().err().unwrap().to_string(), data_time);
        return current_naive_time();
    }
    ret.unwrap()
}

/// 
pub fn from_timestamp_to_naive_date_time(timestamp: i64) -> NaiveDateTime {
    let ret = NaiveDateTime::parse_from_str(&(timestamp.to_string()), "%s");
    if ret.as_ref().is_err() {
        log::error!("时间戳转时间错误: {}, {}", ret.as_ref().err().unwrap().to_string(), timestamp);
        return current_naive_time();
    }
    ret.unwrap()
}

/// 2022-05-05 14:21:48.000
pub fn convert_datetime_to_timestamp(dt: &str) -> i64 {
    let ret = build_naive_date_time(dt);
    ret.timestamp()
}

/// 2022-05-05
pub fn build_naive_date(date: &str) -> NaiveDate {
    let ret = NaiveDate::parse_from_str(date, "%Y-%m-%d");
    if ret.as_ref().is_err() {
        log::error!("解析日期错误: {}, {}", ret.as_ref().err().unwrap().to_string(), date);
        return current_naive_time().date();
    }
    ret.unwrap()
}

/// 14:21:48
pub fn build_naive_time(time: &str) -> NaiveTime {
    let ret = NaiveTime::parse_from_str(time, "%H:%M:%S");
    if ret.as_ref().is_err() {
        log::error!("解析时间错误: {}, {}", ret.as_ref().err().unwrap().to_string(), time);
        return current_naive_time().time();
    }
    ret.unwrap()
}

/// str: %H:%M:%S
pub fn calc_minute_counts(str: &str) -> i32 {
    let naive_time = build_naive_time(str);
    (naive_time.hour() * 60 + naive_time.minute()) as i32
}

pub struct FTimestamp(pub f64);

impl From<FTimestamp> for f64 {
    fn from(f: FTimestamp) -> f64 {
        f.0
    }
}

impl From<&f64> for FTimestamp {
    fn from(f: &f64) -> FTimestamp {
        FTimestamp(*f)
    }
}

impl From<FTimestamp> for NaiveDateTime {
    fn from(f: FTimestamp) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(f.0 as i64, ((f.0 - f.0 as i64 as f64) * 1e9) as u32)
    }
}

impl From<&NaiveDateTime> for FTimestamp {
    fn from(f: &NaiveDateTime) -> FTimestamp {
        FTimestamp(f.timestamp_nanos() as f64 / 1e9)
    }
}

impl From<&DateTime<Utc>> for FTimestamp {
    fn from(f: &DateTime<Utc>) -> FTimestamp {
        FTimestamp(f.timestamp() as f64)
    }
}

impl From<FTimestamp> for DateTime<Utc> {
    fn from(f: FTimestamp) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(f.into(), Utc)
    }
}
