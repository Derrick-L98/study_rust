extern crate chrono;
extern crate time;
extern crate wasm_bindgen_test;
//extern crate convert_case;
mod setting;
mod config;
mod common;


use crate::config::Config;
use crate::common::SummarizedTickData;

use self::chrono::prelude::*;
use self::wasm_bindgen_test::*;
use self::chrono::offset::Local;
use std::borrow::Borrow;
use std::collections::btree_map::ValuesMut;
use std::pin::Pin;
use std::sync::Arc;

use std::collections::{HashMap};
use convert_case::{Case, Casing};
use dashmap::DashMap;

use chrono::{DateTime, TimeZone, Utc};
use time::{Tm, at_utc, at, strptime};

// use time::*;
use std::thread;
use thread_id::{self};
use std::time::*;

use crossbeam_channel::{unbounded, RecvError, tick};
use tokio::sync::{mpsc, oneshot, RwLock};
use tokio::sync::watch;
use tokio::sync::broadcast;

use setting::Settings;
use cached::{proc_macro::cached, proc_macro::once, Cached, SizedCache, TimedCache, TimedSizedCache, UnboundCache};

#[tokio::main]
async fn main() {

    naive_date_time();
    // performance();
    // dash_map();
    // time();
    // thread();
    // hash_map();
    // vec();
    // string();
    // text();

    // mut_no_mut();

    // tokio_channel().await;
    // channel().await;
    // let now = Instant::now();
    // async_channel().await;
    // println!("耗时{:?}", now.elapsed());

    // std::thread::sleep(Duration::from_millis(0));


    // let setting = Settings::new().expect("失败");
    // let config = Config::new().await;
    // config.config(&setting).await;
    // println!("{:#?}", setting);
    

    
    // let s = 0.32f64.to_string();
    // println!("s = {}", s);

    // //s = 0.32 0 0.32 0 0.32 0 0.32
    // println!("s = {0} {1} {0} {1} {0} {1} {0}", s, 0);
    // println!("我的名字叫{name}, 今年{age}岁, 喜欢{hobby}", hobby = "打篮球", name = "张三", age = 18);

    // cache().await;

    //格式化输出保留小数后6位,不足补0
    let kk = format!("{:0<.6} {:.06} {:<.6} {:.6}", 1.0, 2.0, 3.0, 4.0);
    let k = format!("{:04} {:02} {:02} {:02} {:.20}", 1, 2, 3, 4, "fda");
    println!("{}\n{}", kk, k);
}

async fn cache() {
    let mut tickdata_cache2: TimedSizedCache<String, SummarizedTickData> = TimedSizedCache::with_size_and_lifespan(100, 60 * 60 * 80);
    
    let mut data = SummarizedTickData::new();
    data.stock_code = "601388_XSHG".to_string();
    data.close_price = 1.00;
    data.high_price = 5.00;
    data.last_price = 85.00;
    data.low_price = 0.1;
    data.total_turnover = 100000000.0;
    data.total_volume = 5000.0;
    tickdata_cache2.cache_set(data.stock_code.clone(), data.clone());

    let s = tickdata_cache2.cache_get_mut(&"601388_XSHG".to_string()).unwrap();
    s.open_price = 1555555555.0;
    data.stock_code = "601389_XSHG".to_string();
    tickdata_cache2.cache_set(data.stock_code.clone(), data.clone());
    data.open_price = 11111111.0;
    let p = tickdata_cache2.cache_set(data.stock_code.clone(), data.clone()).unwrap();//返回前一个同k的值
    // data.stock_code = "601311_XSHG".to_string();
    // tickdata_cache2.cache_set(data.stock_code.clone(), data.clone());
    // data.stock_code = "601300_XSHG".to_string();
    // tickdata_cache2.cache_set(data.stock_code.clone(), data.clone());
    // data.stock_code = "601319_XSHG".to_string();
    // tickdata_cache2.cache_set(data.stock_code.clone(), data.clone());
    // data.stock_code = "601378_XSHG".to_string();
    // tickdata_cache2.cache_set(data.stock_code.clone(), data.clone());
    
    

    println!("{:#?}", tickdata_cache2.get_store().get_order().borrow());

    println!("{:#?}", p);
    println!("{:#?}", tickdata_cache2.cache_get(&"601389_XSHG".to_string()).unwrap());
    // tickdata_cache2.cache_set(k, v)
    

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

    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();
    // let mut rx3 = tx.subscribe();
    let x = tx.clone();
    let mut rx2 = x.subscribe();
    // let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        for i in 1..10 {
            // println!("i = {}", i);
            if let Err(e) = tx.send(i) {
                println!("{}", e);
            }
        }
    });
    tokio::spawn(async move {
        for i in 11..20 {
            // println!("i = {}", i);
            if let Err(e) = x.send(i) {
                println!("{}", e);
            }
        }
    });
   

    tokio::spawn(async move {
        loop{
            tokio::select! {
                task1 = rx1.recv() => {
                    if let Ok(data) = task1 {
                        println!("rx1 接收: {}", data);
                    }
                }
                // task2 = rx2.recv() => {
                //     if let Ok(data) = task2 {
                //         println!("rx2 接收: {}", data);
                //     }
                // }
                // task3 = rx3.recv() => {
                //     if let Ok(data) = task3 {
                //         println!("rx3 接收: {}", data);
                //     }
                // }
            }
        }
    });

    tokio::spawn(async move {
        loop{
            tokio::select! {
                task2 = rx2.recv() => {
                    if let Ok(data) = task2 {
                        println!("rx2 接收: {}", data);
                    }
                }
                // task3 = rx3.recv() => {
                //     if let Ok(data) = task3 {
                //         println!("rx3 接收: {}", data);
                //     }
                // }
            }
        }
    });

    std::thread::sleep(Duration::from_millis(10));
    // tx.send(10).unwrap();
    // tx.send(20).unwrap();

}

async fn async_channel() {
    let (tx, rx1) = async_channel::unbounded::<i32>();
    // let tx1 = tx.clone();

    let rx2 = rx1.clone();
    let rx3 = rx1.clone();
    let rx4 = rx1.clone();
    let rx5 = rx1.clone();
    let rx6 = rx1.clone();
    tokio::spawn(async move {
        for i in 1..100000000 {
            if let Err(e) = tx.send(i).await {
                println!("error: {}", e);
            }
        }
    });
    // tokio::spawn(async move {
    //     for i in 101..200 {
    //         if let Err(e) = tx1.send(i).await {
    //             println!("error: {}", e);
    //         }
    //     }
    // });

    tokio::spawn(async move {
        loop{
            tokio::select! {
                task1 = rx1.recv() => {
                    if let Ok(data) = task1 {
                        // println!("rx1 接收: {}", data);
                        drop(data)
                    }
                }
                task2 = rx2.recv() => {
                    if let Ok(data) = task2 {
                        // println!("rx2 接收: {}", data);
                        drop(data)
                    }
                }
                task3 = rx3.recv() => {
                    if let Ok(data) = task3 {
                        // println!("rx3 接收: {}", data);
                        drop(data)
                    }
                }
            }
        }
    });
    tokio::spawn(async move {
        loop{
            tokio::select! {
                task4 = rx4.recv() => {
                    if let Ok(data) = task4 {
                        // println!("rx4 接收: {}", data);
                        drop(data)
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

//可变引用与不可变引用
fn mut_no_mut() {
    let mut val = String::from("4.160|4.150|4.150|0|20210323093000+4.147|4.150|284|20210323093100+4.148|4.160|24|20210323093200+4.148|4.150|9|20210323093400");
    println!("原来：{}",val);    
    let mut val = mut_s(val).unwrap();
        
        let s = &mut val;
        // let c = &mut val;//第二个可变借用发生在这里 error
        s.push_str("oob");
        // c.push_str("oob");

        let v = &val;
        let f = &val;
        println!("更改：{}",val);

        let s1 = String::from("run");
        let mut s2 = &s1;
        println!("{}", s2);
        // s2.push_str("oob"); // 错误，禁止修改租借的值
        println!("{}", s2);

        let mut s1 = Test {
            str: 1,
            ptr: 2,
        };
    
        
        test(&mut s1);
        println!("s1:{:#?}", s1);
}
struct S {
    pub map: DashMap<String, String>
}
impl S {
    fn new() -> Self{
        let mut map = DashMap::new();
        map.insert("a".to_string(), "aaaaaaaaaaaaaa".to_string());
        map.insert("b".to_string(), "bbbbbbbbbbbbbb".to_string());
        let s = S{
            map
        };
        s
    }
    fn str(&self, key: String) {
        if let Some(val) = self.map.get(&key) {

        }
    }
    fn ptr(&self, key: &String) {
        if let Some(val) = self.map.get(key) {
            
        }
    }
}
fn str(key: String) {
    println!("{:?}",key.as_ptr());
}
fn ptr(key: &String) {
    println!("{:?}",key.as_ptr());
}

fn mut_s(val: String) ->Result<String, Box<dyn std::error::Error>> {
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
        if !val.is_empty() {                                                            //返回true时为空
            if val.contains("+") { //如果给定模式与该字符串片段的子片段匹配，则返回true。如果没有，则返回false。
                value = val.split("+").collect();//按+分割为一周期
            }
        }
        if value.is_empty() {
            return Ok(val);
        }
        let mut val_tmp = String::default();
        for i in value.iter() {
            if let Some(index) =i.rfind('|') {//返回此字符串片段中模式最右边匹配项的第一个字符的字节索引。
                let update_time: i64 = i[index+1..].to_string().parse().expect("解析失败");//取由|分割开的最后一个值
                let local_time = "20211129103".to_string();
                //i.replace(i[index+1..],&local_time);
                let mut str_tmp = i.trim_end_matches(&update_time.to_string()).to_string();//删除末尾的字符串片段
                str_tmp.push_str(&local_time);
                val_tmp.push_str(&str_tmp);
            }
            val_tmp.push('+');
            
        }
        Ok(val_tmp)
    
}

#[derive(Clone, Debug)]
pub struct Test {
    str: i32,
    ptr: i32,
}

impl Test {
    pub fn new() -> Self{
        Test {
            str: 0,
            ptr: 0,
        }
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
    let mut time_uct: DateTime<Utc> = ("2022-05-05 14:21:48.000".to_owned() + "Z").parse().unwrap();//正常
    let mut time: DateTime<Local> = ("2022-05-05 14:21:48.000".to_owned() + "Z").parse().unwrap();
    let data = time.naive_local();
    println!("{}", data);//2022-05-05 22:21:48
    println!("{:?}", data);//2022-05-05T22:21:48
    println!("{}\n", data.timestamp());//时间戳
    
    let mut dt =NaiveDateTime::parse_from_str("2022-05-05 14:21:48.000", "%Y-%m-%d %H:%M:%S%.3f").unwrap();
    println!("2022-05-05 14:21:48.000 ---> NaiveDateTime: {}", dt);//2022-05-05 14:21:48
    println!("时间戳: {}", dt.timestamp());//时间戳
    // println!("周几: {:#?}", dt.iso_week());


    dt =  dt.with_year(2022)
            .unwrap()
            .with_month(5)
            .unwrap()
            .with_day(12)
            .unwrap()
            .with_hour(20)
            .unwrap();//更改分钟数后生成新值。当结果值无效时，返回None。


    println!("修改后的时间: {}", dt.time());//时间
    println!("修改后的日期: {}\n", dt.date());//日期
    
    
    let s = NaiveTime::parse_from_str("14:21:48", "%H:%M:%S").unwrap();
    println!("14:21:48 ---> NaiveTime: {}", s);
    println!("{}\n", s.hour()*60 + s.minute());

    let t =NaiveTime::parse_from_str("2022-05-05 14:21:48.000", "%Y-%m-%d %H:%M:%S%.3f").unwrap();
    println!("2022-05-05 14:21:48.000 ---> NaiveTime: {}\n", t);

    let d = NaiveDate::parse_from_str("2022-05-05", "%Y-%m-%d").unwrap();
    println!("2022-05-05 ---> NaiveDate: {}\n", d);

    //时间戳构造时间
    {
        let pdt = NaiveDateTime::parse_from_str("1651760508", "%s").unwrap();
        println!("1651760508 ---> NaiveDateTime: {}\n", pdt);

        let ps = Local.timestamp_millis(1651760508 * 1000).naive_local();//多8小时
        // let ps = Local.timestamp_millis(1651760508 * 1000).naive_utc();//正常
        println!("1651760508 ---> NaiveDateTime: {}\n", ps);
    }

    let cop = chrono::Local::now().naive_local();
    println!("current ---> NaiveDateTime: {}", cop);
    println!("current ---> 时间戳: {}\n", cop.timestamp());

    

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
    println!("{:?}",s.as_ptr());                //0x22252ea4b80
    let now = Instant::now();
    str(s.clone());//创建内存                   //0x22252ea4de0
    println!("耗时： {:?}", now.elapsed());//3.2µs

    let now1 = Instant::now();
    ptr(&s);                                    //0x22252ea4b80
    println!("耗时： {:?}\n", now1.elapsed());//200ns

    //=====================================
    let s = &String::from("fdafdafdsafds");
    println!("{:?}",s.as_ptr());                //0x218a6791180
    let now = Instant::now();
    str(s.to_string());//创建内存                   //0x218a67911a0
    println!("耗时： {:?}", now.elapsed());//3.2µs

    let now1 = Instant::now();
    ptr(&s);                                    //0x218a6791180
    println!("耗时： {:?}\n", now1.elapsed());//


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

    let mut map: HashMap<String,String> = HashMap::new();
    map.insert("color".to_string(), "red".to_string());
    map.insert("size".to_string(), "10 m^2".to_string());
    println!("map : {:#?}",map);

    let mut map = HashMap::new();
    let val = map.insert(1, "罗".to_string());
    println!("map = {:#?}",map);
    println!("{:#?}",val);
    
}

fn vec() {
    let mut sklineValue: String = String::from("");
    let mut k_line_hq_info: Vec<String> = Vec::new();
    {
        let s = "hhhh";
        let t = "aaaa";
        sklineValue = format!("hello {} world {}",s,t);
        println!("sklineValue: {}",sklineValue);
        k_line_hq_info.push(sklineValue);
        println!("len: {}",k_line_hq_info.len());
    }{
        let s = "k";
        let t = "q";
        sklineValue = format!("hello {} world {}",s,t);
        println!("sklineValue: {}",sklineValue);
        k_line_hq_info.push(sklineValue);
        println!("len: {}",k_line_hq_info.len());

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
    println!("rfc822: {:#?}", pp.to_string());//rfc822: "Sun, 03 Apr 2022 11:00:31 "

    let ttt: i64 = 1649815862;//992//1649813005;//463;
    let time_tmp = strptime(ttt.to_string().as_str(), "%s").unwrap();
    let date3 = time_tmp.strftime("%F %T").unwrap().to_string();
    println!("date3: {:#?}", date3);

    let pp = time.rfc822z();
    println!("rfc822z: {:#?}", pp.to_string());//rfc822z: "Sun, 03 Apr 2022 11:00:31 +0800"

    let pp = time.ctime();
    println!("ctime : {:#?}", pp.to_string());//ctime : "Sun Apr  3 11:00:31 2022"

    let pp = time.asctime();
    println!("asctime : {:#?}", pp.to_string());//asctime : "Sun Apr  3 11:00:31 2022"

    let pp = time.rfc3339();
    println!("rfc3339 : {:#?}", pp.to_string());//rfc3339 : "2022-04-03T11:00:31+08:00"

    let pp = time.asctime();
    println!("asctime : {:#?}", pp.to_string());//asctime : "Sun Apr  3 11:00:31 2022"

    let pp = time.asctime();
    println!("asctime : {:#?}", pp.to_string());

    let pp = time.asctime();
    println!("asctime : {:#?}", pp.to_string());

    // Pin::new(&st).get_mut();

    let s = time.tm_sec - 30;
    // let p = time.asctime();
    // println!("{:#?}",s);
    let p = time.to_timespec();
    println!("{:#?}",p);

    let mut t = time::get_time();
    println!("{:#?}", t);
    let _ = (t.sec -30);
    println!("{:#?}", t);

    
    println!("{:#?}", t.sec - 0);
    println!("{:#?}", t.nsec - 0);

    let dt = Utc.ymd(2001, 9, 9).and_hms_milli(1, 46, 40, 555);
    let s = dt.timestamp_millis();
    println!("{}",s);

    let clock = SystemTime::now();
    println!("{:#?}",clock);
    let time = Duration::as_nanos(&clock.elapsed().unwrap());
    println!("{:#?}",time);

    // time.tm_sec = 10;
    let mut time = time::now();
    let mut time2 = time::now();
    println!("{:#?}", (time2 - time));

    // println!("{:#?}",time.to_local());
    // println!("{:#?}",time.ctime());

    let time = time::now();
    println!("{:#?}", time);
}


fn dash_map() {
    
    //锁
    let mut map = DashMap::new();
    let mut q = Test::new();
    map.insert("a".to_string(), q.clone());
    q.str = 1;
    q.ptr = 1;
    map.insert("b".to_string(), q.clone());
    for i in 1..50 {
        q.str = i;
        q.ptr = i;
        let k = format!("{}", i);
        map.insert(k, q.clone());
    }

    if let Some(mut val) = map.get_mut("a") {//不会死锁
        val.ptr = 8888888;
        println!("存在");
    } else {
        println!("没有");
    }
    // println!("map : {:#?}",map);

    // let mut m = map.get_mut("888").expect("没有");
    // drop(m);

    let now = Instant::now(); 
    if let Some(m) = map.get_mut("300") {
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
        let (k, v) = s.pair_mut();
        if k == "30" {
            println!("找到了");
        }
    }
    println!("耗时: {:?}\n\n", now.elapsed());
 

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
    // let a = time::now();
    // map.entry("a".to_string()).and_modify(|val| {
    //     val.ptr = 111;
    //     val.str = 222;
    // });
    // map.entry("b".to_string()).and_modify(|val| {
    //     val.ptr = 111;
    //     val.str = 222;
    // }).or_insert_with(|| Test {
    //     ptr: 555,
    //     str: 666,
    // });
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
    
    for i in 0..4{
        std::thread::spawn(move ||{
            println!("[THD:{}] thread:{}", thread_id::get(), i);
        });
    } 
    // for i in 1..3 {
    //     println!("id(): {} {}", thread_id::get(), i);
    // }
}

fn string() {
    let val ="2022-03-10 14:59:00.000".to_string();
    println!("str: {}",&val[11..19]);

    

    let abs = -7.00-0.00f64;
    println!("{}",abs.abs());
    let mut s = String::from("fdasfasGGGG112232");
    
    let a = s.to_case(Case::Upper);//引用self并将其转换为给定的大小写。(大写字符串由空格分隔，所有字符均为大写。)
    let c = s.to_uppercase();//将此字符串片段的大写等效值作为新字符串返回。
    let d = s.to_string().make_ascii_uppercase();
    // let d = s.make_ascii_uppercase();//将此字符串就地转换为等效的ASCII大写字母。(在原来基础上改)
    let e = &s;
    
    println!("{}\n{}\n{}\n{:#?}\n{}",
        s, 
        a, 
        c, 
        d, 
        e);
    let d = s.make_ascii_uppercase();//将此字符串就地转换为等效的ASCII大写字母。(在原来基础上改)


    let day = "20211125".to_string();
    let year:i32 = day[0..4].parse().expect("");
    let month: i32 = day[4..6].parse().expect("");
    let day_ :i32 = day[6..8].parse().expect("");
    println!("{}: {}-{}-{}",day, year,month, day_); 
 

    let s = 1;
    let mut a = s;
    println!("{} {}",s, a);


    
    let mut en_close_price: i32 = Default::default();
    let s = String::default();
    println!("{} '{}'",en_close_price,s);
}

fn text() {
    let mut tmp:i64 = 20211124220208;
    let mut tt = Local::now();
    let second = (tmp % 100) as u32;//秒
    tmp /= 100;
    let minute = (tmp % 100) as u32;//分
    tmp /= 100;
    let hour = (tmp % 100) as u32;//小时
    tmp /= 100;
    let day = (tmp % 100) as u32;//天
    tmp /= 100;
    let month = (tmp % 100) as u32;//月
    tmp /= 100;
    let year = (tmp) as i32;//年
    let s = "+08:00";
    let tt_tmp = format!("{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2} {}",year, month, day, hour, minute, second, s);
    println!("{}",tt_tmp);

    let a = DateTime::parse_from_str(&tt_tmp, "%Y-%m-%d %H:%M:%S  %z").unwrap();
    println!("a:{}",a);
    let q = a.naive_local();
    println!("q:{}",q);
}