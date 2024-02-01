
extern crate chrono;
#[macro_use]
extern crate rbatis;

extern crate time;

// use array::Array;
// use rbson::Array;
// use mysql::binlog::jsonb::Array;
use tokio::sync::{mpsc, RwLock};
use anyhow::Result;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections;

use std::sync::{Arc};
use std::vec;

use chrono::prelude::*;

mod sql;
use crate::sql::rb_sql::RbSql;
use crate::sql::sea_sql::SeaSql;
use crate::sql::sql::Sql;

use crate::sql::data::{System,SysCommodityGroupList};

// #[derive(Debug)]
// pub struct Sql {
//     // rb: MySqlPool,
//     rb: ,
// }

use time::*;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let uri = "mysql://companytest:ZNvC5NoLJNnO3Wn0@uat-stock-data.chinaeast2.cloudapp.chinacloudapi.cn:13301/finances_new?pool_min=0&pool_max=10";
    // let t_r = Local::now().naive_local().time();
    // let dbcon_r = RbSql::new(uri).await;
    // let t1_r = Local::now().naive_local().time();
    // println!("RbSql连接时间: {}",t1_r - t_r);

    // let t = Local::now().naive_local().time();
    // let dbcon_s = Sql::new(uri).await;
    // let t1 = Local::now().naive_local().time();
    // println!("Sql连接时间  : {}",t1 - t);

    
    // SysCommodityGroupList::query_many(2,&dbcon_r).await.unwrap();
    
    
    // System::sql_query(&dbcon_s).await;


///hashMap

//总结: RwLock: 读写锁=> 可以同时多个读,只能存在一个写,读写不能同时,读锁不能修改
    let map: Arc<RwLock<HashMap<String, i32>>> = Arc::new(RwLock::new(HashMap::new()));
    {
        let mut map_1 = map.write().await;
        map_1.insert("hello".to_string(), 1);
        map_1.insert("world".to_string(), 2);
        // map_1.remove("hello");
        // map_1.remove("world");

        print!("{:#?}",map_1);

        // let mut map_5 = map.write().await;
        // map_5.insert("world".to_string(), 3);
        // print!("{:#?}",map_5);
        if map_1.get("hello") == None {
            println!("fdasdfa");
        }

        if map_1.get("ha") == None {
            println!("fdasdfa");
        }
    }

    // // // drop(map_1);//记得释放,不然死锁,除非有作用域限制

    // let mut map_2 = map.read().await;
    // // map_2.remove(&"hello".to_string());
    // print!("{:#?}",map_2);

    // let map_3 = map.read().await;
    // print!("{:#?}",map_3);

    // let map_4 = map.read().await;
    // print!("{:#?}",map_4);

    // let vd : Arc<RwLock<VecDeque<i32>>>= Arc::new(RwLock::new(VecDeque::new()));
    
    // let mut q = vd.write().await;
    // q.push_back(1);
    // q.push_back(2);
    // q.push_back(3);
    // print!("{:#?}\n",q);
    // if let Some(num) = q.pop_front() {//删除第一个元素,并返回他
    //     print!("num = {:#?}\n",num);
    // };
    // print!("{:#?}\n",q);

    // q.pop_front();//头删
    // q.pop_back();//尾删
    // q.push_front(1);
    // q.push_front(2);
    // q.push_front(3);
    // print!("{:#?}",q);

    // let mut p = vd.read().await;
    // // if p.is_empty() {

    // // }
    // // p.pop_back();//尾删
    // print!("{:#?}",p);


    // let ma: HashMap<i32, i32> = HashMap::from([(1, 2), (3, 4)]);
    // print!("{:#?}",ma);

    // let ve = vec![1,2,3,4,5];
    // print!("{}",ve[0]);


// string

        // let mut data = vec![1,2,3,4];
        // let mut str = format!("|0|");

        // str = str + "100|";
        
        // for val in data {
        //     str =str + &val.to_string() + "|"
        // }

        // println!("{}",str);

        // let s = String::from("fad");
        // let b = String::from("fbd");
        // println!("{} {} {}", s > b, s == b, s < b);


    

    Ok(())
}
