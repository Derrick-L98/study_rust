//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "phoenix_ast_operation_detail")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  pub sys_date: i32,
  pub op_businflag: String,
  pub unit_id: i64,
  #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
  pub occur_capital: Decimal,
  pub remark: String,
  pub ext_id: i64,
  pub currency_no: String,
  pub operator: i32,
  pub create_time: i64,
  pub op_type: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
  fn def(&self) -> RelationDef {
    panic!("No RelationDef")
  }
}

impl ActiveModelBehavior for ActiveModel {}
