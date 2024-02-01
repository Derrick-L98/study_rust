// use crate::IVec;
// use sled::{Config as OtherConfig, /*Result*/};
// use sled::{Batch, open};



use {    
    byteorder::{BigEndian, LittleEndian},    
    zerocopy::{byteorder::U64, AsBytes, FromBytes, LayoutVerified, Unaligned,    
    },
};
// use vsdbsled::Db;
// use vsdbsled::{Config as OtherConfig, /*Result*/};
// use vsdbsled::{Batch, open};
// use vsdbsled::IVec;


use sled::Db;
use sled::{Config as OtherConfig, /*Result*/};
use sled::{Batch, open};
use sled::IVec;


// #[derive(FromBytes, AsBytes, Unaligned)]
// #[derive(FromBytes, AsBytes)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
// #[repr(C)]
pub struct Value {
    pub key: String,
    pub value: Vec<i32>,

}


pub async fn dbcache() {
    
    // 打开数据库let tree = sled::open("/tmp/welcome-to-sled").expect("open");
    // 插入KV，读取Key对应的值tree.insert("KEY1", "VAL1");assert_eq!(tree.get(&"KEY1"), Ok(Some(sled::IVec::from("VAL1"))));
    // 范围查询 for kv in tree.range("KEY1".."KEY9") {    ...}
    // 删除 tree.remove(&"KEY1");
    // atomic compare and swap，可以用在并发编程中tree.compare_and_swap("KEY1", Some("VAL1"), Some("VAL2"));
    // 阻塞直到所有修改都写入硬盘tree.flush();


    // let db = open("batch_db_2").unwrap();
    // //  键结构体
    // zerocopy::byteorder::U64;
    // //保证了数据对齐问题
    // #[derive(FromBytes, AsBytes, Unaligned)]
    // #[repr(C)]
    // struct Key {    
    //     a: U64<BigEndian>,    
    //     b: U64<BigEndian>,
    // }
    // // 值结构体
    // #[derive(FromBytes, AsBytes, Unaligned)]
    // #[repr(C)]
    // struct Value {    
    //     count: U64<LittleEndian>,    
    //     whatever: [u8; 16],
    // }

    // let key = Key { 
    //     a: U64::new(21), 
    //     b: U64::new(890) 
    // };
    // // 取得键所对应的值，并对其施加给定函数灿做
    // db.update_and_fetch(key.as_bytes(), |value_opt| {    
    //     if let Some(existing) = value_opt {        
    //         let mut backing_bytes = sled::IVec::from(existing);
    //         // 验证数据对齐（这里其实不是必须的，因为我们使用了U64）        
    //         let layout: LayoutVerified<&mut [u8], Value> = LayoutVerified::new_unaligned(&mut *backing_bytes).expect("bytes do not fit schema");
    //         // 得到底层数据的可变引用       
    //         let value: &mut Value = layout.into_mut();
    //         let new_count = value.count.get() + 1;
    //         println!("incrementing count to {}", new_count);
    //         value.count.set(new_count);
    //         Some(backing_bytes)    
    //     } else {        
    //         println!("setting count to 0");
    //         //  初始化一个Value        
    //         Some(
    //             sled::IVec::from(
    //                 Value{ 
    //                     count: U64::new(0), 
    //                     whatever: [0; 16] 
    //                 }.as_bytes()
    //             )) 
    //     }
    // }).unwrap();


    let db:Db = open("batch_db_2").unwrap();
    
    // //使用自己的键空间打开或创建一个新的磁盘备份树，可通过提供的标识符从Db访问
    // let cats = db.open_tree(b"cats").unwrap();
    // let dogs = db.open_tree(b"dogs").unwrap();

    // db.insert("KEY1", "VAL1");
    // db.insert("KEY2", "VAL2");
    // db.insert("KEY3", "VAL3");
    // db.insert("KEY4", "VAL4");
    // let data = db.get(&"KEY1").unwrap().unwrap();
    // // println!("{:#?}", data);
    // println!("{:#?}", String::from_utf8(data.to_vec()).unwrap());

    // // // atomic compare and swap，可以用在并发编程中
    // // let s = db.compare_and_swap("KEY1", Some("VAL1"), Some("VAL2")).unwrap().unwrap();
    // // // 范围查询
    // for kv in db.range("KEY1".."KEY9") {
    //     // println!("{:#?}", kv.unwrap());
    // }
    // // // 阻塞直到所有修改都写入硬盘
    // // db.flush();
    
    // let p = Value {
    //     key: "86856_XHSE".to_string(),
    //     value: vec![1, 2, 3],
    // };
    // let value = serde_json::to_string(&p).unwrap(); //序列化为字符串 
    // if let Err(err) = cats.insert("1", value.as_str()) {
    //     println!("{}", err);
    // }
    // let value = serde_json::to_string(&p).unwrap(); //序列化为字符串 
    // if let Err(err) = cats.insert(p.key, value.as_str()) {
    //     println!("{}", err);
    // }

    // let pdata = cats.get("1").unwrap().unwrap();
    // println!("1: {}", String::from_utf8(pdata.to_vec()).unwrap());

    // for i in cats.iter() {
    //     let s = i.map(|(k,v)|{
    //         println!("k: {:#?}", String::from_utf8(k.to_vec()).unwrap());
    //         // println!("v: {}", String::from_utf8(v.to_vec()).unwrap());
            
    //         let str = String::from_utf8(v.to_vec()).unwrap();
    //         let val: Value = serde_json::from_str(&str).expect("msg");
    //         println!("{:#?}", val);
           
    //     });
    // }

    let mut batch = Batch::default(); 

    let mut v = Value {
        key: "86856_XHSE".to_string(),
        value: vec![1, 2, 3],
    };
    // batch.insert("key_a", "val_a");
    // // batch.insert("key_b", "val_b");
    // // batch.insert("key_c", "val_c");
    // batch.insert("key", "value");

    let value = serde_json::to_string(&v).unwrap(); //序列化为字符串 
    batch.insert(v.key.as_str(), value.as_str());
    v.key = "fdsad".to_string();
    batch.insert(v.key.as_str(), value.as_str());
    db.apply_batch(batch).unwrap();

    // let a = db.get("key").unwrap().unwrap();
    // println!("{:#?}", String::from_utf8(a.to_vec()).unwrap());


    for i in db.iter() {
        let s = i.map(|(k,v)|{
            println!("k: {:#?}", String::from_utf8(k.to_vec()).unwrap());
            println!("v: {}", String::from_utf8(v.to_vec()).unwrap());
            
            let str = String::from_utf8(v.to_vec()).unwrap();
            let val: Value = serde_json::from_str(&str).expect("msg");
            println!("{:#?}", val);
           
        });
    }
}