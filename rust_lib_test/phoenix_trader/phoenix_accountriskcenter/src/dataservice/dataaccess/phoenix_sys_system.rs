use crate::dataservice::{
  dbsetup::DbConnection,
  entities::{phoenix_sys_system, prelude::*},
};
use anyhow::{anyhow, Result};
use common::constant;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;

impl PhoenixSysSystem {
  pub async fn query(db: &DbConnection) -> Result<PhoenixSysSystem> {
    let ret = PhoenixSysSystemEntity::find()
      .filter(phoenix_sys_system::Column::SystemCode.eq(constant::SYSTEM_CODE))
      .one(db.get_connection())
      .await;
    if let Err(err) = ret {
      log::error!("查询失败query,{:?}", err);
      return Err(anyhow!("查询失败query,{:?}", err));
    }
    let d = ret.unwrap();
    if d.is_none() {
      return Err(anyhow!("can't find sys info"));
    }
    return Ok(d.unwrap());
  }

  pub async fn update(model: &PhoenixSysSystem, db: &DbConnection) -> Result<PhoenixSysSystem> {
    let av_model = phoenix_sys_system::ActiveModel::from_json(json!(model)).expect("convert to active model error");

    // av_model.system_code = NotSet;
    // av_model.id = NotSet;
    log::info!("active model is:{:?}", &av_model);
    // av_model.id = NotSet;
    let ret = PhoenixSysSystemEntity::update(av_model)
      .filter(phoenix_sys_system::Column::SystemCode.eq(constant::SYSTEM_CODE))
      .exec(db.get_connection())
      .await;
    if let Err(err) = ret {
      log::error!("update失败 ,{:?}", err);
      return Err(anyhow!("update失败 ,{:?}", err));
    }
    let d = ret.unwrap();
    Ok(d)
  }
}
