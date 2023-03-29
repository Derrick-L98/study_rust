//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "phoenix_ord_pendsettle")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i64,
  pub settle_id: i64,
  pub unit_id: i64,
  pub sys_date: i32,
  pub trade_date: i32,
  pub settle_date: i32,
  pub clear_speed: i32,
  pub order_no: i64,
  pub order_amount: i32,
  #[sea_orm(column_type = "Decimal(Some((14, 8)))", nullable)]
  pub deal_avg_price: Option<Decimal>,
  pub exchange_id: i32,
  pub stock_id: i64,
  pub order_direction: i32,
  pub settle_amount: i32,
  pub settle_internal_amount: i32,
  #[sea_orm(column_type = "Decimal(Some((18, 2)))")]
  pub settle_balance: Decimal,
  #[sea_orm(column_type = "Decimal(Some((18, 2)))")]
  pub net_balance: Decimal,
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
  #[sea_orm(column_type = "Decimal(Some((12, 2)))", nullable)]
  pub fee_js2: Option<Decimal>,
  #[sea_orm(column_type = "Decimal(Some((12, 2)))", nullable)]
  pub fee_jg: Option<Decimal>,
  pub asset_settle_date: i32,
  pub p_status: i32,
  pub currency_no: String,
  pub remark: Option<String>,
  #[sea_orm(column_type = "Decimal(Some((10, 6)))")]
  pub currency_rate: Decimal,
  pub channel_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
  fn def(&self) -> RelationDef {
    panic!("No RelationDef")
  }
}

impl ActiveModelBehavior for ActiveModel {}
