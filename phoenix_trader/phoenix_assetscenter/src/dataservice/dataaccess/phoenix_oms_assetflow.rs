use crate::dataservice::{
    dbsetup::DbConnection,
    entities::{phoenix_oms_assetflow, prelude::*},
  };
  use anyhow::{anyhow, Result, Ok};
  use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, Set};
 



impl  PhoenixOmsAssetflow { 
    
    pub async fn insert(data:&PhoenixOmsAssetflow,db:&DbConnection)->Result<i64>{

        let newval=data.to_owned().into_active_model();
        let ret =newval.insert(db.get_connection()).await;
        if ret.as_ref().is_err(){
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }
        let id=ret.unwrap().id;
        return Ok(id);
    }


    pub async fn insert_many(data: &Vec<PhoenixOmsAssetflow>, db: &DbConnection) ->Result<()>{
        if data.len() <= 0 {
            return Err(anyhow!("empty data"));
        }
        let mut active_values: Vec<phoenix_oms_assetflow::ActiveModel> = Vec::new();
        for val in data.into_iter() {
            active_values.push(val.to_owned().into_active_model());
        }
        let ret = PhoenixOmsAssetflowEntity::insert_many(active_values).exec(db.get_connection()).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }
      
        return  Ok(());
    }
}

