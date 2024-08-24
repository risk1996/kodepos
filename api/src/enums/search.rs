use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchColumn {
  Id,
  Name,
  PostalCode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchOperation {
  #[serde(rename = "equal")]
  Equal,
  #[serde(rename = "start")]
  StartsWith,
  #[serde(rename = "contain")]
  Contains,
}

impl Default for SearchOperation {
  fn default() -> Self {
    Self::Equal
  }
}
