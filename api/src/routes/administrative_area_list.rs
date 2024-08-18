use std::sync::Arc;

use axum::{
  extract::{Path, Query, State},
  Json,
};
use entity::models::administrative_area;
use sea_orm::prelude::*;

use crate::{
  dto::{request::ApiSearchRequest, response::ApiListResponse},
  enums::search::{SearchColumn, SearchOperation},
  error::AppError,
  state::AppState,
};

pub async fn get_administrative_areas(
  Path(column): Path<SearchColumn>,
  State(state): State<Arc<AppState>>,
  Query(request): Query<ApiSearchRequest>,
) -> Result<Json<ApiListResponse<administrative_area::Model>>, AppError> {
  let AppState { db } = state.as_ref();
  let ApiSearchRequest { q, op, page, size } = request;

  let column = match column {
    | SearchColumn::Id => administrative_area::Column::Id,
    | SearchColumn::Name => administrative_area::Column::Name,
    | SearchColumn::PostalCode => administrative_area::Column::PostalCode,
  };
  let filter = match op {
    | SearchOperation::Equal => column.eq(q),
    | SearchOperation::StartsWith => column.starts_with(q),
    | SearchOperation::Contains => column.contains(q),
  };
  let result = administrative_area::Entity::find()
    .filter(filter.clone())
    .paginate(db, size as u64)
    .fetch_page(page as u64)
    .await?;
  let total = administrative_area::Entity::find()
    .filter(filter.clone())
    .count(db)
    .await? as usize;

  Ok(Json(ApiListResponse::new(result, page, size, total)))
}
