use sea_orm::entity::prelude::*;
use sea_orm::DeriveEntityModel;
use serde::{Deserialize, Serialize};

use crate::enums::AdministrativeLevel;

#[derive(
  Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize, DeriveEntityModel,
)]
#[sea_orm(table_name = "administrative_area")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: String,
  pub parent_id: Option<String>,
  pub level: AdministrativeLevel,
  pub name: String,
  pub postal_code: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(belongs_to = "Entity", from = "Column::ParentId", to = "Column::Id")]
  SelfReferencing,
}

impl ActiveModelBehavior for ActiveModel {}

pub struct SelfReferencingLink;
impl Linked for SelfReferencingLink {
  type FromEntity = Entity;

  type ToEntity = Entity;

  fn link(&self) -> Vec<RelationDef> {
    vec![Relation::SelfReferencing.def()]
  }
}

#[derive(DeriveIden)]
pub enum AdministrativeArea {
  Table,
  Id,
  ParentId,
  Level,
  Name,
  PostalCode,
}
