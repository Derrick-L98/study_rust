use crate::dataservice::{
  dbsetup::DbConnection,
  entities::{phoenix_sys_system, prelude::*},
};
use anyhow::{anyhow, Result};
use common::constant::SYSTEM_CODE;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;
use utility::timeutil;

impl PhoenixSysSystem {
  pub async fn find(db: &DbConnection) -> Result<Option<PhoenixSysSystem>> {
    let ret = PhoenixSysSystemEntity::find()
      .filter(phoenix_sys_system::Column::SystemCode.eq(SYSTEM_CODE))
      .one(db.get_connection())
      .await;
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }

    let letdata = ret.unwrap();
    if letdata.is_some() {
      return Ok(letdata);
    }

    Err(anyhow!("can't find system info by code:{}", SYSTEM_CODE))
    // let current_date = timeutil::current_date();
    // let default_model = phoenix_sys_system::Model {
    //   init_date: current_date,
    //   id: 0,
    //   system_code: SYSTEM_CODE.to_string(),
    //   system_name: "".to_string(),
    //   before_preinit_date: 0,
    //   init_real_time: 0,
    //   preinit_date: 0,
    // };
    // return Ok(Some(default_model));
  }
}
