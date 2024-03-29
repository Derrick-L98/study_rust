//! SeaORM Entity. Generated by sea-orm-codegen 0.10.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "phoenix_sys_dictionary")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i64,
  pub dictionary_no: i32,
  pub lemma_item: String,
  pub item_name: String,
  pub allow_edit: String,
  pub code_length: i32,
  pub opposite_no: i32,
  pub opposite_item: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
