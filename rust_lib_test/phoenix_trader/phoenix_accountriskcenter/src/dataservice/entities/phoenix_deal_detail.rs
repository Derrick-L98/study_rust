//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "phoenix_deal_detail")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i64,
  pub p_date: i32,
  pub p_order_no: i64,
  pub p_exchange_id: i64,
  pub p_stock_id: i64,
  pub p_stock_code: String,
  pub p_order_direction: i32,
  pub p_deal_amount: i32,
  pub p_prebuy_amount: i32,
  #[sea_orm(column_type = "Decimal(Some((20, 8)))")]
  pub p_deal_price: Decimal,
  #[sea_orm(column_type = "Decimal(Some((20, 8)))")]
  pub p_fee_total: Decimal,
  #[sea_orm(column_type = "Decimal(Some((20, 8)))")]
  pub p_currency_rate: Decimal,
  #[sea_orm(column_type = "Decimal(Some((20, 8)))")]
  pub p_currency_rate_cj: Decimal,
  pub p_unit_id: i64,
  pub p_channel_type: i32,
  pub p_channel_id: i64,
  pub p_qfii_state: i32,
  pub p_account_id: i64,
  pub p_trade_type: i32,
  pub p_stock_type: i32,
  #[sea_orm(column_type = "Decimal(Some((20, 8)))")]
  pub p_leverage: Decimal,
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
