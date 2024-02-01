use crate::dataservice::{
  dbsetup::DbConnection,
  entities::{phoenix_ast_stockposition, prelude::*},
};
use anyhow::{anyhow, Result};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde_json::json;

impl PhoenixAstStockposition {

  pub async fn query_many(unit_id: i64, stock_id: i64, db: &DbConnection) -> Result<Vec<PhoenixAstStockposition>> {
    let mut conditions = Condition::all();
    if unit_id > 0 {
      conditions = conditions.add(phoenix_ast_stockposition::Column::UnitId.eq(unit_id));
    }
    if stock_id > 0 {
      conditions = conditions.add(phoenix_ast_stockposition::Column::StockId.eq(stock_id));
    }
    let ret = PhoenixAstStockpositionEntity::find().filter(conditions).all(db.get_connection()).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }

    Ok(ret.unwrap())
  }

  pub async fn insert(data: &PhoenixAstStockposition, db: &DbConnection) -> Result<i64> {
    
    let newval = data.to_owned().into_active_model();
    let ret = newval.insert(db.get_connection()).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    Ok(ret.unwrap().id)
  }

  pub async fn save(data: &PhoenixAstStockposition, db: &DbConnection) -> Result<()> {
    let ret = phoenix_ast_stockposition::ActiveModel::from_json(json!(data));
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let mut newval = ret.unwrap();
    if data.id > 0 {
      newval.id = Set(data.id);
    } else {
      newval.id = NotSet;
    }
    let ret = newval.save(db.get_connection()).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    Ok(())
  }

  pub async fn save_many(infos: &Vec<PhoenixAstStockposition>, db: &DbConnection) -> Result<()> {
    if infos.len() <= 0 {
      return Err(anyhow!("empty data"));
    }
    let mut insert_values: Vec<phoenix_ast_stockposition::ActiveModel> = Vec::new();
    let mut update_values: Vec<phoenix_ast_stockposition::ActiveModel> = Vec::new();
    for val in infos.iter() {
      let ret = phoenix_ast_stockposition::ActiveModel::from_json(json!(val));
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
      let ret = PhoenixAstStockpositionEntity::insert_many(insert_values).exec(db.get_connection()).await;
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

  pub async fn insert_many(info: &Vec<PhoenixAstStockposition>, db: &DbConnection) -> Result<i64> {
    if info.len() <= 0 {
      return Err(anyhow!("empty data"));
    }
    let mut active_values: Vec<phoenix_ast_stockposition::ActiveModel> = Vec::new();
    for val in info.into_iter() {
      active_values.push(val.to_owned().into_active_model());
    }
    let ret = PhoenixAstStockpositionEntity::insert_many(active_values).exec(db.get_connection()).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }

    Ok(ret.unwrap().last_insert_id)
  }
}

