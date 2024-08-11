use sea_orm_migration::{prelude::*, schema::*};

use entity::models::administrative_area::AdministrativeArea;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(AdministrativeArea::Table)
          .if_not_exists()
          .col(string(AdministrativeArea::Id).primary_key())
          .col(string_null(AdministrativeArea::ParentId))
          .col(small_integer(AdministrativeArea::Level))
          .col(string(AdministrativeArea::Name))
          .col(string_null(AdministrativeArea::PostalCode))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(AdministrativeArea::Table).to_owned())
      .await
  }
}
