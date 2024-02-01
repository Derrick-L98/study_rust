use crate::dataservice::{
  dbsetup::DbConnection,
  entities::{phoenix_account_reset_detail, prelude::*},
};
use anyhow::{anyhow, Ok, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter};
use serde_json::json;

impl PhoenixAccountResetDetail {
  pub async fn insert(data: &PhoenixAccountResetDetail, db: &DbConnection) -> Result<i64> {
    let newval = data.to_owned().into_active_model();

    let ret = newval.insert(db.get_connection()).await;

    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let id = ret.unwrap().id;
    return Ok(id);
  }

  pub async fn query_many(accountid: i64, stdate: i64, enddate: i64, db: &DbConnection) -> Result<Vec<PhoenixAccountAssets>> {
    if accountid <= 0 || stdate <= 0 || enddate <= 0 {
      return Err(anyhow!("error conditions, 0 is not allowed"));
    }
    let conditions = Condition::all()
      .add(phoenix_account_reset_detail::Column::PAccountUnitId.eq(accountid))
      .add(phoenix_account_reset_detail::Column::PDatetime.gte(stdate))
      .add(phoenix_account_reset_detail::Column::PDatetime.lte(enddate));

    let ret = PhoenixAccountAssetsEntity::find().filter(conditions).all(db.get_connection()).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let d = ret.unwrap();
    return Ok(d);
  }
}
