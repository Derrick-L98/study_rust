use crate::dataservice::{
  dbsetup::DbConnection,
  entities::{phoenix_risk_details, prelude::*},
};
use anyhow::{anyhow, Ok, Result};
use sea_orm::{ActiveModelTrait, Condition, EntityTrait, IntoActiveModel};
use sea_orm::{ActiveValue::NotSet, ColumnTrait, QueryFilter, Set};

impl PhoenixRiskDetails {
  pub async fn insert(data: &PhoenixRiskDetails, db: &DbConnection) -> Result<i64> {
    let newval = data.to_owned().into_active_model();

    let ret = newval.insert(db.get_connection()).await;

    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let id = ret.unwrap().id;
    return Ok(id);
  }
}
