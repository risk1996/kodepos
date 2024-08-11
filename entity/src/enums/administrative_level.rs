use sea_orm::{DeriveActiveEnum, EnumIter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
pub enum AdministrativeLevel {
  Province = 1,
  City = 2,
  District = 3,
  Village = 4,
}
