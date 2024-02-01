use crate::dataservice::entities::{phoenix_sys_system, prelude::*};
use anyhow::{anyhow, Result};
use sea_orm::{entity::*, query::*, DatabaseConnection};

impl PhoenixSysSystem {
  pub async fn find_by_id(db: &DatabaseConnection) -> Result<Option<PhoenixSysSystem>> {
    let ret = PhoenixSysSystemEntity::find().filter(phoenix_sys_system::Column::SystemCode.eq("oms-stock")).one(db).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!("phoenix_sys_system find_by_id数据查询失败:{:?}", ret));
    }
    //   let mut letdata = ret.unwrap();
    Ok(ret.unwrap())
  }
}
