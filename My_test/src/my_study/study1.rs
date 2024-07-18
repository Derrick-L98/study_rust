use time::strptime;
use time::*;
use chrono::prelude::*;
use chrono::offset::Local;
use chrono::{DateTime, TimeZone, Utc};

use std::time::Instant;
use std::time::*;
use std::time::Duration;
use convert_case::{Case, Casing};
use std::thread;

/// 测试 time 模块
/// cargo test -p time --lib tests::test_time
#[test]
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


/// 测试 thread 模块
/// cargo test -p time --lib tests::test_thread
#[test]
fn thread() {
    //线程

    let id = thread_id::get();
    println!("[main THD:{}]", id);
    // let s = std::thread::current().name();
    // let ss = s.clone();
    // {
    //     println!("::std::thread::current().id(): {:#?}", std::thread::current().id());
    // }

    for i in 0..4 {
        let t = std::thread::spawn(move || {
            println!("[THD:{}] thread:{}", thread_id::get(), i);
        });
        let _ = t.join();
    }
    // for i in 1..3 {
    //     println!("id(): {} {}", thread_id::get(), i);
    // }
}



/// 测试 string 模块
/// cargo test -p time --lib tests::test_string
#[test]
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