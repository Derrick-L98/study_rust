//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "phoenix_risk_details")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i64,
  pub sys_date: i32,
  pub user_id: i64,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub current_cash: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub credit_cash: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub real_cash: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub warn_line: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub close_line: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 5)))")]
  pub risk_rate: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub credit_multiple: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub loan_cash: Decimal,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub total_value: Decimal,
  pub create_datetime: i64,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub total_asset_value: Decimal,
  pub risk_type: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
  fn def(&self) -> RelationDef {
    panic!("No RelationDef")
  }
}

impl ActiveModelBehavior for ActiveModel {}
