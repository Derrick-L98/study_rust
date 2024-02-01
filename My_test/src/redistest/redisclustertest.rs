use anyhow::Result;
use anyhow::anyhow;

use mobc::Pool;
// use mobc_redis_cluster::;
use mobc_redis_cluster::RedisClusterConnectionManager;
use mobc_redis_cluster::{Client, Connection};
use mobc_redis_cluster::redis::AsyncCommands;
use std::time::Instant;
// use redis_cluster_async::{Client, redis::cmd};


pub struct RedisClusterClient {
    pool: Pool<RedisClusterConnectionManager>,
}

impl RedisClusterClient {
    pub async fn new() ->Self {
        let nodes = vec!["redis://:pl9rz1L0hbOoacesE3Jh@52.131.255.193:7000/", "redis://:pl9rz1L0hbOoacesE3Jh@52.131.255.193:7001/", "redis://:pl9rz1L0hbOoacesE3Jh@52.131.255.193:7002/", "redis://:pl9rz1L0hbOoacesE3Jh@52.131.255.193:7003/", "redis://:pl9rz1L0hbOoacesE3Jh@52.131.255.193:7004/", "redis://:pl9rz1L0hbOoacesE3Jh@52.131.255.193:7005/" ];
        let client = Client::open(nodes).unwrap();
        let manager = RedisClusterConnectionManager::new(client);
        let pool = Pool::builder().max_open(16).build(manager);
        RedisClusterClient{ pool }
    }

    pub async fn get_value(&self, key: &String) -> Option<String> {
        let mut conn = self.pool.get().await.unwrap();
        let value = match conn.get(key).await {
            Ok(val) => Some(val),
		    Err(err) => {
                println!("redis error: {:#?}", err);
                None
            }
        };
        value
    }

    pub async fn hget_value(&self, key: &String, field: &String) -> Option<String> {
        let mut conn = self.pool.get().await.unwrap();

        let value = match conn.hget(key, field).await {
            Ok(val) => Some(val),
		    Err(err) => {
                println!("redis error: {:#?}",err);
                None
            }
        };
        value
    }

    pub async fn set_value(&self, key: &String, value: &String) -> Option<String>{
        println!("set_value begin..");
        let mut conn = self.pool.get().await.unwrap();

        // let ret: i32 =  conn
        //     .set(key,value)
        //     .await
        //     .unwrap();

        // println("SET Value {}", ret);
        println!("set_value begin..");
        let val = match conn.set(key, value).await {
            Ok(v) => {
                // println("val: {}", v);
                Some(v)
            },
            Err(err) => {
                println!("redis error: {:#?}",err);
                None
            }
        };
        
        val
    }

    pub async fn hset_value(&self, key: &String, field: &String, value: &String) -> Option<i32>{
        let mut conn = self.pool.get().await.unwrap();

        // let ret: i32 = conn
        //     .hset(key, field, value)
        //     .await
        //     .unwrap();

        // println("HSET Value {}", ret);
        // Some(ret)

        //hget 成功返回 0
        let val = match conn.hset(key,field,value).await {
            Ok(v) => Some(v),
            Err(err) => {
                println!("redis error: {:#?}",err);
                None
            }
        };
        // println("HSET Value {:#?}", val);

        val

    }

    pub async fn set_ex_value(&self, key: &String, value: &String, seconds: i64) -> Option<String>{
        let mut conn = self.pool.get().await.unwrap();

        //set_ex 成功返回Ok
        // let ret: String = conn
        //     .set_ex(key, value, seconds.try_into().unwrap())
        //     .await
        //     .unwrap();

        // println("SET EX Value {}", ret);

        let val = match conn.set_ex(key, value, seconds.try_into().unwrap()).await {
            Ok(v) => Some(v),
            Err(err) => {
                println!("redis error: {:#?}",err);
                None
            }
        };
        val
    }


    #[allow(dead_code)]
    pub async fn get_redis_con(&self) -> &Pool<RedisClusterConnectionManager> {
        &self.pool
    }
}