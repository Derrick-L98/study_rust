use crate::dataservice::{
  dbsetup::DbConnection,
  entities::{phoenix_ast_operation_detail, prelude::*},
};
use anyhow::{anyhow, Result};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};



impl PhoenixAstOperationDetail {
  pub async fn insert(data: &PhoenixAstOperationDetail, db: &DbConnection) -> Result<()> {
  
    let newval = data.to_owned().into_active_model();

    let ret = newval.insert(db.get_connection()).await;
    if ret.as_ref().is_err() {
        return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }

    Ok(())
  }


  pub async fn insert_many(data: &Vec<PhoenixAstOperationDetail>, db: &DbConnection) ->Result<()>{
    if data.len() <= 0 {
        return Err(anyhow!("empty data"));
    }
    let mut active_values: Vec<phoenix_ast_operation_detail::ActiveModel> = Vec::new();
    for val in data.into_iter() {
        active_values.push(val.to_owned().into_active_model());
    }
    let ret = PhoenixAstOperationDetailEntity::insert_many(active_values).exec(db.get_connection()).await;
    if ret.as_ref().is_err() {
        return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
  
    return  Ok(());
}
}
