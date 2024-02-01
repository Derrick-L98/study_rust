use snowflake::SnowflakeIdBucket;

#[derive(Debug, Clone)]
pub struct UidgenService {
  pub idgen: SnowflakeIdBucket,
}

impl UidgenService {
  pub fn new(machineid: i32, nodeid: i32) -> Self {
    UidgenService {
      idgen: SnowflakeIdBucket::new(machineid, nodeid),
    }
  }

  pub fn get_uid(&mut self) -> i64 {
    let id = self.idgen.get_id();
    id
  }
}
