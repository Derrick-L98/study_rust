#![allow(dead_code, unused_imports)]//功能:没有使用的代码或模块不警告
#[macro_use]
extern crate chrono;
extern crate time;
extern crate wasm_bindgen_test;
extern crate anyhow;
extern crate num_cpus;
extern crate lazy_static;
extern crate clap;
extern crate bindgen;
// extern crate log;
// extern crate sled;
//extern crate convert_case;
mod common;
mod config;
mod outputcolor;
mod setting;
mod sled;
mod structure;
mod myyew;
mod my_thread;
mod my_gui;
mod redistest;
mod socketsub;

use anyhow::Result;
use anyhow::anyhow;
use axum::response::Html;
use tokio::sync::RwLock;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::iter::repeat;
use std::mem::size_of_val;
use std::net::SocketAddr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Write;

use crate::common::SummarizedTickData;
use crate::common::UidgenService;
use crate::config::Config;
// use crate::redistest::redisclustertest::RedisClusterClient;

use self::chrono::prelude::*;
use self::chrono::offset::Local;
use chrono::{DateTime, TimeZone, Utc};


use convert_case::{Case, Casing};
use dashmap::DashMap;

use time::strptime;
use ansi_term::Colour::Red;
// use time::*;

use std::time::Instant;
use std::time::*;
// use thread_id::{self};



use tokio::sync::broadcast;

use cached::{Cached, TimedSizedCache};
use setting::Settings;
use tokio::runtime;
// use myyew::Model;
// use loom::sync::atomic::AtomicUsize;
// use loom::sync::atomic::Ordering::{Acquire, Relaxed, Release};
// use loom::sync::Arc;
// use loom::thread;

use console::{style, Term};

use outputcolor::{console, print_da, write_chars, test as TEST};
use crate::outputcolor::do_stuff;
use structure::Structure;

use axum::{
    routing::get,
    Router,
};
use local_ip_address::local_ip;

// use gtk::prelude::*;
// use gtk::{glib, Application};
#[tokio::main]
async fn main() {

    let mut s = HashMap::new();
    s.insert("1", 1);
    s.entry("1").or_insert(2);
    s.entry("1").or_insert(8);
    println!("{:?}", s);
    // let s = vec!(1,2,3,4,5);
    // for a in s {
    //     println!("{a}");
    // }

    // // Create a new application
    // let app = Application::builder().application_id(APP_ID).build();

    // // Run the application
    // app.run();
    
    
    // my_gui::my_gui().await;
    println!("This is in red: {}", Red.paint("a red string"));
    // yew::start_app::<Model>();
    // count logical cores this process could try to use
    // let num = num_cpus::get();

    // let output = "❄️ 🐼 🚓 👅";
    // println!("{} ========={}",num, output);
    // let s:i32 = format!("{:02}{:02}", 1,20).parse().unwrap();
    // println!("{}", s);
    // // my_thread::thread().await;
    // let my_local_ip = local_ip().unwrap();

    // println!("This is my local IP address: {:?}", my_local_ip);
    // p().await;
    // cache().await;
    // mut_no_mut();
    // dash_map();
    naive_date_time();
    // performance();
    // hash_map();
    // vec();
    // time();

    // lock().await;
    // f64_decimal();
    // web().await;
    // Rw().await;

    // println!("{:?}", error().await.as_ref().err().unwrap().to_string());

    // channel test
    // tokio_channel().await;
    // async_channel().await;
    // println!("{}", meh(1000000000.10396412, 5));
    // println!("{}", 1.precision(5));
    // //格式化输出保留小数后6位,不足补0
    // let kk = format!("{:0.6} {:0<.6} {:.06} {:<.6} {:.6}", 1.0619, 1.06193, 1.06193, 1.06193, 1.06193);
    // let k = format!("{:04} {:02} {:02} {:02} {:.20}", 1, 2, 3, 4, "fda");
    // println!("{}\n{}", kk, k);
    // console().await;
    // write_chars().await;
    // do_stuff().await;
	/*
	let mut num: i8 = 0;
	loop {
		println!("{}", num);//release 模式下死循环 i8 -128 ~ 127
							//debug 模式下 i8 0 ~ 127 数据溢出,程序崩溃
		num += 1;
	}

    // time test

	*/
    // thread();
    // string();
    // text();

    // mut_no_mut();

    // channel().await;
    // let now = Instant::now();
    // println!("耗时{:?}", now.elapsed());

    std::thread::sleep(Duration::from_millis(0));

    /*
    let setting = Settings::new().expect("失败");
    // let config = Config::new().await;
    // config.config(&setting).await;
    println!("{:#?}", setting);

    // let s = 0.32f64.to_string();
    // println!("s = {}", s);

    // //s = 0.32 0 0.32 0 0.32 0 0.32
    // println!("s = {0} {1} {0} {1} {0} {1} {0}", s, 0);
    // println!("我的名字叫{name}, 今年{age}岁, 喜欢{hobby}", hobby = "打篮球", name = "张三", age = 18);

    // cache().await;

    //格式化输出保留小数后6位,不足补0
    let kk = format!("{:0<.6} {:.06} {:<.6} {:.6}", 1.0, 2.0, 3.0, 4.0);
    let k = format!("{:04} {:02} {:02} {:02} {:.20}", 1, 2, 3, 4, "fda");
    // println!("{}\n{}", kk, k);
    dbg!(k); //打印的位置和变量的名称

    let mut s = String::new();
    // let _ = write!(s, "0x{:X}", 1024);
    let now = Instant::now();
    let _ = write!(s, "{}", "aaa");
    // let _ = write!(s, "{}", " bbb");
    // let _ = write!(s, "{}", " ccc");
    // let d = " ddd";
    // let _ = write!(s, "{}", d);

    println!("{} {:?}", s, now.elapsed());

    // string_write().await;
    // string_cmp().await;

    // console().await;
    // write_chars().await;
    // p().await;
    // print_da().await;

    // let mut structs = Structure::new().await;
    // structs.push().await;
    // structs.show().await;
    */
    // sled::dbcache().await;
/* 
    let s = vec![
        1, 2, 3, 4, 5, 6, 7, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 99, 9, 9,
        9, 9, 9, 9, 9, 99, 9, 9, 9, 9, 9, 9,
    ];
    let mut p: Vec<Vec<i32>> = s.chunks(8).map(|x| x.into()).collect();

    // let pp = p.next().unwrap();
    println!("{:?}", p);

    // let str = "fafdsafd".to_string();
    // println!("{:p}", &str);
    // ptrc(&str).await;

    tokio::spawn(async move {
        for i in 0..1 {
            let str = "fafdsafd".to_string();
            println!("str:   {:p}", &str);
            ptrc(&str).await;
        }
    });
    */


    // let client = RedisClusterClient::new().await;
    // let now = Instant::now();
    // let ret = client.get_value(&"order_7085462332042600371".to_string()).await;
    // println!("cost: {:?}", now.elapsed());
    // println!("{:#?}", ret.unwrap());
    
    // socketsub::sock();
    // socketsub::sock2();

    // if let Err(e) = socketsub::socket3::server().await {
    //     println!("{:?}", e);
    // }
}


async fn precision() {
    let sp: Decimal = dec!(6.5);
    println!("1:       {}", sp.round_dp_with_strategy(0, RoundingStrategy::MidpointNearestEven));//当一个数字介于另外两个数字之间时，它会向最近的偶数取整。也称为“银行家取整”
    println!("2:       {}", sp.round_dp_with_strategy(0, RoundingStrategy::MidpointAwayFromZero));//当一个数字介于另外两个数字之间时，它会向离零最近的数字四舍五入
    println!("3:       {}", sp.round_dp_with_strategy(0, RoundingStrategy::MidpointTowardZero));//当一个数字位于另外两个数字的中间时，它会向接近零的数字四舍五入
    println!("4:       {}", sp.round_dp_with_strategy(2, RoundingStrategy::ToZero));//数字总是向零四舍五入    X
    println!("5:       {}", sp.round_dp_with_strategy(2, RoundingStrategy::AwayFromZero));//数字总是从零开始四舍五入   X
    println!("6:       {}", sp.round_dp_with_strategy(2, RoundingStrategy::ToNegativeInfinity));//数字总是向负无穷大四舍五入   X
    println!("7:       {}", sp.round_dp_with_strategy(2, RoundingStrategy::ToPositiveInfinity));//数字总是向正无穷大取整    X
    // println!("8:       {}", sp.round_dp_with_strategy(2, RoundingStrategy::BankersRounding));//当一个数字介于另外两个数字之间时，它会向最近的偶数取整
    // println!("9:       {}", sp.round_dp_with_strategy(2, RoundingStrategy::RoundHalfUp));//如果值>=5，则向上取整，否则向下取整
    // println!("10:      {}", sp.round_dp_with_strategy(2, RoundingStrategy::RoundHalfDown));//如果值=<5，则向下取整，否则向上取整
    // println!("11:      {}", sp.round_dp_with_strategy(2, RoundingStrategy::RoundDown));//向下舍入
    // println!("12:      {}", sp.round_dp_with_strategy(2, RoundingStrategy::RoundUp));//向上舍入
}


async fn web() {
    // // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // println!("================");
    // // run it with hyper on localhost:3000
    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
    // println!("================");

        // build our application with a route
    #[derive(Debug)]
    struct X {
        a: i32,
        b: i32,
    };
    let mut vec = vec![X{a:1,b:2}];
    println!("{:#?}", vec);
    // let _ = vec.iter_mut().map(|x| {x.a = 3; x.b = 4});
    for x in vec.iter_mut() {
        x.a = 3; 
        x.b = 4;
    }
    println!("{:#?}", vec);
    let mut vec2 = vec![1,1,1,1];
    let vec1 = vec![22,2,2,2];
    // println!("{:#?}", s);
    for (i, &mut val) in vec2.iter_mut().enumerate() {
        if vec1.iter().find(|&&x| x == val).is_none() {
            continue;
            // vec2.remove(i);
        }
    }
    println!("{:#?}", vec2);
    let app = Router::new().route("/", get(handler));

    // run it
    let addr = SocketAddr::from(([172,16,1,109], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// 保留小数多少位
fn meh(float: f64, precision: usize) -> String {
    // compute absolute value
    // let a = float.abs();

    // // if abs value is greater than 1, then precision becomes less than "standard"
    // let precision = if a >= 1. {
    //     // reduce by number of digits, minimum 0
    //     let n = (1. + a.log10().floor()) as usize;
    //     println!("位1: {}", n);
    //     if n <= precision {
    //         precision
    //     } else {
    //         0
    //     }
    // // if precision is less than 1 (but non-zero), then precision becomes greater than "standard"
    // } else if a > 0. {
    //     // increase number of digits
    //     let n = -(1. + a.log10().floor()) as usize;
    //     println!("位2: {}", n);
    //     precision + n
    // // special case for 0   
    // } else {
    //     0
    // };

    // format with the given computed precision
    format!("{0:.1$}", float, precision)
}

pub trait Point<T> {
    fn precision(&self, indix: usize) -> T;
} 
impl<T: std::str::FromStr + std::fmt::Display + std::default::Default> Point<T> for T {
    fn precision(&self, indix: usize) -> T {
        format!("{0:.1$}", self, indix).parse::<T>().unwrap_or_default()
    }
}


async fn ptrc(str: &String) {
    println!("str:   {:p}", str);
}


async fn cache() {
    let mut tickdata_cache2: TimedSizedCache<String, SummarizedTickData> =
        TimedSizedCache::with_size_and_lifespan(100, 60 * 60 * 80);

    let mut data = SummarizedTickData::new();
    data.stock_code = "601388_XSHG".to_string();
    data.close_price = 1.00;
    data.high_price = 5.00;
    data.last_price = 85.00;
    data.low_price = 0.1;
    data.total_turnover = 100000000.0;
    data.total_volume = 5000.0;
    tickdata_cache2.cache_set(data.stock_code.clone(), data.clone());
    data.open_price = 11111111.0;
    tickdata_cache2.cache_set(data.stock_code.clone(), data.clone());
    data.open_price = 11111.0;
    tickdata_cache2.cache_set(data.stock_code.clone(), data.clone());
    data.open_price = 1.0;
    tickdata_cache2.cache_set(data.stock_code.clone(), data);

    // let s = tickdata_cache2.cache_get_mut(&"601388_XSHG".to_string()).unwrap();
    // s.open_price = 1555555555.0;
    // data.stock_code = "601389_XSHG".to_string();
    // tickdata_cache2.cache_set(data.stock_code.clone(), data.clone());
    // data.open_price = 11111111.0;
    // let p = tickdata_cache2.cache_set(data.stock_code.clone(), data.clone()).unwrap();//返回前一个同k的值

    // data.stock_code = "601311_XSHG".to_string();
    // tickdata_cache2.cache_set(data.stock_code.clone(), data.clone());
    // data.stock_code = "601300_XSHG".to_string();
    // tickdata_cache2.cache_set(data.stock_code.clone(), data.clone());
    // data.stock_code = "601319_XSHG".to_string();
    // tickdata_cache2.cache_set(data.stock_code.clone(), data.clone());
    // data.stock_code = "601378_XSHG".to_string();
    // tickdata_cache2.cache_set(data.stock_code.clone(), data.clone());

    println!("{:#?}", tickdata_cache2.get_store().get_order().borrow());

    // println!("{:#?}", p);
    // println!("{:#?}", tickdata_cache2.cache_get(&"601389_XSHG".to_string()).unwrap());
    println!("{}", tickdata_cache2.cache_hits().unwrap());
    println!("{}", tickdata_cache2.cache_misses().unwrap());
    // println!("{}", );s
    // tickdata_cache2.cache_set(k, v)
}


//可变引用与不可变引用
fn mut_no_mut() {
    let val = String::from("4.160|4.150|4.150|0|20210323093000+4.147|4.150|284|20210323093100+4.148|4.160|24|20210323093200+4.148|4.150|9|20210323093400");
    println!("原来：{}", val);
    let mut val = mut_s(val).unwrap();

    let s = &mut val;
    // let c = &mut val;//第二个可变借用发生在这里 error
    s.push_str("oob");
    // c.push_str("oob");

    let _v = &val;
    let _f = &val;
    println!("更改：{}", val);

    let s1 = String::from("run");
    let s2 = &s1;
    println!("{}", s2);
    // s2.push_str("oob"); // 错误，禁止修改租借的值
    println!("{}", s2);

    let mut s1 = Test { str: 1, ptr: 2 };

    test(&mut s1);
    println!("s1:{:#?}", s1);
}
struct S {
    pub map: DashMap<String, String>,
}
impl S {
    fn new() -> Self {
        let map = DashMap::new();
        map.insert("a".to_string(), "aaaaaaaaaaaaaa".to_string());
        map.insert("b".to_string(), "bbbbbbbbbbbbbb".to_string());
        S { map }
    }
    fn str(&self, key: String) {
        if let Some(_val) = self.map.get(&key) {}
    }
    fn ptr(&self, key: &String) {
        if let Some(_val) = self.map.get(key) {}
    }
}
fn str(key: String) {
    println!("{:?}", key.as_ptr());
}
fn ptr(key: &String) {
    println!("{:?}", key.as_ptr());
}

fn mut_s(val: String) -> Result<String, Box<dyn std::error::Error>> {
    // let mut value: Vec<&str> = Vec::new();
    // if !val.is_empty() {
    //     if val.contains("+") {
    //         value = val.split("+").collect();

    //     }
    // }
    // //println!("val:{}",val);
    // // let qq = val.find('|');
    // // println!("len:{:?}",qq);
    // // if let Some(index) = val.rfind("|") {
    // //     //let str_q = i[index+1];
    // //     println!("len:{}",index);
    // // };
    // //val.clear();
    // for i in value.iter() {
    //     //println!("i: {}",i);
    //     if let Some(index) = i.rfind("|") {
    //         //let str_q:i64 = i[index+1..].parse().expect("");
    //         let str_q:String = (&i[index+1..]).to_string();
    //         //let str_q:&str = &i[index+1..];
    //         //i.replace(str_q,&str_q);
    //         let mut sm = i.trim_end_matches(&str_q).to_string();
    //         println!("s: {}",sm);
    //         sm.push_str("+++++");
    //         println!("len:{}",index);
    //         println!("str_q:{}",str_q);
    //         println!("i: {}",i);
    //         println!("s: {}",sm);
    //         //val.push_str(&sm.to_string());
    //     }
    // }
    // let mut s = String::from("bbbb");
    // //*val = s;     //可
    // *val = "fdsafdas".to_string();

    let mut value: Vec<&str> = Vec::new();
    //4.160|4.150|4.150|0|20210323093000+4.147|4.150|284|20210323093100+
    //4.148|4.160|24|20210323093200+4.148|4.150|9|20210323093400+................
    if !val.is_empty() {
        //返回true时为空
        if val.contains('+') {
            //如果给定模式与该字符串片段的子片段匹配，则返回true。如果没有，则返回false。
            value = val.split('+').collect(); //按+分割为一周期
        }
    }
    if value.is_empty() {
        return Ok(val);
    }
    let mut val_tmp = String::default();
    for i in value.iter() {
        if let Some(index) = i.rfind('|') {
            //返回此字符串片段中模式最右边匹配项的第一个字符的字节索引。
            let update_time: i64 = i[index + 1..].to_string().parse().expect("解析失败"); //取由|分割开的最后一个值
            println!("{}", update_time);
            let local_time = "20211129103".to_string();
            //i.replace(i[index+1..],&local_time);
            let mut str_tmp = i.trim_end_matches(&update_time.to_string()).to_string(); //删除末尾匹配的字符串片段, 返回剩下的
            println!("{}", str_tmp);
            str_tmp.push_str(&local_time);
            val_tmp.push_str(&str_tmp);
        }
        val_tmp.push('+');
    }
    Ok(val_tmp)
}

#[derive(Clone, Debug, Default)]
pub struct Test {
    str: i32,
    ptr: i32,
}

impl Test {
    pub fn new() -> Self {
        Test { str: 0, ptr: 0 }
    }
}

pub fn test(info: &mut Test) {
    info.str = 5;
    info.ptr = 10;
    println!("{:#?}", info);
}

//chrono::NaiveDateTime
pub fn naive_date_time() {
    let now = Local::now().timestamp_millis();
    println!("{}", now);
    //给定字符串构造时间结构
    let mut time_uct: DateTime<Utc> = ("2022-05-05 14:21:48.000".to_owned() + "Z")
        .parse()
        .unwrap(); //正常
        // println!("{}", _time_uct);//2022-05-05 14:21:48 UTC
        // println!("{:?}", _time_uct);//2022-05-05T14:21:48Z
        let data = time_uct.naive_local();
        println!("1====={}", data); //2022-05-05 14:21:48
        println!("2====={:?}", data); //2022-05-05T14:21:48
        println!("3====={}\n", data.and_utc().timestamp()); //时间戳1651760508 2022-05-05 22:21:48
    let time: DateTime<Local> = ("2022-05-05 14:21:48.000".to_owned() + "Z")
        .parse()
        .unwrap();
        // println!("{}", time);//2022-05-05 22:21:48 +08:00
        // println!("{:?}", time);//2022-05-05T22:21:48+08:00
    let data = time.naive_local();
    println!("1====={}", data); //2022-05-05 22:21:48
    println!("2====={:?}", data); //2022-05-05T22:21:48
    println!("3====={}\n", data.and_utc().timestamp()); //时间戳1651789308 2022-05-06 06:21:48

    let mut dt =
        NaiveDateTime::parse_from_str("2022-05-05 14:21:48", "%Y-%m-%d %H:%M:%S%.3f").unwrap();
    println!("2022-05-05 14:21:48 ---> NaiveDateTime: {}", dt); //2022-05-05 14:21:48
    println!("时间戳: {}", dt.and_utc().timestamp()); //时间戳
                                            // println!("周几: {:#?}", dt.iso_week());

    dt = dt
        .with_year(2022)
        .unwrap()
        .with_month(5)
        .unwrap()
        .with_day(12)
        .unwrap()
        .with_hour(20)
        .unwrap(); //更改分钟数后生成新值。当结果值无效时，返回None。

    println!("修改后的时间: {}", dt.time()); //时间
    println!("修改后的日期: {}\n", dt.date()); //日期

    let s = NaiveTime::parse_from_str("14:21:48", "%H:%M:%S").unwrap();
    println!("14:21:48 ---> NaiveTime: {}", s);
    println!("{}\n", s.hour() * 60 + s.minute());

    let t = NaiveTime::parse_from_str("2022-05-05 14:21:48.000", "%Y-%m-%d %H:%M:%S%.3f").unwrap();
    println!("2022-05-05 14:21:48.000 ---> NaiveTime: {}\n", t);

    let d = NaiveDate::parse_from_str("2022-05-05", "%Y-%m-%d").unwrap();
    println!("2022-05-05 ---> NaiveDate: {}\n", d);

    //时间戳构造时间
    {
        let pdt = NaiveDateTime::parse_from_str("1651760508", "%s").unwrap();
        println!("1651760508 ---> NaiveDateTime: {}\n", pdt);

        let ps = Local.timestamp_millis(1651760508 * 1000).naive_local(); //多8小时
                                                                          // let ps = Local.timestamp_millis(1651760508 * 1000).naive_utc();//正常
        println!("1651760508 ---> NaiveDateTime: {}\n", ps);
    }

    let cop = chrono::Local::now().naive_utc();
    // let cop = chrono::Local::now().naive_local();//多北京时间8小时
    println!("current ---> NaiveDateTime: {}", cop);
    // println!("current ---> 时间戳: {}\n", cop.timestamp());//0.4.35版本过后起, 不推荐使用,推荐使用下面的方式
    println!("current ---> 时间戳: {}\n", cop.and_utc().timestamp());

    let sss = cop.format("%Y%m%d%H%M").to_string();
    let d = cop.format("%Y%m%d").to_string();
    let t = cop.format("%H%M%S").to_string();
    println!("ssss: {}", sss);
    println!("dddd: {}", d);
    println!("tttt: {}", t);
    let da = sss.parse::<i64>().unwrap();
    println!("SSSSS: {}", da * 100);

    let end_time = (cop.year() as i64) * 10000000000
        + (cop.month() as i64) * 100000000
        + (cop.day() as i64) * 1000000
        + (cop.hour() as i64) * 10000
        + (cop.minute() as i64) * 100;
    println!("SSSSS: {}", end_time);


    let mut new_st = chrono::Local::now().naive_local();
    println!("new: {}\n", new_st);
    // let utc: DateTime<Utc> = Utc::now();
    // let local: DateTime<Local> = Local::now();

    // let now = Local::now();//str
    // let ndt = now.naive_local();//chrono::NaiveDateTime
    // let res = match Local.from_local_datetime(&ndt).single() {
    //     Some(v) => v,
    //     None => panic! {"Required for test!"},
    // };//chrono::DateTime<chrono::Local>
    // println!("now:{}\nndt:{}\nres:{}",now, ndt,res);

    // println!("utc = {}\nlocal = {}",utc, local);
    // println!("Hello, world!");
    // let dt = Local.ymd(2014, 7, 24).and_hms(12, 34, 6);
    // println!("{}",dt);
    // let dt = FixedOffset::east(9*3600).ymd(2014, 11, 28).and_hms_nano(21, 45, 59, 324310806);
    // let year = dt.year();
    // let month = dt.month();
    // let day = dt.day();
    // let hour = dt.hour();
    // let minute = dt.minute();
    // let second = dt.second();
    // println!("{}\n{}-{}-{} {}:{}:{}",dt,year,month,day,hour,minute,second);
    // assert_eq!(dt.weekday(), Weekday::Fri);
    // assert_eq!(dt.weekday().number_from_monday(), 5); // Mon=1, ..., Sun=7

    /*
    {
    let tt = Local::now();
    println!("tt：{}",tt);
    let t = tt.hour();
    tt.year();
    tt.month();
    tt.day();
    tt.hour();
    tt.minute();
    tt.second();
    tt.timestamp_millis();
    tt.weekday();
    println!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",tt,
    tt.year(),              //年
    tt.month(),             //月
    tt.day(),               //天
    tt.hour(),              //小时
    tt.minute(),            //分
    tt.second(),            //秒
    tt.timestamp_millis(),  //时间戳
    tt.date(),              //日期
    tt.weekday(),           //周
    );
    let tmp = format!("{:04}|{:02}|{:02}",tt.year(),tt.month(),tt.day());
    print!("{}",tmp);
    /////////////////////////////////////////////////////
    let data = tt.timestamp_millis()/1000; //时间戳
    println!("时间戳；{}\n",data);
    let s = Local.timestamp_millis(data*1000).naive_local();//时间戳转换成时间
    println!("时间戳转换成时间；{}\n",s);
    }
    println!("--------");
    // // let tt = Local::now().naive_local();
    // println!("处理过的时间：{}\n",(tt.year() as i64)* 10000000000 +
    //                 (tt.month() as i64)* 100000000  +
    //                 (tt.day() as i64 )* 1000000 +
    //                 (tt.hour() as i64)* 10000 +
    //                 (tt.minute() as i64)* 100);//20211111165400

    let tt = Local::now().naive_local();//2021-11-11 16:54:20.415889600
    println!("日期时间（无时区）{}\n",tt);
    let data = tt.timestamp_millis()/1000;
    println!("时间戳：{}\n",data);//1636649660415
    let s = Local.timestamp_millis(data*1000).naive_local();//时间戳转换成时间
    println!("时间戳转换成时间；{}\n",s);

    println!("--------");
    let mut ss = tt.timestamp_millis();
    ss = ss - 15 * 60 -12 * 3600;//1636723108685
    let s = Local.timestamp_millis(1636652005).naive_local();//1636652005
    println!("处理过的时间：{}\n",(s.year() as i64)* 10000000000 +
                    (s.month() as i64)* 100000000  +
                    (s.day() as i64 )* 1000000 +
                    (s.hour() as i64)* 10000 +
                    (s.minute() as i64)* 100);

    */
}

//引用和克隆性能分析
fn performance() {
    let s = String::from("fdafdafdsafds");
    println!("原字符地址: {:?}", s.as_ptr()); //0x22252ea4b80

    let now = Instant::now();
    str(s.clone()); //创建内存                   //0x22252ea4de0
    println!("克隆耗时： {:?}", now.elapsed()); //3.2µs

    let now1 = Instant::now();
    ptr(&s); //0x22252ea4b80
    println!("引用耗时： {:?}\n", now1.elapsed()); //200ns

    //=====================================
    let s = &String::from("fdafdafdsafds");
    println!("原字符地址: {:?}", s.as_ptr()); //0x218a6791180
    let now = Instant::now();
    str(s.to_string()); //创建内存                   //0x218a67911a0
    println!("耗时： {:?}", now.elapsed()); //3.2µs

    let now1 = Instant::now();
    ptr(s); //0x218a6791180
    println!("耗时： {:?}\n", now1.elapsed()); //
    //==========================================================

    let struct_s = S::new();
    let key = &String::from("a");
    // let key = "a";
    let now1 = Instant::now();
    struct_s.str(key.clone());
    // struct_s.str(key.to_string());//3.9µs
    println!("耗时： {:?}", now1.elapsed());

    let now1 = Instant::now();
    struct_s.ptr(key);
    // struct_s.ptr(&key.to_string());//4µs
    println!("耗时： {:?}", now1.elapsed());
}

fn hash_map() {
    let mut mp: HashMap<i32, i32> = HashMap::new();
    mp.insert(1, 1);
    mp.insert(2, 2);

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("color".to_string(), "red".to_string());
    map.insert("size".to_string(), "10 m^2".to_string());
    println!("map : {:#?}", map);

    let mut map = HashMap::new();
    let val = map.insert(1, "罗".to_string());
    println!("map = {:#?}", map);
    println!("{:#?}", val);
}

fn vec() {
    let mut skline_value: String = String::from("");
    let mut k_line_hq_info: Vec<String> = Vec::new();
    {
        let s = "hhhh";
        let t = "aaaa";
        skline_value = format!("hello {} world {}", s, t);
        println!("skline_value: {}", skline_value);
        k_line_hq_info.push(skline_value);
        println!("len: {}", k_line_hq_info.len());
    }
    {
        let s = "k";
        let t = "q";
        skline_value = format!("hello {} world {}", s, t);
        println!("skline_value: {}", skline_value);
        k_line_hq_info.push(skline_value);
        println!("len: {}", k_line_hq_info.len());
    }
}

fn time() {
    // let mut st:Tm = time::now();
    // println!("{:#?}", st);
    // st.tm_hour = 15;
    // st.tm_min = 2;
    // st.tm_sec = 0;
    // println!("{}", st.strftime("%F %T").unwrap().to_string());

    // let time = time::now();
    // println!("time: {:#?}", time);
    // // let time = strptime("2022-04-13 09:29:30", "%F %T").unwrap();
    // // println!("time: {:#?}", time);

    // let date = time.strftime("%F %T").unwrap().to_string();//时间结构 转换成时间
    // println!("date: {:#?}", date);//date: "2022-04-03 15:38:15"

    // let mut tt = time.to_timespec().sec;//时间戳
    // println!("tt: {}",tt);
    // // let mut n_tt = time.to_timespec().nsec;//时间戳
    // // println!("n_tt: {}",n_tt);

    // tt -= 30;
    // println!("tt: {}",tt);

    // let tmp = strptime(tt.to_string().as_str(), "%s").unwrap();//时间戳 转换成时间结构
    // println!("tmp: {:#?}", tmp);
    // let date1 = tmp.strftime("%F %T").unwrap().to_string();
    // println!("date1: {:#?}", date1);
    // // println!("date1 tmp_tt: {:#?}", tmp.to_timespec().sec);

    // let time1 = strptime("15:02:00", "%T").unwrap();
    // // println!("time1: {:#?}", time1);
    // println!("{}", time1.strftime("%F %T").unwrap().to_string());
    // let time2 = time::now();
    // println!("{}", time2.to_timespec().sec);

    // let a = time::now();
    // for i in 0..100 {
    //     thread::sleep(Duration::from_millis(100));//10秒
    // }
    // let b = time::now();

    // let c = b - a;
    // println!("耗时:{}", c);

    //时间
    let time = time::now();
    let pp = time.rfc822();
    println!("rfc822: {:#?}", pp.to_string()); //rfc822: "Sun, 03 Apr 2022 11:00:31 "

    let ttt: i64 = 1649815862; //992//1649813005;//463;
    let time_tmp = strptime(ttt.to_string().as_str(), "%s").unwrap();
    // println!("{:#?}", time_tmp);
    let date3 = time_tmp.strftime("%F %T").unwrap().to_string();
    println!("date3: {:#?}", date3);

    let pp = time.rfc822z();
    println!("rfc822z: {:#?}", pp.to_string()); //rfc822z: "Sun, 03 Apr 2022 11:00:31 +0800"

    let pp = time.ctime();
    println!("ctime : {:#?}", pp.to_string()); //ctime : "Sun Apr  3 11:00:31 2022"

    let pp = time.asctime();
    println!("asctime : {:#?}", pp.to_string()); //asctime : "Sun Apr  3 11:00:31 2022"

    let pp = time.rfc3339();
    println!("rfc3339 : {:#?}", pp.to_string()); //rfc3339 : "2022-04-03T11:00:31+08:00"

    // Pin::new(&st).get_mut();

    let _s = time.tm_sec - 30;
    // let p = time.asctime();
    // println!("{:#?}",s);
    let p = time.to_timespec();
    println!("{:#?}", p);

    let t = time::get_time();
    println!("{:#?}", t);
    let _ = t.sec - 30;
    println!("{:#?}", t);

    println!("{:#?}", t.sec - 0);
    println!("{:#?}", t.nsec - 0);

    let dt = Utc.ymd(2001, 9, 9).and_hms_milli(1, 46, 40, 555);
    let s = dt.timestamp_millis();
    println!("{}", s);

    let clock = SystemTime::now();
    println!("{:#?}", clock);
    let time = Duration::as_nanos(&clock.elapsed().unwrap());
    println!("{:#?}", time);

    // time.tm_sec = 10;
    let time = time::now();
    let time2 = time::now();
    println!("{:#?}", (time2 - time));

    // println!("{:#?}",time.to_local());
    // println!("{:#?}",time.ctime());

    let time = time::now();
    println!("{:#?}", time);
}

fn dash_map() {
    //锁
    let map = DashMap::new();
    let mut q = Test::new();
    map.insert("a".to_string(), q.clone());
    q.str = 1;
    q.ptr = 1;
    map.insert("b".to_string(), q.clone());
    println!("============1");
    // let mut s = "".to_owned();
    // for i in map.iter() {
    //     println!("============2");
    //     if i.value().str == 1 {
    //         println!("============3");
    //         //  map.remove(&i.key());//死锁
    //         s = i.key().to_string();
    //         println!("============4");
    //     }
    //     println!("============5");

    // }
    // map.remove(&s);

    // if map.contains_key("b") {
        // 不会死锁
    //     map.remove("b");
    // }

    for i in 0..2 {
        q.str = i;
        q.ptr = i;
        let k = format!("{}", i);
        map.insert(k, q.clone());
    }

    if let Some(mut val) = map.get_mut("a") {
        //不会死锁
        val.ptr = 8888888;
        println!("存在");
    } else {
        println!("没有");
    }
    println!("map : {:#?}",map);

    let mut m = map.get_mut("a").expect("没有");
    drop(m);//释放,不释放阻塞
    //get_mut 和 get 不能同时存在(DashMap多线程,存在相互等待锁的问题,不能同时存在, 只能同时存在一个可变,可以同时存在多个不可变, 可变和不可变不能同时)
    // println!("=================");
    // println!("================={}", map.len());
    // let mut pppp = map.get("a").expect("没有");
    // println!("================={}", map.len());

    let now = Instant::now();
    if let Some(_m) = map.get_mut("300") {
        println!("找到了");
        println!("找到了");
        println!("找到了");
        println!("找到了");
        println!("找到了");
        println!("找到了");
        println!("找到了");
    } else {
        println!("没有");
    }

    println!("耗时: {:?}\n\n", now.elapsed());

    let now = Instant::now();
    for mut s in map.iter_mut() {
        let (k, _v) = s.pair_mut();
        if k == "1" {
            println!("找到了");
        }
    }
    println!("耗时: {:?}\n\n", now.elapsed());

    // if let Some(mut val) = map.get_mut("a") {
    //     //会死锁
    //     map.remove(val.key());
    //     println!("已删除");
    // } else {
    //     println!("没有");
    // };

    //==================================================这里可能出现死锁问题，同一作用域内，相互等待map
    // {
    // let a = time::now();
    // let mut val = map.get_mut(&"a".to_string()).unwrap();
    // let ppp = val.value_mut();
    // ppp.str = 9;
    // // println!("{}",i);
    // // }=============1
    // drop(val);//=======2用完后释放

    // map.get_mut(&"a".to_string()).unwrap().ptr = 1000;//这种方法不会造成死锁，能改变值

    // {//会死锁，能改变值
    //     let mut val = map.get_mut(&"b".to_string()).unwrap();
    //     val.ptr = 22222;
    // }
    // let b = time::now();
    // println!("耗时:{}", b - a);
    // println!("map : {:#?}",map);
    //=======================================================================================

    //------------------------------能改变map值,不会死锁
    let a = time::now();
    map.entry("a".to_string()).and_modify(|val| {
        val.ptr = 111;
        val.str = 222;
    });
    map.entry("b".to_string()).and_modify(|val| {
        val.ptr = 111;
        val.str = 222;
    }).or_insert_with(|| Test {
        ptr: 555,
        str: 666,
    });
    // //------------------------------------------------------
    // let b = time::now();
    // println!("耗时:{}", b - a);
    // println!("map : {:#?}",map);

    // let mut map1 = DashMap::new();
    // map1.insert("a".to_string(), 88888);
    // println!("map1 : {:#?}",map1);

    // {
    //     let mut val1 = map1.get_mut(&"a".to_string()).unwrap();
    //     let ll = val1.value_mut();
    //     *ll = 111;
    //     drop(val1);
    //     // drop(ll);释放这个没有
    // }
    // println!("map1 : {:#?}",map1);

    // {
    //     let mut f = *map1.get_mut(&"a".to_string()).unwrap();
    //     f = 5;//不能改变map的值
    // }

    // *map1.get_mut(&"a".to_string()).unwrap() = 5;//不会死锁,可以改变值

    // println!("map1 : {:#?}",map1);
}

fn thread() {
    //线程

    let id = thread_id::get();
    println!("[THD:{}]", id);
    // let s = std::thread::current().name();
    // let ss = s.clone();
    // {
    //     println!("::std::thread::current().id(): {:#?}", std::thread::current().id());
    // }

    for i in 0..4 {
        std::thread::spawn(move || {
            println!("[THD:{}] thread:{}", thread_id::get(), i);
        });
    }
    // for i in 1..3 {
    //     println!("id(): {} {}", thread_id::get(), i);
    // }
}

fn string() {
    let val = "2022-03-10 14:59:00.000".to_string();
    println!("str: {}", &val[11..19]);

    let abs = -7.00 - 0.00f64;
    println!("{}", abs.abs());
    let mut s = String::from("fdasfasGGGG112232");

    let a = s.to_case(Case::Upper); //引用self并将其转换为给定的大小写。(大写字符串由空格分隔，所有字符均为大写。)
    let c = s.to_uppercase(); //将此字符串片段的大写等效值作为新字符串返回。
    let d = s.to_string().make_ascii_uppercase();
    // let d = s.make_ascii_uppercase();//将此字符串就地转换为等效的ASCII大写字母。(在原来基础上改)
    let e = &s;

    println!("{}\n{}\n{}\n{:#?}\n{}", s, a, c, d, e);
    let _d = s.make_ascii_uppercase(); //将此字符串就地转换为等效的ASCII大写字母。(在原来基础上改)

    let day = "20211125".to_string();
    let year: i32 = day[0..4].parse().expect("");
    let month: i32 = day[4..6].parse().expect("");
    let day_: i32 = day[6..8].parse().expect("");
    println!("{}: {}-{}-{}", day, year, month, day_);

    let s = 1;
    let a = s;
    println!("{} {}", s, a);

    let en_close_price: i32 = Default::default();
    let s = String::default();
    println!("{} '{}'", en_close_price, s);
}

fn text() {
    let mut tmp: i64 = 20211124220208;
    let _tt = Local::now();
    let second = (tmp % 100) as u32; //秒
    tmp /= 100;
    let minute = (tmp % 100) as u32; //分
    tmp /= 100;
    let hour = (tmp % 100) as u32; //小时
    tmp /= 100;
    let day = (tmp % 100) as u32; //天
    tmp /= 100;
    let month = (tmp % 100) as u32; //月
    tmp /= 100;
    let year = (tmp) as i32; //年
    let s = "+08:00";
    let tt_tmp = format!(
        "{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2} {}",
        year, month, day, hour, minute, second, s
    );
    println!("{}", tt_tmp);

    let a = DateTime::parse_from_str(&tt_tmp, "%Y-%m-%d %H:%M:%S  %z").unwrap();
    println!("a:{}", a);
    let q = a.naive_local();
    println!("q:{}", q);
}

async fn string_write() {
    //性能: write! > fmt > +

    let mut str1 = String::new();
    let now = Instant::now();
    for val in 0..1000 {
        let _ = write!(str1, "{}|", val);
    }
    println!("write: {}, time: {:?}", str1, now.elapsed());

    let mut str2 = String::new();
    let now = Instant::now();
    for val in 0..1000 {
        str2 = str2 + &val.to_string() + "|";
    }
    println!("+: {}, time: {:?}", str2, now.elapsed());

    // let mut str3 = String::new();

    // let now = Instant::now();
    // for val in 0..1000 {
    //     let _ = write!(str3, "|");
    // }
    // println!("+: {}, time: {:?}", str3, now.elapsed());

    let mut str4 = String::new();
    let now = Instant::now();
    for val in 0..1000 {
        str4 = str4 + &format!("{}|", val);
    }
    println!("fmt: {}, time: {:?}", str4, now.elapsed());

    // time: 1.65µs
    // time: 1.11µs
    // Bad
    // let now = Instant::now();
    // let mut vec1 = Vec::with_capacity(10);
    // let mut vec2 = Vec::with_capacity(10);
    // println!("time: {:?}", now.elapsed());

    // vec1.resize(10, 0);
    // vec2.extend(repeat(0).take(10));

    // // Good
    // let now = Instant::now();
    // let mut vec1 = vec![0; 10];
    // let mut vec2 = vec![0; 10];
    // println!("time: {:?}", now.elapsed());

    // let now = Instant::now();
    // let mut vec1: Vec<i32> = Vec::new();
    // let mut vec2: Vec<i32> = Vec::new();
    // println!("time: {:?}", now.elapsed());
}

async fn string_cmp() -> String {
    if ("09:15:00".."09:30:00").contains(&"09:30:00") {
        println!("在范围内...");
    }
    let mut btime = String::from("09:15:00");
    let mut etime = String::from("09:30:00");
    let time = String::from("09:20:00");

    // if (btime..etime).contains(&time) {
    //     println!("在范围内...");
    //     return etime
    // }
    if (btime..etime.clone()).contains(&time) {
        //[09:15:00,09:30:00)
        println!("在范围内...");
        return etime; //开市 盘前
    }

    time
}


async fn async_channel() {
    // 多对多，一个消息只能一个接收使用
    let (tx, rx1) = async_channel::unbounded::<i32>();
    let tx1 = tx.clone();

    let rx2 = rx1.clone();
    let rx3 = rx1.clone();
    let rx4 = rx1.clone();
    let _rx5 = rx1.clone();
    let _rx6 = rx1.clone();
    tokio::spawn(async move {
        for i in 1..10 {
            if let Err(e) = tx.send(i).await {
                println!("error: {}", e);
            }
        }
    });
    tokio::spawn(async move {
        for i in 10..20 {
            if let Err(e) = tx1.send(i).await {
                println!("error: {}", e);
            }
        }
    });

    tokio::spawn(async move {
        loop {
            tokio::select! {
                task1 = rx1.recv() => {
                    if let Ok(data) = task1 {
                        println!("rx1 接收: {}", data);
                        // drop(data)
                    }
                }
                task2 = rx2.recv() => {
                    if let Ok(data) = task2 {
                        println!("rx2 接收: {}", data);
                        // drop(data)
                    }
                }
                task3 = rx3.recv() => {
                    if let Ok(data) = task3 {
                        println!("rx3 接收: {}", data);
                        // drop(data)
                    }
                }
            }
        }
    });
    tokio::spawn(async move {
        loop {
            tokio::select! {
                task4 = rx4.recv() => {
                    if let Ok(data) = task4 {
                        println!("rx4 接收: {}", data);
                        // drop(data)
                    }
                }
            }
        }
    });
    // tokio::spawn(async move {
    //     loop{
    //         tokio::select! {
    //             task5 = rx5.recv() => {
    //                 if let Ok(data) = task5 {
    //                     println!("rx5 接收: {}", data);
    //                 }
    //             }
    //         }
    //     }
    // });
    // tokio::spawn(async move {
    //     loop{
    //         tokio::select! {
    //             task6 = rx6.recv() => {
    //                 if let Ok(data) = task6 {
    //                     println!("rx6 接收: {}", data);
    //                 }
    //             }
    //         }
    //     }
    // });
    std::thread::sleep(Duration::from_millis(10));
}


async fn tokio_channel() {
    // let (s, r) = unbounded();
    // s.send(1).unwrap();
    // s.send(2).unwrap();
    // s.send(3).unwrap();

    // // The only sender is dropped, disconnecting the channel.
    // //唯一的发送器被丢弃，断开了通道。
    // drop(s);

    // // The remaining messages can be received.
    // //剩下的信息可以接收。
    // println!("{:?}", r.recv());
    // println!("{:?}", r.recv());
    // println!("{:?}", r.recv());

    // // There are no more messages in the channel.
    // // 频道中没有更多消息。
    // println!("{}", r.is_empty());

    // // Note that calling `r.recv()` does not block.
    // // Instead, `Err(RecvError)` is returned immediately.
    // println!("{:?}", r.recv());

    //多 - 多
    // //不管接收端编号是多少, 排在前面的先接受
    // let (s1, r1) = unbounded();
    // let (s2, r2) = (s1.clone(), r1.clone());
    // let (s3, r3) = (s2.clone(), r2.clone());

    // s1.send(10).unwrap();
    // s2.send(20).unwrap();
    // s3.send(30).unwrap();

    // // assert_eq!(r3.recv(), Ok(10));
    // // assert_eq!(r1.recv(), Ok(20));
    // // assert_eq!(r2.recv(), Ok(30));
    // println!("{:?}", r1.recv());
    // println!("{:?}", r2.recv());
    // println!("{:?}", r3.recv());

    // 多对多 ，一个消息可以多个接收使用
    let (tx1, mut rx1) = broadcast::channel(16);
  

    let tx2 = tx1.clone();
    let mut rx1 = tx2.subscribe();
    let mut rx2 = tx2.subscribe();
    let mut rx3 = tx1.subscribe();
    let mut rx4 = tx1.subscribe();
    let x = rx1.recv();

    tokio::spawn(async move {
        for i in 1..10 {
            // println!("i = {}", i);
            if let Err(e) = tx1.send(i) {
                println!("{}", e);
            }
        }
    });
    tokio::spawn(async move {
        for i in 11..20 {
            // println!("i = {}", i);
            if let Err(e) = tx2.send(i) {
                println!("{}", e);
            }
        }
    });

    tokio::spawn(async move {
        loop {
            tokio::select! {
                // task1 = rx1.recv() => {
                //     if let Ok(data) = task1 {
                //         println!("rx1 接收: {}", data);
                //     }
                // }
                task2 = rx2.recv() => {
                    if let Ok(data) = task2 {
                        println!("rx2 接收: {}", data);
                    }
                }
                task3 = rx3.recv() => {
                    if let Ok(data) = task3 {
                        println!("rx3 接收: {}", data);
                    }
                }
            }
        }
    });

    tokio::spawn(async move {
        loop {
            tokio::select! {
                task4 = rx4.recv() => {
                    if let Ok(data) = task4 {
                        println!("rx4 接收: {}", data);
                    }
                }
                // task5 = rx5.recv() => {
                //     if let Ok(data) = task5 {
                //         println!("rx5 接收: {}", data);
                //     }
                // }
            }
        }
    });

    std::thread::sleep(Duration::from_millis(10));
    // tx.send(10).unwrap();
    // tx.send(20).unwrap();
}

fn fmt() {
    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);
    // You can pad numbers with extra zeroes,
    //and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1);
    // You can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number=1, width=5);
}

struct SS {
    pub id: Arc<UidgenService>,
}
impl SS {
    pub async fn new() ->Self{
        SS {
           id: Arc::new(UidgenService::new(1,1))
        }
    }
}
async fn lock() {
    let uid = Arc::new(tokio::sync::RwLock::new(UidgenService::new(1,1)));
    let mut id = UidgenService::new(2,1);

    println!("{}", id.get_uid());
    println!("{}", id.get_uid());
    println!("{}", id.get_uid());
    println!("{}", id.get_uid());
    println!("{}", id.get_uid());
    println!("{}", id.get_uid());

    println!("========={}", size_of_val(&uid));
    let mut w = uid.write().await;
    println!("========={}", size_of_val(&w));
    let mut w = UidgenService::new(1,1);
    println!("========={}", size_of_val(&w));
    println!("========={}", size_of_val("11111"));
    // let s = SS::new().await;
    // let mut w= s.id;
    // println!("1===={:#?}", w.get_uid());
    // println!("2===={:#?}", w.get_uid());
    // println!("3===={:#?}", w.get_uid());
    // println!("4===={:#?}", w.get_uid());
    // println!("5===={:#?}", w.get_uid());
    // tokio::spawn(async move {
    //     for i in 1..5 {
    //         let mut w = Arc::new(UidgenService::new(1,1));
    //         println!("========={}", size_of_val(&w));
    //         let s = SS::new().await;
    //         let mut w= s.id.as_ref().to_owned();
    //         println!("1===={:#?}", w.get_uid());
    //         println!("2===={:#?}", w.get_uid());
    //         println!("3===={:#?}", w.get_uid());
    //         println!("4===={:#?}", w.get_uid());
    //         println!("5===={:#?}", w.get_uid());
    //     }
    // });

    let mut w = SummarizedTickData::new();
    println!("原始=========size:{}", size_of_val(&w));
    w.average_price = 1.1;
    w.last_price = 111.255;
    w.low_price = 65.2465;
    println!("赋值=========size:{}", size_of_val(&w));
    let s = Arc::new(w.clone());
    println!("Arc=========size:{}", size_of_val(&s));
    let pp = &w;
    println!("引用=========size:{}", size_of_val(&pp));
    let ppp = String::from("ffdsaf");
    let v: Vec<i32> = vec![1,2,3,4,5,6,7];
    let s = v.into_iter().find(|&x| x == 1);
    // into_iter: 借用后不归还
    // iter: 借用后归还
    // println!("{:#?}", v);
}


fn f64_decimal(){
    let s = Decimal::new(1234, 3);
    println!("{}", s);
    let v = dec!(2);
    println!("{}", v);

    println!("{}", s * v);


    let pp = 123;
    // println!("{}", v/Decimal::from(0));

    // println!("{}", 1/0)
    
    //判断f32数据是否为0
    if 0.0 <= 0.000001 || 0.0 >= -0.000001 {
        println!("===============");
        println!("{}", 48 as char);
        println!("{}", 65 as char);
        println!("{}", 97 as char);
        let s = String::from("love: ❤️");
        println!("{}", s);
        println!("{}", s.to_ascii_uppercase());
        println!("{}", s.to_ascii_lowercase());

        let s = String::from("❤️");


        // for _ in 1..=50 {
        //     for _ in 1..=50 {
        //         print!("{}", s);
        //     }
        //     println!();
        // }

        let a = 1.0;
        let b = 0;
        println!("{}", a / b as f64);
    }
}

async fn Rw() {
    let ticks_cache: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(Vec::new()));
    {
        let mut tick = ticks_cache.write().await;
        tick.push(format!("=================={}=============", 1));

        for i in 0..50 {
            tick.push(format!("=================={}=============", i));
        }

    }
    {
        let tick = ticks_cache.write().await;
        if let Some(s) = tick.iter().find(|&x| x == "==================1=============") {
            println!("{:#?}", s);
        }

    }
    {
        let mut tick = ticks_cache.write().await;
        let s = tick.clone();
        tick.clear();
        drop(tick);
        println!("---{:#?}", s);
    }
    {
        let tick = ticks_cache.write().await;
        println!("{:#?}", tick);
    }
    ticks_cache.write().await.push(format!("=================={}=============", 1));
    ticks_cache.write().await.push(format!("=================={}=============", 2));
    ticks_cache.write().await.push(format!("=================={}=============", 3));
    // let mut s = ticks_cache.write().await;
    // let mut t = ticks_cache.write().await;
    // let mut p = ticks_cache.read().await;
    // s.push(format!("=================={}=============", 3));
    println!("{:#?}", ticks_cache);
}

async fn error() -> Result<()> {
    let s = "afdafda发达省份的";
    let now = std::time::Instant::now();
    println!("persist completed, elapsed1: {:?}", now.elapsed());
    println!("persist completed, elapsed2: {:?}", now.elapsed());
    println!("persist completed, elapsed3: {:?}", now.elapsed());
    println!("persist completed, elapsed4: {:?}", now.elapsed());
    println!("persist completed, elapsed5: {:?}", now.elapsed());
    let ab = "20230309,1".to_string();
    println!("{}", ab.len());
    // println!("{}", ab);
    // if ab.rfind("1").is_none() {
    //    println!("{}", ab); 
    // }
    // if ab.rfind("1") != Some(8) {
    //     println!("===========");
    // }
    // // println!("==========={:#?}",ab.as_bytes());
    // // println!("==========={:#?}",ab.as_bytes()[9] != 49);
    // if ab.as_bytes()[9] != 49 {
    //     println!("==========={:#?}",ab.as_bytes());
    // }

    // // let a = ab[9..1];
    // // println!("{}", a);

    return Err(anyhow!(s));
}


// #![feature(round_char_boundary)]
// let s = "❤️🧡💛💚💙💜";
// assert_eq!(s.len(), 26);
// assert!(!s.is_char_boundary(13));

// let closest = s.ceil_char_boundary(13);
// assert_eq!(closest, 14);
// assert_eq!(&s[..closest], "❤️🧡💛");
