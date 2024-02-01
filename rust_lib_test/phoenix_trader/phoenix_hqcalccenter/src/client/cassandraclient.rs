use anyhow::{anyhow, Result};
use scylla::prepared_statement::PreparedStatement;
use scylla::transport::session::PoolSize;
use scylla::{batch::Batch, Session, SessionBuilder};
// use serde::Deserialize;
// use std::collections::HashMap;
// use std::fmt::Write;
use std::num::NonZeroUsize;

use crate::config::settings::CassandraConfig;
use crate::server::service::persistservice::{CassandraData, CassandraKLineData, PersistService, FenShiData};

pub struct CassandraClient {
    pub namespace: String,
    pub session: Session, //该连接用于执行Sql语句
}

impl CassandraClient {
    pub async fn new(config: &CassandraConfig) -> Result<Self> {
        let session = SessionBuilder::new()
            .known_node(&config.addr)
            .user(&config.username, &config.password)
            .pool_size(PoolSize::PerHost(NonZeroUsize::new(16).unwrap()))
            .keepalive_interval(std::time::Duration::from_secs(2))
            .build()
            .await
            .expect("连接服务器失败");

        session.use_keyspace(&config.namespace.clone(), false).await.expect("user namespace error......");

        log::info!("Cassandra 连接成功...");

        let client = CassandraClient {
            namespace: config.namespace.clone(),
            session,
        };

        Ok(client)
    }

    pub async fn do_prepare(&self, cql: &str) -> Result<PreparedStatement> {
        let ret = self.session.prepare(cql).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("prepare cql error"));
        }
        Ok(ret.unwrap())
    }

    pub async fn execute_prepare(&self, prepared: &PreparedStatement, val: &FenShiData) -> Result<()> {
        let ret = self.session.execute(&prepared, &val).await;
        // let ret = self.session.query(cql, &[]).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("{}", ret.as_ref().err().unwrap().to_string()));
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn execute_cql(&self, cql: &str, val: &CassandraData) -> Result<()> {
        // self.session.prepare(&cql).await;
        let ret = self.session.prepare(cql).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("prepare cql error"));
        }
        let prepared = ret.unwrap();
        let ret = self.session.execute(&prepared, &val).await;
        // let ret = self.session.query(cql, &[]).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("{}", ret.as_ref().err().unwrap().to_string()));
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn execute_batch(&self, cql: &str, parms: &Vec<CassandraData>) -> Result<()> {
        let dst: Vec<Vec<CassandraData>> = parms.chunks(12).map(|s| s.into()).collect();

        for val in dst {
            let mut batch: Batch = Default::default();
            let len = val.len();
            for _n in 0..len {
                batch.append_statement(cql);
            }
            let ret = self.session.batch(&batch, &val).await;
            if ret.as_ref().is_err() {
                return Err(anyhow!("{}", ret.as_ref().err().unwrap().to_string()));
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn execute_batch_fenshi(&self, cql: &str, fenshi: &Vec<FenShiData>) -> Result<()> {
        let dst: Vec<Vec<FenShiData>> = fenshi.chunks(6).map(|s| s.into()).collect();
        let ret = self.session
            .prepare(cql)
            .await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("prepare cql error: {}", ret.as_ref().err().unwrap().to_string()));
        }
        let prepared = ret.unwrap();
        for val in dst {
            let mut batch: Batch = Default::default();
            let len = val.len();
            for _i in 0..len {         
                batch.append_statement(prepared.clone());
            }
            let ret = self.session.batch(&batch, &val).await;
            if ret.as_ref().is_err() {
                return Err(anyhow!("{}", ret.as_ref().err().unwrap().to_string()));
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn insert_klinedate_to_cass(&self, cql: &String) -> Result<()> {
        let ret = self.session.prepare(cql.as_str()).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("prepare cql error"));
        }
        let prepared = ret.unwrap();
        let ret = self.session.execute(&prepared, &[]).await; //.query(cql.to_owned(), &[]).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret);
            return Err(anyhow!("{:?}", ret));
        }
        Ok(())
    }

    pub async fn execute_batch_k(&self, parms: &Vec<CassandraKLineData>) -> Result<()> {
        let mut values:Vec<()> = Vec::new();
        let dst: Vec<Vec<CassandraKLineData>> = parms.chunks(12).map(|s| s.into()).collect();
        for val in dst {
            let mut batch: Batch = Default::default();
            for data in val.iter() {
                let table_name = PersistService::get_table_name(data.peroid).await;
                let cql = format!(
                    "insert into {} (en_close_price, en_high_price, en_last_price, en_low_price, en_open_price, l_volume, vc_code, l_update_time, l_fixed_time) values ({:.03}, {:.03}, {:.03}, {:.03}, {:.03}, {}, '{}', {}, toTimestamp(now()))",
                    table_name,
                    data.en_close_price,
                    data.en_high_price,
                    data.en_last_price,
                    data.en_low_price,
                    data.en_open_price,
                    data.l_volume,
                    data.vc_code,
                    data.l_update_time,
                );
                // log::info!("{}", &cql);
                batch.append_statement(cql.as_str());
                values.push(());
            }
            let ret = self.session.batch(&batch, values.clone()).await;
            if ret.as_ref().is_err() {
                return Err(anyhow!("{}", ret.as_ref().err().unwrap().to_string()));
            }
            values.clear();
        }
        Ok(())
    }

}
