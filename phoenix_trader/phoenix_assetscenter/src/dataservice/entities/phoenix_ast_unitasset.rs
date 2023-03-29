//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "phoenix_ast_unitasset")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i64,
  pub sys_date: i32,
  pub unit_id: i64,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub begin_cash: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub current_cash: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub frozen_capital: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub trade_frozen_capital: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub cash_in_transit: Decimal,
  pub currency_no: String,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub credit_multiple: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub enable_cash: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub today_total_value: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub today_deposit: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub today_withdraw: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub total_deposit: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub total_withdraw: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub gem_frozen_capital: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
  fn def(&self) -> RelationDef {
    panic!("No RelationDef")
  }
}

impl ActiveModelBehavior for ActiveModel {}
