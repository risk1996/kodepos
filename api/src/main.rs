mod error;

use axum::{extract, routing::get, serve, Json, Router};
use entity::models::administrative_area::Model as AdministrativeArea;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
  let app = Router::new()
    .route("/", get(get_about))
    .route("/id", get(get_administrative_area_by_id));
  let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
  serve(listener, app).await.unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct About {
  name: String,
  version: String,
  description: String,
  author: String,
}

async fn get_about() -> Json<About> {
  Json(About {
    name: "Kodepos".to_string(),
    version: "0.1.0".to_string(),
    description: "API untuk mencari daerah dari ID, nama, atau kode pos".to_string(),
    author: "William Darian <williamdariansutedjo@gmail.com>".to_string(),
  })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SearchRequest {
  q: String,
  exact: Option<bool>,
}

async fn get_administrative_area_by_id(
  extract::Query(_request): extract::Query<SearchRequest>,
) -> Json<AdministrativeArea> {
  todo!()
}
