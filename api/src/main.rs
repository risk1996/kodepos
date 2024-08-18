mod dto;
mod enums;
mod error;
mod routes;
mod state;

use std::sync::Arc;

use axum::{routing::get, serve, Router};
use dotenvy::var;
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;

use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let db_url = var("DATABASE_URL")?;
  let db: DatabaseConnection = Database::connect(db_url).await?;

  let app_state = Arc::new(AppState { db });
  let app = Router::new()
    .route("/", get(routes::about::get_about))
    .route(
      "/:column",
      get(routes::administrative_area_list::get_administrative_areas),
    )
    .with_state(app_state);

  let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

  Ok(serve(listener, app).await.unwrap())
}
