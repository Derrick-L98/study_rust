
use rbatis::core::db::DBPoolOptions;
use rbatis::rbatis::Rbatis;

#[derive(Debug)]
pub struct RbSql {
    // rb: MySqlPool,
    rb: Rbatis,
}

impl RbSql {
    pub async fn new(uri: &str) -> Self {
        let mut opt = DBPoolOptions::new();
        opt.max_connections = 20;
        opt.connect_timeout = std::time::Duration::from_secs(10);

        let rbconn = Rbatis::new();
        // rbconn.set_log_plugin(arg)
        rbconn.link_opt(uri, opt).await.unwrap();
        RbSql { rb: rbconn }
    }

    pub fn get_connection(&self) -> &Rbatis {
        &self.rb
    }
}
