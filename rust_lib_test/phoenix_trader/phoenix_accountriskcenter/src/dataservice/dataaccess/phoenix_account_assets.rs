use crate::dataservice::{
  dbsetup::DbConnection,
  entities::{phoenix_account_assets, prelude::*},
};
use anyhow::{anyhow, Ok, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter};
use serde_json::json;

impl PhoenixAccountAssets {
  pub async fn query_many(account_id: i64, db: &DbConnection) -> Result<Vec<PhoenixAccountAssets>> {
    // if deal_date <= 0 {
    //   return Err(anyhow!("error date, 0 is not allowed"));
    // }
    let mut conditions = Condition::all();
    if account_id > 0 {
      conditions = conditions.add(phoenix_account_assets::Column::PAccountId.eq(account_id));
    }
    let ret = PhoenixAccountAssetsEntity::find().filter(conditions).all(db.get_connection()).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let d = ret.unwrap();
    return Ok(d);
  }

  pub async fn save_many(info: &Vec<PhoenixAccountAssets>, db: &DbConnection) -> Result<()> {
    if info.len() <= 0 {
      return Err(anyhow!("empty data"));
    }

    let mut insert_values: Vec<phoenix_account_assets::ActiveModel> = Vec::new();
    let mut update_values: Vec<phoenix_account_assets::ActiveModel> = Vec::new();
    for val in info {
      let ret = phoenix_account_assets::ActiveModel::from_json(json!(val));
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
      let ret = PhoenixAccountAssetsEntity::insert_many(insert_values).exec(db.get_connection()).await;
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
