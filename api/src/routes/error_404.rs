use axum::{http::StatusCode, response::IntoResponse};

pub async fn handle_404() -> impl IntoResponse {
  let status = StatusCode::NOT_FOUND;
  let message = "Data yang anda cari tidak ditemukan";
  (status, message)
}
