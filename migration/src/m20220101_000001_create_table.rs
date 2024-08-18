use sea_orm_migration::prelude::*;

use entity::models::administrative_area::AdministrativeArea;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let db = manager.get_connection();

    // NOTE: Unable to use Table::create() because sea_orm does not support SQLite's FTS5 yet
    // manager
    //   .create_table(
    //     Table::create()
    //       .table(AdministrativeArea::Table)
    //       .if_not_exists()
    //       .col(string(AdministrativeArea::Id).primary_key())
    //       .col(string_null(AdministrativeArea::ParentId))
    //       .col(small_integer(AdministrativeArea::Level))
    //       .col(string(AdministrativeArea::Name))
    //       .col(string_null(AdministrativeArea::PostalCode))
    //       .to_owned(),
    //   )
    //   .await;
    db.execute_unprepared(
      "CREATE VIRTUAL TABLE IF NOT EXISTS `administrative_area` USING fts5 (
        id,
        parent_id,
        level,
        name,
        postal_code,
        FOREIGN KEY parent_id REFERENCES administrative_area(id)
      )",
    )
    .await?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(AdministrativeArea::Table).to_owned())
      .await
  }
}
