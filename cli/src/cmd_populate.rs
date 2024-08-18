use std::fs;

use anyhow::Result;
use entity::models::administrative_area;
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, IntoActiveModel};

use crate::data_structure::Data;

pub async fn populate(input: String, db: &DatabaseConnection) -> Result<()> {
  let data = fs::read_to_string(&input)?;
  let data = serde_json::from_str::<Data>(&data)?;
  let data: Vec<administrative_area::ActiveModel> = data
    .into_models()
    .into_iter()
    .map(|model| model.into_active_model())
    .collect();

  administrative_area::Entity::delete_many().exec(db).await?;

  for datum in data.chunks(64) {
    administrative_area::Entity::insert_many(datum.to_vec())
      .exec(db)
      .await?;
  }

  Ok(())
}
