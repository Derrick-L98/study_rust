
use anyhow::Result;
use tokio::sync::RwLock;
use std::{collections::HashMap, sync::Arc};

use super::service::prelude::*;
use crate::commonutil::commonutil::CommonUtil;
use super::service::{errors, persistservice::PersistService};

use utility::timeutil;
use crate::protofiles::{LastPriceReq, LastPriceResp, YsHqInfo, TickReq, TickResp};

use crate::client::{
    sledclient::SledClient,
    tickmqclient::TickMqClient,
    cassandraclient::CassandraClient,
};

#[derive(Clone)]
pub struct TickCenterController {
    pub tick_ctl: Arc<TickService>,
    pub common_util: Arc<CommonUtil>,
    pub cassandra_client: Option<Arc<CassandraClient>>,
    pub tick_mq: Option<Arc<TickMqClient>>,
    pub sledclient: Arc<SledClient>,
    pub ticks_cache: Arc<RwLock<Vec<YsHqInfo>>>,
}

impl TickCenterController {
    pub async fn send_tick_to_mq(&self, tick: &YsHqInfo) -> Result<()> {
        if self.tick_mq.is_none() {
            return Ok(());
        }
        let market_type = PersistService::get_markettype(&tick.exchange_id).await;
        if let Err(err) = self.tick_mq.as_ref().unwrap().publish(market_type, tick).await {
            log::error!("push tick data to mq err: {:?}", err);
            return Err(anyhow!("push tick data to mq err: {:?}", err));
        }
        Ok(())
    }

    pub async fn persist_ticks(&self) -> Result<()> {
        let mut ticks = self.ticks_cache.write().await;
        if ticks.len() > 0 {
            let ticks_clone = ticks.clone();
            ticks.clear();
            drop(ticks);
            // log::info!("缓存的tick数据:{}", ticks_clone.len());
            let ret = self.insert_local_db(&ticks_clone).await;
            if ret.as_ref().is_err() {
                log::error!("{}", ret.as_ref().err().unwrap().to_string());
                return Err(anyhow!("{}", ret.as_ref().err().unwrap().to_string()));
            }

            if self.cassandra_client.is_some() {
                let now = std::time::Instant::now();
                let ret = PersistService::insert_ticks_into_cassandra(&ticks_clone, &self.cassandra_client.as_ref().unwrap()).await;
                if ret.as_ref().is_err() {
                    log::error!("{}", ret.as_ref().err().unwrap().to_string());
                    return Err(anyhow!("{}", ret.as_ref().err().unwrap().to_string()));
                }
                log::info!("Tick数据写入Cassandra完成,数据量:{},用时:{:?}", ticks_clone.len(), now.elapsed());
            }
        }

        Ok(())
    }

    pub async fn insert_local_db(&self, ticks_clone: &Vec<YsHqInfo>) -> Result<()> {
        let mut tick_map: HashMap<String, YsHqInfo> = HashMap::new();
        for val in ticks_clone.iter() {
            let info = tick_map.get(&val.contract_no1);
            if info.is_none() {
                tick_map.insert(val.contract_no1.clone(), val.clone());
                continue;
            }
            let exist_tick = info.unwrap();
            let current_time = timeutil::convert_datetime_to_timestamp(&val.tapidtstamp);
            let exist_time = timeutil::convert_datetime_to_timestamp(&exist_tick.tapidtstamp);
            if current_time > exist_time {
                tick_map.insert(val.contract_no1.clone(), val.clone());
            }
        }

        let now = std::time::Instant::now();
        let ret = self.sledclient.batch_insert(&tick_map).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("{}", ret.as_ref().err().unwrap().to_string()));
        }
        log::info!("Tick数据写入本地缓存完成,数据量:{},用时:{:?}", tick_map.len(), now.elapsed());
        Ok(())
    }

    pub async fn insert_ticks(&self, tick: &mut YsHqInfo) -> Result<()> {
        self.tick_ctl.insert_ticks(tick, &self.common_util).await
    }

    pub async fn insert_cache(&self, tick: &YsHqInfo) -> Result<()> {
        let mut ticks_write = self.ticks_cache.write().await;
        ticks_write.push(tick.to_owned());
        Ok(())
    }

    //最新价
    pub async fn get_stock_last_price(&self, req: &LastPriceReq) -> Result<LastPriceResp> {
        log::info!("get_stock_last_price request:{:?} ", req.contract_nos);

        let infos = self.tick_ctl.query_ticks(&req.contract_nos).await;
        // res.data = infos;

        let res = LastPriceResp {
            err_msg: errors::get_error_code(errors::ErrorCode::CodeOk).0,
            err_code: errors::get_error_code(errors::ErrorCode::CodeOk).1,
            data: infos,
        };
        log::info!("get_stock_last_price response:{:?}\n", res);
        // if let Some(value) = self.tick_ctl.query_ticks(&req.contract_nos).await {
        //     log::info!("获取本地最新价: {:?}", value);
        //     last_price_info.last_price = value.latest_price; //最新价
        //     last_price_info.change_rate = value.change_rate; //涨跌幅
        //     last_price_info.change_value = value.change_value; //涨跌值
        // } else {
        //     log::error!("本地未找到证券 {} 最新价", &req.contract_no1);
        //     res.err_msg = errors::get_error_code(errors::ErrorCode::CodeNoData).0;
        //     res.err_code = errors::get_error_code(errors::ErrorCode::CodeNoData).1;
        //     return Ok(res);
        // }
        // res.data = Some(last_price_info);
        Ok(res)
    }

    pub async fn get_tick_hq(&self, req: &TickReq) ->Result<TickResp> {
        log::info!("最新tick请求: {}", &req.contract_no);
        let mut resp = TickResp::default();
        resp.tick_hq_info = self.tick_ctl.get_ticks(&req.contract_no).await;
        Ok(resp)
    }
}
