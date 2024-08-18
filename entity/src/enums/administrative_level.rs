use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(
  Debug,
  Clone,
  PartialEq,
  Eq,
  Hash,
  Ord,
  PartialOrd,
  Serialize,
  Deserialize,
  EnumIter,
  DeriveActiveEnum,
)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
pub enum AdministrativeLevel {
  Province = 1,
  City = 2,
  District = 3,
  Village = 4,
}
