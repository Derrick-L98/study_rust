use crate::dataservice::{
  dbsetup::DbConnection,
  entities::{phoenix_stockposition_channel_his, prelude::*},
};
use anyhow::{anyhow, Ok, Result};
use sea_orm::{ActiveModelTrait, Condition, EntityTrait, IntoActiveModel};
use sea_orm::{ActiveValue::NotSet, ColumnTrait, QueryFilter};
use serde_json::json;

impl PhoenixStockPositionChannelHis {
  pub async fn insert(data: &PhoenixStockPositionChannelHis, db: &DbConnection) -> Result<i64> {
    let newval = data.to_owned().into_active_model();

    let ret = newval.insert(db.get_connection()).await;

    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let id = ret.unwrap().id;
    return Ok(id);
  }

  pub async fn query_many(unit_id: i64, pdate: i64, db: &DbConnection) -> Result<Vec<PhoenixStockPositionChannelHis>> {
    if unit_id <= 0 && pdate <= 0 {
      return Err(anyhow!("conditions are all empty, now allowed"));
    }
    let mut conditions = Condition::all();
    if unit_id > 0 {
      conditions = conditions.add(phoenix_stockposition_channel_his::Column::PAccountUnitId.eq(unit_id));
    }
    if pdate > 0 {
      conditions = conditions.add(phoenix_stockposition_channel_his::Column::PDate.eq(pdate));
    }
    let ret = PhoenixStockpositionChannelHisEntity::find().filter(conditions).all(db.get_connection()).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let d = ret.unwrap();
    return Ok(d);
  }
  pub async fn query_many_in_duration(unit_id: i64, pstdate: i64, penddate: i64, db: &DbConnection) -> Result<Vec<PhoenixStockPositionChannelHis>> {
    if unit_id <= 0 && pstdate <= 0 {
      return Err(anyhow!("conditions are all empty, now allowed"));
    }
    let mut conditions = Condition::all();
    if unit_id > 0 {
      conditions = conditions.add(phoenix_stockposition_channel_his::Column::PAccountUnitId.eq(unit_id));
    }
    if pstdate > 0 {
      conditions = conditions.add(phoenix_stockposition_channel_his::Column::PDate.gte(pstdate));
    }
    if penddate > 0 {
      conditions = conditions.add(phoenix_stockposition_channel_his::Column::PDate.lte(penddate));
    }
    let ret = PhoenixStockpositionChannelHisEntity::find().filter(conditions).all(db.get_connection()).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let d = ret.unwrap();
    return Ok(d);
  }
  pub async fn insert_many(info: &Vec<PhoenixStockPositionChannelHis>, db: &DbConnection) -> Result<i64> {
    if info.len() <= 0 {
      return Err(anyhow!("empty data"));
    }

    let mut active_values: Vec<phoenix_stockposition_channel_his::ActiveModel> = Vec::new();
    for val in info {
      let ret = phoenix_stockposition_channel_his::ActiveModel::from_json(json!(val));
      if ret.as_ref().is_err() {
        return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
      }
      active_values.push(ret.unwrap());
    }
    let ret = PhoenixStockpositionChannelHisEntity::insert_many(active_values).exec(db.get_connection()).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }

    Ok(ret.unwrap().last_insert_id)
  }

  pub async fn save(info: &PhoenixStockPositionChannelHis, db: &DbConnection) -> Result<i64> {
    let ret = phoenix_stockposition_channel_his::ActiveModel::from_json(json!(info));

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

  pub async fn save_many(info: &Vec<PhoenixStockPositionChannelHis>, db: &DbConnection) -> Result<()> {
    if info.len() <= 0 {
      return Err(anyhow!("empty data"));
    }

    let mut insert_values: Vec<phoenix_stockposition_channel_his::ActiveModel> = Vec::new();
    let mut update_values: Vec<phoenix_stockposition_channel_his::ActiveModel> = Vec::new();
    for val in info {
      let ret = phoenix_stockposition_channel_his::ActiveModel::from_json(json!(val));
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
      let ret = PhoenixStockpositionChannelHisEntity::insert_many(insert_values).exec(db.get_connection()).await;
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
