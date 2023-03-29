//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "phoenix_ord_settledetail")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i64,
  pub sys_date: i32,
  pub settle_id: i64,
  pub unit_id: i64,
  pub stock_code: String,
  pub order_direction: i32,
  pub order_amount: i32,
  #[sea_orm(column_type = "Decimal(Some((14, 8)))")]
  pub deal_price: Decimal,
  #[sea_orm(column_type = "Decimal(Some((10, 6)))")]
  pub currency_rate: Decimal,
  #[sea_orm(column_type = "Decimal(Some((10, 6)))")]
  pub real_profit: Decimal,
  pub settle_amount: i32,
  pub settle_internal_amount: i32,
  #[sea_orm(column_type = "Decimal(Some((18, 2)))")]
  pub settle_balance: Decimal,
  #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
  pub fee_total: Decimal,
  #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
  pub fee_jy: Decimal,
  #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
  pub fee_yh: Decimal,
  #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
  pub fee_gh: Decimal,
  #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
  pub fee_yj: Decimal,
  #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
  pub fee_js: Decimal,
  #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
  pub fee_zg: Decimal,
  #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
  pub fee_other: Decimal,
  #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
  pub fee_js2: Decimal,
  #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
  pub fee_jg: Decimal,
  pub settle_type: i32,
  pub currency_no: String,
  pub remark: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
  fn def(&self) -> RelationDef {
    panic!("No RelationDef")
  }
}

impl ActiveModelBehavior for ActiveModel {}
