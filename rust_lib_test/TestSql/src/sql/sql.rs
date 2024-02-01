
use mysql_async::prelude::*;
use mysql_async::Pool;

#[derive(Debug)]
pub struct Sql {
    // rb: MySqlPool,
    rb: Pool,
}

impl Sql {
    pub async fn new(uri: &str) -> Self {
        let pool = mysql_async::Pool::new(uri);
        
        Sql { rb: pool }
    }

    pub fn get_connection(&self) -> &Pool {
        &self.rb
    }
}
