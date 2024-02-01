use crate::dataservice::{
  dbsetup::DbConnection,
  entities::{phoenix_stockposition_channel, prelude::*},
};
use anyhow::{anyhow, Ok, Result};
use sea_orm::{ActiveModelTrait, Condition, EntityTrait, IntoActiveModel};
use sea_orm::{ActiveValue::NotSet, ColumnTrait, QueryFilter};
use serde_json::json;

impl PhoenixStockPositionChannel {
  pub async fn query_many(unit_id: i64, stock_id: i64, channel_id: i64, db: &DbConnection) -> Result<Vec<PhoenixStockPositionChannel>> {
    let mut conditions = Condition::all();
    if unit_id > 0 {
      conditions = conditions.add(phoenix_stockposition_channel::Column::PAccountUnitId.eq(unit_id));
    }
    if stock_id > 0 {
      conditions = conditions.add(phoenix_stockposition_channel::Column::PStockId.eq(stock_id));
    }
    if channel_id > 0 {
      conditions = conditions.add(phoenix_stockposition_channel::Column::PChannelId.eq(channel_id));
    }
    let ret = PhoenixStockpositionChannelEntity::find().filter(conditions).all(db.get_connection()).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let d = ret.unwrap();
    return Ok(d);
  }

  pub async fn insert_many(info: &Vec<PhoenixStockPositionChannel>, db: &DbConnection) -> Result<i64> {
    if info.len() <= 0 {
      return Err(anyhow!("empty data"));
    }

    let mut active_values: Vec<phoenix_stockposition_channel::ActiveModel> = Vec::new();
    for val in info {
      let ret = phoenix_stockposition_channel::ActiveModel::from_json(json!(val));
      if ret.as_ref().is_err() {
        return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
      }
      active_values.push(ret.unwrap());
    }
    let ret = PhoenixStockpositionChannelEntity::insert_many(active_values).exec(db.get_connection()).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }

    Ok(ret.unwrap().last_insert_id)
  }

  pub async fn save(info: &PhoenixStockPositionChannel, db: &DbConnection) -> Result<i64> {
    let ret = phoenix_stockposition_channel::ActiveModel::from_json(json!(info));

    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let mut newval = ret.unwrap();
    if info.id <= 0 {
      newval.id = NotSet;
    }
    let ret = newval.save(db.get_connection()).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let val_id: Option<i64> = ret.unwrap().id.into_value().unwrap().unwrap();
    if val_id.is_none() {
      return Err(anyhow!("insert result error"));
    }
    // let
    Ok(val_id.unwrap())
  }

  pub async fn save_many(info: &Vec<PhoenixStockPositionChannel>, db: &DbConnection) -> Result<()> {
    if info.len() <= 0 {
      return Err(anyhow!("empty data"));
    }

    let mut insert_values: Vec<phoenix_stockposition_channel::ActiveModel> = Vec::new();
    let mut update_values: Vec<phoenix_stockposition_channel::ActiveModel> = Vec::new();
    for val in info {
      let ret = phoenix_stockposition_channel::ActiveModel::from_json(json!(val));
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
      let ret = PhoenixStockpositionChannelEntity::insert_many(insert_values).exec(db.get_connection()).await;
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
