use std::collections::HashMap;
// use sled::Db;
// use std::fmt::Write;
use chrono::prelude::*;
use anyhow::{anyhow, Result};
use super::common::KLineData;
use crate::client::{
    cassandraclient::CassandraClient,
};
use utility::timeutil::build_naive_date_time;
// use crate::client::sledclient::SledClient;


pub struct PersistService {}

/// 行情数据
#[derive(scylla::ValueList, Clone)]
pub struct CassandraData {
    pub key: String,
    pub uptime: String,
    pub val: String,
}

/// K线数据
#[derive(scylla::ValueList, Clone, Debug)]
pub struct CassandraKLineData {
    pub peroid: i32,
    pub vc_code: String,
    pub l_update_time: String,
    pub en_close_price: f64,
    pub en_high_price: f64,
    pub en_last_price: f64,
    pub en_low_price: f64,
    pub en_open_price: f64,
    pub l_volume: f64,
}

// /// K线数据
// #[derive(scylla::ValueList, Clone)]
// pub struct KLineData {
//     pub table_name: String,
//     pub vc_code: String,
//     pub l_update_time: String,
//     pub en_close_price: f64,
//     pub en_high_price: f64,
//     pub en_last_price: f64,
//     pub en_low_price: f64,
//     pub en_open_price: f64,
//     pub l_volume: f64,
// }

/// 分时数据
#[derive(scylla::ValueList, Clone, Debug)]
pub struct FenShiData {
    pub vc_code: String,
    pub l_update_time: i64,
    pub vc_content: String,
}

impl PersistService {
    pub async fn get_table_name(period: i32) -> String {
        if period == 1 {
            "tstockhq1kline".to_string()
        } else if period == 5 {
            "tstockhq5kline".to_string()
        } else if period == 10 {
            "tstockhq10kline".to_string()
        } else if period == 30 {
            "tstockhq30kline".to_string()
        } else if period == 60 {
            "tstockhq60kline".to_string()
        } else {
            //1440
            "tstockhqdaykline".to_string()
        }
    }

    pub async fn insert_klines_into_cassandra(klines: &Vec<KLineData>, cass_client: &CassandraClient) -> Result<()> {
        let mut tick_time = String::new();
        let mut minutes = 0;
        let contents: Vec<CassandraKLineData> = klines
            .iter()
            .map(|x| CassandraKLineData {
                peroid: x.period,
                vc_code: x.stock_code.clone(),
                l_update_time: {
                    tick_time = x.tick_time.clone();
                    let st = build_naive_date_time(&tick_time);

                    minutes = if x.period == 1440 {
                        x.end_minutes
                    } else {
                        (x.prev_minutes + x.period) % 1440
                    };
            
                    //年月日时分
                    format!("{:04}{:02}{:02}{:02}{:02}00", &st.year(), &st.month(), &st.day(), minutes / 60, minutes % 60)
                },
                en_close_price: x.close_price,
                en_high_price: x.high_price,
                en_last_price: x.last_price,
                en_low_price: x.low_price,
                en_open_price: x.open_price,
                l_volume: x.current_period_volume,
            })
            .collect();
            // log::info!("{:#?}", contents);

        cass_client.execute_batch_k(&contents).await
    }

    pub async fn insert_fenshi_into_cassandra(fenshi: &HashMap<String, String>, cass_client: &CassandraClient) -> Result<()> {
        let contents: Vec<FenShiData> = fenshi
            .iter()
            .map(|(k, v)| FenShiData {
                vc_code: {
                    let str: Vec<&str> = k.split("|").collect();//[STOCK_FS:600000_XSHG, 20220823]
                    str[0].to_owned()
                },
                l_update_time: {
                    let str: Vec<&str> = k.split("|").collect();//[STOCK_FS:600000_XSHG, 20220823]
                    str[1].parse::<i64>().unwrap_or_default()
                },
                vc_content: v.to_owned(),
            })
            .collect();
        let cql = "insert into tstockhqtimeshare (vc_code, l_update_time, vc_content) values (?, ?, ?)";
        
        // cass_client.execute_batch_fenshi(cql, &contents).await

        let ret = cass_client.do_prepare(cql).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("prepare error:{}", ret.as_ref().err().unwrap().to_string()));
        }
        let prepared = ret.unwrap();

        for val in contents {
            let ret = cass_client.execute_prepare(&prepared, &val).await;
            if ret.as_ref().is_err() {
                return Err(anyhow!("insert cassandra error:{}", ret.as_ref().err().unwrap().to_string()));
            }
        }
        Ok(())
    }

}
