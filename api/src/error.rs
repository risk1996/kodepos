use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let status = StatusCode::INTERNAL_SERVER_ERROR;
    let message = format!("Something went wrong: {}", self.0);
    (status, message).into_response()
  }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
  fn from(err: E) -> Self {
    Self(err.into())
  }
}
