use anyhow::Result;
use anyhow::anyhow;
use crate::config::settings::RedisConfig;

// use mobc::Pool;
use mobc_redis::redis;
use mobc_redis::mobc::Pool;
use mobc_redis::redis::AsyncCommands;
use mobc_redis::RedisConnectionManager;

pub struct RedisClient {
    pool: Pool<RedisConnectionManager>,
}

impl RedisClient {
    pub async fn new(config: &RedisConfig) ->Result<Self> {
        if config.redisaddrs.is_empty() {
            log::error!("无连接服务器地址");
            return Err(anyhow!("无连接服务器地址"));
        }
        let url = format!("redis://:{}@{}/{}",config.password, config.redisaddrs, config.redisdb);

        let client = redis::Client::open(url).unwrap();

        let manager = RedisConnectionManager::new(client);
        let pool = Pool::builder().max_open(20).build(manager);
        Ok(RedisClient{ pool })
    }

    pub async fn get_value(&self, key: &String) -> Option<String> {
        let mut conn = self.pool.get().await.unwrap();
        let value = match conn.get(key).await {
            Ok(val) => Some(val),
		    Err(err) => {
                log::error!("redis error: {:#?}", err);
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
                log::error!("redis error: {:#?}",err);
                None
            }
        };
        value
    }

    pub async fn set_value(&self, key: &String, value: &String) -> Option<String>{
        log::info!("set_value begin..");
        let mut conn = self.pool.get().await.unwrap();

        // let ret: i32 =  conn
        //     .set(key,value)
        //     .await
        //     .unwrap();

        // log::info!("SET Value {}", ret);
        log::info!("set_value begin..");
        let val = match conn.set(key, value).await {
            Ok(v) => {
                // log::info!("val: {}", v);
                Some(v)
            },
            Err(err) => {
                log::error!("redis error: {:#?}",err);
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

        // log::info!("HSET Value {}", ret);
        // Some(ret)

        //hget 成功返回 0
        let val = match conn.hset(key,field,value).await {
            Ok(v) => Some(v),
            Err(err) => {
                log::error!("redis error: {:#?}",err);
                None
            }
        };
        // log::info!("HSET Value {:#?}", val);

        val

    }

    pub async fn set_ex_value(&self, key: &String, value: &String, seconds: i64) -> Option<String>{
        let mut conn = self.pool.get().await.unwrap();

        //set_ex 成功返回Ok
        // let ret: String = conn
        //     .set_ex(key, value, seconds.try_into().unwrap())
        //     .await
        //     .unwrap();

        // log::info!("SET EX Value {}", ret);

        let val = match conn.set_ex(key, value, seconds.try_into().unwrap()).await {
            Ok(v) => Some(v),
            Err(err) => {
                log::error!("redis error: {:#?}",err);
                None
            }
        };
        val
    }


    #[allow(dead_code)]
    pub async fn get_redis_con(&self) -> &Pool<RedisConnectionManager> {
        &self.pool
    }
}