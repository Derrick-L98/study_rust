use crate::dataservice::{
  dbsetup::DbConnection,
  entities::{phoenix_trans_detail, prelude::*},
};
use anyhow::{anyhow, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter};
// use serde_json::json;

impl PhoenixTransDetail {
  pub async fn insert(data: &PhoenixTransDetail, db: &DbConnection) -> Result<i64> {
    let newval = data.to_owned().into_active_model();

    let ret = newval.insert(db.get_connection()).await;

    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let id = ret.unwrap().id;
    return Ok(id);
  }

  pub async fn query_many(targetaccount: i64, db: &DbConnection) -> Result<Vec<PhoenixTransDetail>> {
    if targetaccount <= 0 {
      return Err(anyhow!("error conditions, 0 is not allowed"));
    }
    let conditions = Condition::all().add(phoenix_trans_detail::Column::PAccountTarget.eq(targetaccount));

    let ret = PhoenixTransDetailEntity::find().filter(conditions).all(db.get_connection()).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let d = ret.unwrap();
    return Ok(d);
  }
}
