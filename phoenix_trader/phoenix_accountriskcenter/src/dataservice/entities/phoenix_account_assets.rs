//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "phoenix_account_assets")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i64,
  #[sea_orm(unique)]
  pub p_account_id: i64,
  #[sea_orm(column_type = "Decimal(Some((20, 4)))")]
  pub p_current_principal: Decimal,
  #[sea_orm(column_type = "Decimal(Some((20, 4)))")]
  pub p_credit_cash: Decimal,
  #[sea_orm(column_type = "Decimal(Some((20, 4)))")]
  pub p_current_financial: Decimal,
  #[sea_orm(column_type = "Decimal(Some((20, 8)))")]
  pub p_position_value: Decimal,
  #[sea_orm(column_type = "Decimal(Some((20, 8)))")]
  pub p_position_value_star: Decimal,
  #[sea_orm(column_type = "Decimal(Some((20, 4)))")]
  pub p_prebuy_capital_star: Decimal,
  #[sea_orm(column_type = "Decimal(Some((20, 4)))")]
  pub p_financing_borrowed: Decimal,
  #[sea_orm(column_type = "Decimal(Some((20, 4)))")]
  pub p_financing_occupied: Decimal,
  #[sea_orm(column_type = "Decimal(Some((20, 8)))")]
  pub p_fee_total_hkd: Decimal,
  #[sea_orm(column_type = "Decimal(Some((20, 8)))")]
  pub p_real_profit: Decimal,
  #[sea_orm(column_type = "Decimal(Some((20, 8)))")]
  pub p_floating_profit: Decimal,
  pub p_account_type: i32,
  pub p_date: i32,
  pub p_updatetime: i64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
  fn def(&self) -> RelationDef {
    panic!("No RelationDef")
  }
}

impl ActiveModelBehavior for ActiveModel {}