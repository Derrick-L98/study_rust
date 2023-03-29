use crate::dataservice::{
  dbsetup::DbConnection,
  entities::{phoenix_account_assets_his, prelude::*},
};
use anyhow::{anyhow, Ok, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter};
use serde_json::json;

impl PhoenixAccountAssetsHis {
  pub async fn query_many(unit_id: i64, account_type: i32, db: &DbConnection) -> Result<Vec<PhoenixAccountAssetsHis>> {
    let mut conditions = Condition::all();
    if unit_id > 0 {
      conditions = conditions.add(phoenix_account_assets_his::Column::PAccountId.eq(unit_id));
    }
    if account_type > 0 {
      conditions = conditions.add(phoenix_account_assets_his::Column::PAccountType.eq(account_type));
    }
    let ret = PhoenixAccountAssetsHisEntity::find().filter(conditions).all(db.get_connection()).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let d = ret.unwrap();
    return Ok(d);
  }

  pub async fn query_many_in_duration(unit_id: i64, stdate: i64, enddate: i64, db: &DbConnection) -> Result<Vec<PhoenixAccountAssetsHis>> {
    if unit_id <= 0 || stdate <= 0 || enddate <= 0 {
      return Err(anyhow!("error conditions"));
    }
    let mut conditions = Condition::all();
    if unit_id > 0 {
      conditions = conditions.add(phoenix_account_assets_his::Column::PAccountId.eq(unit_id));
    }
    if stdate > 0 {
      conditions = conditions.add(phoenix_account_assets_his::Column::PDate.gte(stdate));
    }
    if enddate > 0 {
      conditions = conditions.add(phoenix_account_assets_his::Column::PDate.lte(enddate));
    }
    let ret = PhoenixAccountAssetsHisEntity::find().filter(conditions).all(db.get_connection()).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let d = ret.unwrap();
    return Ok(d);
  }

  pub async fn save_many(info: &Vec<PhoenixAccountAssetsHis>, db: &DbConnection) -> Result<()> {
    if info.len() <= 0 {
      return Err(anyhow!("empty data"));
    }

    let mut insert_values: Vec<phoenix_account_assets_his::ActiveModel> = Vec::new();
    let mut update_values: Vec<phoenix_account_assets_his::ActiveModel> = Vec::new();
    for val in info {
      let ret = phoenix_account_assets_his::ActiveModel::from_json(json!(val));
      if ret.as_ref().is_err() {
        return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
      }
      if val.id <= 0 {
        insert_values.push(ret.unwrap());
      } else {
        update_values.push(ret.unwrap());
      }
    }

    // do insert
    if insert_values.len() > 0 {
      let ret = PhoenixAccountAssetsHisEntity::insert_many(insert_values).exec(db.get_connection()).await;
      if ret.as_ref().is_err() {
        return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
      }
    }

    //do update
    if update_values.len() > 0 {
      for val in update_values {
        let ret = val.update(db.get_connection()).await;
        if ret.as_ref().is_err() {
          return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }
      }
    }

    Ok(())
  }
}
