//! SeaORM Entity. Generated by sea-orm-codegen 0.10.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "phoenix_ord_stockdeal")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i64,
  #[sea_orm(unique)]
  pub deal_no: i64,
  pub sys_date: i32,
  pub order_no: i64,
  pub exchange_id: i32,
  pub unit_id: i64,
  pub stock_id: i64,
  pub stock_code: String,
  pub order_direction: i32,
  pub deal_time: i32,
  pub deal_amount: i32,
  #[sea_orm(column_type = "Decimal(Some((16, 8)))")]
  pub deal_price: Decimal,
  #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
  pub fee_jy: Decimal,
  #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
  pub fee_gh: Decimal,
  #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
  pub fee_yj: Decimal,
  #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
  pub fee_js: Decimal,
  #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
  pub fee_zg: Decimal,
  #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
  pub fee_qt: Decimal,
  #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
  pub fee_js2: Decimal,
  #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
  pub fee_jg: Decimal,
  #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
  pub fee_yh: Decimal,
  #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
  pub fee_real_yj: Decimal,
  #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
  pub fee_total: Decimal,
  pub exec_no: String,
  pub channel_type: i32,
  pub channel_id: i32,
  #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
  pub refer_profit: Decimal,
  pub trade_type: i32,
  pub create_time: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}