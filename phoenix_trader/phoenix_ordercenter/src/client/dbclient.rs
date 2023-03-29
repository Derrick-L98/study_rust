// use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use sea_orm::{DatabaseTransaction ,SqlxMySqlPoolConnection ,SqlxMySqlConnector, ConnectOptions, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, DbErr, ExecResult, Statement};
#[derive(Debug, Clone)]
pub struct DbClient {
    pub dbconn: DatabaseConnection,
    // backend: DatabaseBackend,
}

impl DbClient {
    pub async fn new(uri: &String) -> Self {
        let mut opt = ConnectOptions::new(uri.to_owned());
        opt.max_connections(64)
            .min_connections(16)
            .connect_timeout(Duration::from_secs(10))
            .max_lifetime(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(8))
            .sqlx_logging(false);
            // .sqlx_logging(true)
            // .sqlx_logging_level(log::LevelFilter::Error); 

        // let dbconn = Database::connect(opt).await.expect("can't connect to database");
        let dbconn = SqlxMySqlConnector::connect(opt).await.expect("can't connect to database");

        DbClient { 
            dbconn,
            // backend: DatabaseBackend::MySql,
        }
    }

    pub fn get_conn(&self)-> &DatabaseConnection {
        &self.dbconn
    }
}