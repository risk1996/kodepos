use serde::{Deserialize, Serialize};

use crate::enums::search::SearchOperation;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiSearchRequest {
  pub q: String,
  pub op: SearchOperation,
  pub page: usize,
  pub size: usize,
}

impl Default for ApiSearchRequest {
  fn default() -> Self {
    Self {
      q: "".to_string(),
      op: Default::default(),
      page: 1,
      size: 20,
    }
  }
}
