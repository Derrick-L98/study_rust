//! SeaORM Entity. Generated by sea-orm-codegen 0.10.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "phoenix_oms_feeset")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i64,
  pub fee_type: String,
  pub exchange_id: i32,
  #[sea_orm(column_type = "Decimal(Some((12, 8)))")]
  pub capital_ratio: Decimal,
  #[sea_orm(column_type = "Decimal(Some((12, 8)))")]
  pub face_value_ratio: Decimal,
  #[sea_orm(column_type = "Decimal(Some((12, 8)))")]
  pub amount_ratio: Decimal,
  #[sea_orm(column_type = "Decimal(Some((12, 4)))")]
  pub maximum_fee: Decimal,
  #[sea_orm(column_type = "Decimal(Some((12, 4)))")]
  pub minimum_fee: Decimal,
  pub currency_no: Option<String>,
  pub modify_date: i32,
  pub order_direction: i32,
  pub unit_id: i64,
  pub serfee_type: i32,
  pub close_type: i32,
  pub stock_type: i32,
  pub decimal_type: i32,
  pub channel_id: i32,
  pub plate_id: i32,
  pub tid: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
