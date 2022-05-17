use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
// use tracing;
// use tracing_subscriber;

#[derive(Debug, Clone)]
pub struct SeaSql {
    pub dbconn: DatabaseConnection,
}

impl SeaSql {
    pub async fn new(uri: &str) -> Self {
        let mut opt = ConnectOptions::new(uri.to_owned());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(10))
            // .idle_timeout(Duration::from_secs(8))
            .sqlx_logging(false);

        let dbconn = Database::connect(opt).await.expect("can't connect to database");

        SeaSql { dbconn }
    }
}