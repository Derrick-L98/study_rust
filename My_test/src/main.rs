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
mod my_study;

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

use std::time::Duration;
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
    // naive_date_time();
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



pub trait Point<T> {
    fn precision(&self, indix: usize) -> T;
} 
impl<T: std::str::FromStr + std::fmt::Display + std::default::Default> Point<T> for T {
    fn precision(&self, indix: usize) -> T {
        format!("{0:.1$}", self, indix).parse::<T>().unwrap_or_default()
    }
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
