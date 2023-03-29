use anyhow::{anyhow, Result};
use scylla::prepared_statement::PreparedStatement;
use scylla::transport::session::PoolSize;
use scylla::{batch::Batch, Session, SessionBuilder};
use std::num::NonZeroUsize;
// use serde::Deserialize;
// use std::collections::HashMap;
// use std::fmt::Write;

use crate::config::settings::CassandraConfig;
// use crate::protofiles::YsHqInfo;
use crate::server::service::persistservice::CassandraData;

pub struct CassandraClient {
    pub namespace: String,
    pub session: Session, //该连接用于执行Sql语句
}

impl CassandraClient {
    pub async fn new(config: &CassandraConfig) -> Result<Self> {
        // let username = config.username.clone();
        // let password = config.password.clone();
        // let uri = config.addr.clone();

        let session = SessionBuilder::new()
            .known_node(&config.addr)
            .user(&config.username, &config.password)
            .pool_size(PoolSize::PerHost(NonZeroUsize::new(16).unwrap()))
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

    pub async fn execute_prepare(&self, prepared: &PreparedStatement, val: &CassandraData) -> Result<()> {
        let ret = self.session.execute(&prepared, &val).await;
        // let ret = self.session.query(cql, &[]).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("{}", ret.as_ref().err().unwrap().to_string()));
        }
        Ok(())
    }

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

    pub async fn execute_batch(&self, cql: &str, parms: &Vec<CassandraData>) -> Result<()> {
        let dst: Vec<Vec<CassandraData>> = parms.chunks(6).map(|s| s.into()).collect();

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
}
