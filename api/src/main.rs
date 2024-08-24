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

use crate::{routes::error_404::handle_404, state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let db_url = var("DATABASE_URL")?;
  let db: DatabaseConnection = Database::connect(db_url).await?;

  let app_state = Arc::new(AppState { db });
  let app = Router::new()
    .route("/", get(routes::about::get_about))
    .route("/id", get(routes::administrative_area_list::get_by_id))
    .route("/name", get(routes::administrative_area_list::get_by_name))
    .route(
      "/kodepos",
      get(routes::administrative_area_list::get_by_postal_code),
    )
    .fallback(handle_404)
    .with_state(app_state);

  let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

  Ok(serve(listener, app).await.unwrap())
}
