use std::borrow::Borrow;
use dashmap::DashMap;
use std::time::Instant;

use std::collections::HashMap;
use chrono::prelude::*;
use chrono::offset::Local;
use chrono::{DateTime, TimeZone, Utc};

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;



use cached::{Cached, TimedSizedCache};



use crate::common::SummarizedTickData;



/// 不同舍入策略
/// https://docs.rs/rust_decimal/1.13.0/rust_decimal/struct.RoundingStrategy.html
/// 
#[test]
fn precision() {
    let sp: Decimal = dec!(6.5);
    println!("1:       {}", sp.round_dp_with_strategy(0, RoundingStrategy::MidpointNearestEven));//当一个数字介于另外两个数字之间时，它会向最近的偶数取整。也称为“银行家取整”
    println!("2:       {}", sp.round_dp_with_strategy(0, RoundingStrategy::MidpointAwayFromZero));//当一个数字介于另外两个数字之间时，它会向离零最近的数字四舍五入
    println!("3:       {}", sp.round_dp_with_strategy(0, RoundingStrategy::MidpointTowardZero));//当一个数字位于另外两个数字的中间时，它会向接近零的数字四舍五入
    println!("4:       {}", sp.round_dp_with_strategy(2, RoundingStrategy::ToZero));//数字总是向零四舍五入    X
    println!("5:       {}", sp.round_dp_with_strategy(2, RoundingStrategy::AwayFromZero));//数字总是从零开始四舍五入   X
    println!("6:       {}", sp.round_dp_with_strategy(2, RoundingStrategy::ToNegativeInfinity));//数字总是向负无穷大四舍五入   X
    println!("7:       {}", sp.round_dp_with_strategy(2, RoundingStrategy::ToPositiveInfinity));//数字总是向正无穷大取整    X
    println!("8:       {}", sp.round_dp_with_strategy(2, RoundingStrategy::BankersRounding));//当一个数字介于另外两个数字之间时，它会向最近的偶数取整
    println!("9:       {}", sp.round_dp_with_strategy(2, RoundingStrategy::RoundHalfUp));//如果值>=5，则向上取整，否则向下取整
    println!("10:      {}", sp.round_dp_with_strategy(2, RoundingStrategy::RoundHalfDown));//如果值=<5，则向下取整，否则向上取整
    println!("11:      {}", sp.round_dp_with_strategy(2, RoundingStrategy::RoundDown));//向下舍入
    println!("12:      {}", sp.round_dp_with_strategy(2, RoundingStrategy::RoundUp));//向上舍入
}


/// 缓存测试
/// https://docs.rs/cached/0.7.0/cached/
#[test]
fn cache() {
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


/// 保留小数多少位
fn meh(float: f64, precision: usize) -> String {
    format!("{0:.1$}", float, precision)
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


/// 可变引用与不可变引用
#[test]
fn mut_no_mut_test() {
    mut_no_mut();
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

fn mut_s(val: String) -> Result<String, Box<dyn std::error::Error>> {
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


/// 死锁测试
#[test]
fn dash_map_test() {
    dash_map();
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

    // =========================================================
    //循环检查删除
    // let mut s = "".to_owned();
    // for i in map.iter() {//锁给到i
    //     println!("============2");
    //     if i.value().str == 1 {
    //         println!("============3");
    //             // map.remove(i.key());//死锁
    //         s = i.key().to_string();
    //         println!("============4: {}", s);
    //     }
    //     println!("============5");
    // }
    // map.remove(&s);

    //查询删除
    // if map.contains_key("b") {
    //     // 不会死锁
    //     map.remove("b");
    // }

    // ========================================

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

    //锁给到m
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
        _v.ptr = 777777;
    }
    println!("{:#?}", map);
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


/// 时间测试
#[test]
fn test_time() {
    naive_date_time();
}

//chrono::NaiveDateTime
pub fn naive_date_time() {
    let now = Local::now().timestamp_millis();
    println!("now: {}", now);
    //给定字符串构造时间结构
    let time_uct: DateTime<Utc> = ("2022-05-05 14:21:48.000".to_owned() + "Z")
        .parse()
        .unwrap(); //正常
        // println!("{}", _time_uct);//2022-05-05 14:21:48 UTC
        // println!("{:?}", _time_uct);//2022-05-05T14:21:48Z
        let utc_data_local = time_uct.naive_local();
        println!("utc_data_local==1====={}", utc_data_local); //2022-05-05 14:21:48
        println!("utc_data_local==2====={:?}", utc_data_local); //2022-05-05T14:21:48
        println!("utc_data_local==3====={}\n", utc_data_local.and_utc().timestamp()); //时间戳1651760508 2022-05-05 22:21:48
        let utc_data_utc = time_uct.naive_utc();
        println!("utc_data_utc==1====={}", utc_data_utc); //2022-05-05 14:21:48
        println!("utc_data_utc==2====={:?}", utc_data_utc); //2022-05-05T14:21:48
        println!("utc_data_utc==3====={}\n", utc_data_utc.and_utc().timestamp()); //时间戳1651760508 2022-05-05 22:21:48 // 多8小时

    let time_local: DateTime<Local> = ("2022-05-05 14:21:48.000".to_owned() + "Z")
        .parse()
        .unwrap();
        // println!("{}", time);//2022-05-05 22:21:48 +08:00
        // println!("{:?}", time);//2022-05-05T22:21:48+08:00
        let local_data_local = time_local.naive_local();
        println!("local_data_local==1====={}", local_data_local); //2022-05-05 22:21:48
        println!("local_data_local==2====={:?}", local_data_local); //2022-05-05T22:21:48
        println!("local_data_local==3====={}\n", local_data_local.and_utc().timestamp()); //时间戳1651789308 2022-05-06 06:21:48 //多16小时
        let local_data_utc = time_local.naive_utc();
        println!("local_data_utc==1====={}", local_data_utc); //2022-05-05 14:21:48
        println!("local_data_utc==2====={:?}", local_data_utc); //2022-05-05T14:21:48
        println!("local_data_utc==3====={}\n", local_data_utc.and_utc().timestamp()); //时间戳1651760508 2022-05-05 22:21:48

    let mut dt =
        NaiveDateTime::parse_from_str("2022-05-05 14:21:48", "%Y-%m-%d %H:%M:%S%.3f").unwrap();
    println!("2022-05-05 14:21:48 ---> NaiveDateTime: {}", dt); //2022-05-05 14:21:48
    println!("时间戳: {}", dt.and_utc().timestamp()); //时间戳
    println!("周几: {:#?}", dt.iso_week());

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
        println!("1651760508 ---> NaiveDateTime: {}", pdt);//2022-05-05 14:21:48

        let ps = Local.timestamp_millis(1651760508 * 1000).naive_local();//多8小时
        // let ps = Local.timestamp_millis_opt(1651760508 * 1000).unwrap().naive_utc(); 
        // let ps = Local.timestamp_millis_opt(1651760508 * 1000).unwrap().naive_local(); 
        println!("1651760508 ---> NaiveDateTime: {}", ps);//2022-05-05 22:21:48

        let ps = Local.timestamp_millis(1651760508 * 1000).naive_utc();
        // let ps = Utc.timestamp_millis_opt(1651760508 * 1000).unwrap().naive_utc();
        // let ps = Utc.timestamp_millis_opt(1651760508 * 1000).unwrap().naive_local();
        println!("1651760508 ---> NaiveDateTime: {}\n", ps);//2022-05-05 14:21:48
    }


    //需要时间戳用naive_utc,需要时间字符串用naive_local
    // println!("current ---> 时间戳: {}\n", cop.timestamp());//0.4.35版本过后起, 不推荐使用,推荐使用下面的方式
    let cop = chrono::Local::now().naive_utc();
    println!("current naive_utc ---> NaiveDateTime: {}", cop);//2024-06-21 09:25:56.296871900
    println!("current naive_utc---> 时间戳: {}", cop.and_utc().timestamp());//1718961956  2024-06-21 17:25:56
    let cop = chrono::Local::now().naive_local();
    println!("current naive_local---> NaiveDateTime: {}", cop);//2024-06-21 17:25:56.296874100
    println!("current naive_local---> 时间戳: {}\n", cop.and_utc().timestamp());//1718990756 2024-06-22 01:25:56

    let sss = cop.format("%Y%m%d%H%M%S").to_string();
    let d = cop.format("%Y%m%d").to_string();
    let t = cop.format("%H%M%S").to_string();
    println!("ssss: {}", sss);
    println!("dddd: {}", d);
    println!("tttt: {}", t);
    let da = sss.parse::<i64>().unwrap();
    println!("SSSSS: {}", da);

    let end_time = (cop.year() as i64) * 10000000000
        + (cop.month() as i64) * 100000000
        + (cop.day() as i64) * 1000000
        + (cop.hour() as i64) * 10000
        + (cop.minute() as i64) * 100;
    println!("SSSSS: {}", end_time);

    let new_st = chrono::Local::now().naive_local();
    println!("new: {}\n", new_st);

    let now = Local::now();//str
    let ndt = now.naive_local();//chrono::NaiveDateTime
    let res = match Local.from_local_datetime(&ndt).single() {
        Some(v) => v,
        None => panic! {"Required for test!"},
    };//chrono::DateTime<chrono::Local>
    println!("now:{}\nndt:{}\nres:{}",now, ndt,res);


    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();
    println!("utc = {}\nlocal = {}",utc, local);

    println!("Hello, world!");
    let dt = Local.ymd(2014, 7, 24).and_hms(12, 34, 6);
    println!("{}",dt);
    let dt = FixedOffset::east(9*3600).ymd(2014, 11, 28).and_hms_nano(21, 45, 59, 324310806);
    let year = dt.year();
    let month = dt.month();
    let day = dt.day();
    let hour = dt.hour();
    let minute = dt.minute();
    let second = dt.second();
    println!("{}\n{}-{}-{} {}:{}:{}",dt,year,month,day,hour,minute,second);
    assert_eq!(dt.weekday(), Weekday::Fri);
    assert_eq!(dt.weekday().number_from_monday(), 5); // Mon=1, ..., Sun=7

    
    {
    let tt = Local::now();
    println!("tt：{}",tt);
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

    let tt = Local::now().naive_local();//2021-11-11 16:54:20.415889600
    println!("日期时间（无时区）{}\n",tt);
    let data = tt.timestamp_millis()/1000;
    println!("时间戳：{}\n",data);//1636649660415
    let s = Local.timestamp_millis(data*1000).naive_local();//时间戳转换成时间
    println!("时间戳转换成时间；{}\n",s);
}


/// 引用和克隆性能测试
#[test]
fn reference_clone() {
    performance();
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
fn clone(key: String) {
    println!("{:?}", key.as_ptr());
}
fn quote(key: &String) {
    println!("{:?}", key.as_ptr());
}

//引用和克隆性能分析
fn performance() {
    let s = String::from("fdafdafdsafds");
    println!("原字符地址: {:?}", s.as_ptr()); //0x22252ea4b80
    let now = Instant::now();
    clone(s.clone()); //创建内存                   //0x22252ea4de0
    println!("克隆耗时： {:?}", now.elapsed()); //3.2µs
    let now1 = Instant::now();
    quote(&s); //0x22252ea4b80
    println!("引用耗时： {:?}\n", now1.elapsed()); //200ns

    //=====================================
    let s = &String::from("fdafdafdsafds");
    println!("原字符地址: {:?}", s.as_ptr()); //0x218a6791180
    let now = Instant::now();
    clone(s.to_string()); //创建内存                   //0x218a67911a0
    println!("耗时： {:?}", now.elapsed()); //3.2µs
    let now1 = Instant::now();
    quote(s); //0x218a6791180
    println!("耗时： {:?}\n", now1.elapsed()); //
    //==========================================================

    let struct_s = S::new();
    let key = &String::from("a");
    let now1 = Instant::now();
    struct_s.str(key.clone());
    // struct_s.str(key.to_string());//3.9µs
    println!("耗时： {:?}", now1.elapsed());

    let now1 = Instant::now();
    struct_s.ptr(key);
    // struct_s.ptr(&key.to_string());//4µs
    println!("耗时： {:?}", now1.elapsed());
}


/// 哈希表测试
#[test]
fn hash_map_test() {
    hash_map();
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


/// 向量测试
#[test]
fn vec_test() {
    vec();
}

fn vec() {
    let mut skline_value: String;
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