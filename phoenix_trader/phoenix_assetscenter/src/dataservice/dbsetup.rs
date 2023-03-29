use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, DbErr, ExecResult, Statement};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct DbConnection {
  pub dbconn: DatabaseConnection,
  backend: DatabaseBackend,
}

impl DbConnection {
  pub async fn new(uri: &str) -> Self {
    let mut opt = ConnectOptions::new(uri.to_owned());

    opt
      .max_connections(100)
      .min_connections(5)
      .connect_timeout(Duration::from_secs(8))
      .idle_timeout(Duration::from_secs(8))
      .max_lifetime(Duration::from_secs(8))
      .sqlx_logging(true)
      .sqlx_logging_level(log::LevelFilter::Info); 

    let db = Database::connect(opt).await.expect("can't connect to database");
    DbConnection {
      dbconn: db,
      backend: DatabaseBackend::MySql,
    }
  }

  pub fn get_connection(&self) -> &DatabaseConnection {
      &self.dbconn
  }

  pub async fn execute_sql(&self, sqlstr: &str) -> Result<ExecResult, DbErr> {
    let ret = self.dbconn.execute(Statement::from_string(self.backend, sqlstr.to_owned())).await;
    log::info!("execute sql ret:{:?}, sql:{}", ret, sqlstr);

    ret
  }
}
