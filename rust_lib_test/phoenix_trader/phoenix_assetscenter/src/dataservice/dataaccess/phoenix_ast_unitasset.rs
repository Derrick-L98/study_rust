use crate::dataservice::{
    dbsetup::DbConnection,
    entities::{phoenix_ast_unitasset, prelude::*},
  };
  use anyhow::{anyhow, Result, Ok};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, Set};
  use serde_json::json;

impl PhoenixAstUnitasset{


    pub async fn find_by_unitid(unit_id:i64,db:&DbConnection)-> Result<Option<PhoenixAstUnitasset>>{
        let ret=PhoenixAstUnitassetEntity::find().filter(phoenix_ast_unitasset::Column::UnitId.eq(unit_id)).one(db.get_connection()).await;
        if ret.as_ref().is_err(){
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }
        return Ok(ret.unwrap());

    }
    

    pub async fn find_all(db:&DbConnection)->Result<Vec<PhoenixAstUnitasset>>{
        let ret=PhoenixAstUnitassetEntity::find().all(db.get_connection()).await;
        if ret.as_ref().is_err(){
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        } 
        return Ok(ret.unwrap());
    }

    pub async fn insert(data:&PhoenixAstUnitasset,db:&DbConnection)->Result<i64>{

        let newval=data.to_owned().into_active_model();

        let ret=newval.insert(db.get_connection()).await;
        
        //let ret=PhoenixAstUnitasset::insert(newval, db.get_connection()).await;
        if ret.as_ref().is_err(){
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }
        let id=ret.unwrap().id;
        return Ok(id);
    }


    pub async fn update(data:&PhoenixAstUnitasset,db:&DbConnection)->Result<()>{

        let ret = phoenix_ast_unitasset::ActiveModel::from_json(json!(data));
        if ret.is_err(){
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }
        let mut newval=ret.unwrap();
        println!("{:?}",newval);
        if newval.id.is_not_set(){
            return Err(anyhow!("id不能为空"));
        }
        let ret=newval.update(db.get_connection()).await;
        if ret.as_ref().is_err(){
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }
      
        return Ok(());
    }

}


// //慎用update
// // pub async fn update(data:phoenix_ast_unitasset::ActiveModel,db:&DbConn)->Result<(),anyhow::Error>{
// //     let ret=PhoenixAstUnitasset::update(data).exec(db).await;
// //     if ret.as_ref().is_err(){
// //         log::info!("access_phoenix_ast_unitasset数据操作失败:{:?}",ret);
// //         return Err(anyhow!(constdata::SQL_INSERT_OR_UPDATE_ERROR));
// //     }

// //     return Ok(());
// // }


// pub async fn update_by_expr(db:&DbConn, data:phoenix_ast_unitasset::ActiveModel)->Result<(),anyhow::Error>{

//     let mut update=PhoenixAstUnitasset::update_many();
//     let current_cash=data.current_cash;
//     if current_cash.is_set(){
//         let d=current_cash.unwrap();
//         if d> Decimal::new(0, 0){
//             let param=Some(Box::new(d));
//             update=update.col_expr(phoenix_ast_unitasset::Column::CurrentCash, Expr::value(Value::Decimal(param)));
//         }
//     } 

//     let begin_cash=data.begin_cash;
//     if begin_cash.is_set(){
//        let d=begin_cash.unwrap();
//        if d> Decimal::new(0, 0){
//             let param=Some(Box::new(d));
//             update=update.col_expr(phoenix_ast_unitasset::Column::BeginCash, Expr::value(Value::Decimal(param)));
//         }
//     }
//     let frozen_capital=data.frozen_capital;
//     if frozen_capital.is_set(){
//         let d=frozen_capital.unwrap();
//         if d> Decimal::new(0, 0){
//             let param=Some(Box::new(d));
//             update=update.col_expr(phoenix_ast_unitasset::Column::FrozenCapital, Expr::value(Value::Decimal(param)));
//         }
//     }

//     let trade_frozen_capital=data.trade_frozen_capital;
//     if trade_frozen_capital.is_set(){
//         let d=trade_frozen_capital.unwrap();
//         if d> Decimal::new(0, 0){
//             let param=Some(Box::new(d));
//             update=update.col_expr(phoenix_ast_unitasset::Column::TradeFrozenCapital, Expr::value(Value::Decimal(param)));
//         }
//     }

//     let cash_in_transit=data.cash_in_transit;
//     if cash_in_transit.is_set(){
//         let d=cash_in_transit.unwrap();
//         if d> Decimal::new(0, 0){
//             let param=Some(Box::new(d));
//             update=update.col_expr(phoenix_ast_unitasset::Column::CashInTransit, Expr::value(Value::Decimal(param)));
//         }
//     }

//     let credit_multiple=data.credit_multiple;
//     if credit_multiple.is_set(){
//         let d=credit_multiple.unwrap();
//         if d> Decimal::new(0, 0){
//             let param=Some(Box::new(d));
//             update=update.col_expr(phoenix_ast_unitasset::Column::CreditMultiple, Expr::value(Value::Decimal(param)));
//         }
//     }


//     let id=data.unit_id;
//     if id.is_not_set(){
//         log::info!("update_by_expr数据操作失败，用户id未传:{:?}",id);
//         return Err(anyhow!(constdata::SQL_INSERT_OR_UPDATE_ERROR));
//     }
//     update.filter(phoenix_ast_unitasset::Column::UnitId.eq(id.unwrap()))
//     .exec(db)
//     .await?;

    
//     return Ok(());
// }



