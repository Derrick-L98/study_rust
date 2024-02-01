use anyhow::Result;
use common::redisclient::redisclient::RedisClient;
use crate::client::DbClient;
use crate::dataservice::entities::prelude::{PhoenixOrdStockorder, PhoenixOrdSuborder};


pub const ORDER:&'static str ="order_";  
pub const LOCK_ORDER:&'static str = "lock_order_";
pub const SUB_ORDER:&'static str ="sub_order_";  
// pub const LOCK_SUB_ORDER:&'static str = "lock_sub_order_";

//缓存超时时间常量
pub const EXPARE_TIME_1_HOUR:i64 = 1*60*60;
pub const EXPARE_TIME_8_HOUR:i64 = 8*60*60;
pub const EXPARE_TIME_1_DAY:i64 = 24*60*60;


#[derive(Debug, Clone, Default)]
pub struct CacheKey {
    pub unit_id: i64,
    pub order_id: i64,
    pub sub_id: i64,
    pub channel_id: i32,
    pub channel_type: i32,
}


impl CacheKey {
    pub async fn new() -> Self{
        CacheKey::default()
    }
    
    pub async fn init_order_cache(&self, redis: &RedisClient, db: &DbClient) -> Result<PhoenixOrdStockorder>{
        match PhoenixOrdStockorder::query_order(self.order_id, db).await {
            Ok(model) => {
                let _ = self.update_order_cache(&model, redis).await;
                Ok(model)
            }
            Err(err) => {
                log::error!("{:?}", &err);
                return Err(err);
            }
        }
    }

    pub async fn init_sub_order_cache(&self, redis: &RedisClient, db: &DbClient) -> Result<Vec<PhoenixOrdSuborder>>{
        match PhoenixOrdSuborder::query_all_sub_order(self.order_id, db).await {
            Ok(model) => {
                let _ = self.update_sub_order_cache(&model, redis).await;
                Ok(model)
            }
            Err(err) => {
                log::error!("{:?}", &err);
                return Err(err);
            }
        }
    }

    pub async fn update_order_cache(&self, model: &PhoenixOrdStockorder, redis: &RedisClient) -> Result<()>{
        let key=format!("{}{}", ORDER, self.order_id);
        let data=serde_json::json!(model).to_string();
        // let data = serde_json::to_string(&model).unwrap();
        if let Err(err) = redis.set_str_value(&key,EXPARE_TIME_8_HOUR,&data).await{
            log::error!("更新订单缓存出错: {:?}",err);
            return Err(err);   
        }
        log::info!("更新订单缓存成功...");
        Ok(())
    }

    pub async fn update_sub_order_cache(&self, model: &Vec<PhoenixOrdSuborder>, redis: &RedisClient) -> Result<()>{
        let key=format!("{}{}", SUB_ORDER, self.order_id);
        let data=serde_json::json!(model).to_string();
        // let data = serde_json::to_string(&model).unwrap();
        if let Err(err) = redis.set_str_value(&key,EXPARE_TIME_8_HOUR,&data).await{
            log::error!("更新子订单缓存出错: {:?}",err);
            return Err(err);   
        }
        log::info!("更新子订单缓存成功...");
        Ok(())
    }

    pub async fn query_order(&self, redis: &RedisClient, db: &DbClient) -> Result<PhoenixOrdStockorder>{
        let key=format!("{}{}", ORDER, self.order_id);
        let data=redis.get_value_by_get(&key).await;
        if !data.is_empty(){
            match serde_json::from_str(&data) {
                Ok(model) => {
                    log::info!("取缓存 {} 订单: {:?}", self.order_id, &model);
                    return Ok(model);
                },
                Err(err) => {
                    log::error!("订单{}解析失败{:?}", &self.order_id, &err);
                },
            }
        }
        self.init_order_cache(redis, db).await
    }

    pub async fn query_all_sub_order(&self, redis: &RedisClient, db: &DbClient) -> Result<Vec<PhoenixOrdSuborder>>{
        let key=format!("{}{}", SUB_ORDER, self.order_id);
        let data=redis.get_value_by_get(&key).await;
        if !data.is_empty(){
            match serde_json::from_str(&data) {
                Ok(models) => {
                    log::info!("取缓存 {} 子订单: {:?}", self.order_id, &models);
                    return Ok(models);
                },
                Err(err) => {
                    log::error!("子订单{}解析失败{:?}", &self.order_id, &err);
                },
            }
        }
        self.init_sub_order_cache(redis, db).await
    }

    pub async fn query_order_info(&self, redis: &RedisClient, db: &DbClient) ->Result<(Vec<PhoenixOrdSuborder>, PhoenixOrdStockorder)> {
        let ret = self.query_all_sub_order(redis, db).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }
        let sub_orders = ret.unwrap();

        let ret = self.query_order(redis, db).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }
        let order = ret.unwrap();
        Ok((sub_orders, order))
    }
}
