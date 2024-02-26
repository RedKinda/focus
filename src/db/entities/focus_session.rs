//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "focus_session")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i64,
  pub user_id: i64,
  pub expires_at: DateTime,
  pub summarize: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::focus_summary_entry::Entity")]
  FocusSummaryEntry,
}

impl Related<super::focus_summary_entry::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::FocusSummaryEntry.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
