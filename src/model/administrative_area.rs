use sea_orm::entity::prelude::*;
use sea_orm::DeriveEntityModel;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
pub enum AdministrativeLevel {
  Province = 1,
  City = 2,
  District = 3,
  Village = 4,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, DeriveEntityModel)]
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
