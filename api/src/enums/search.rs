use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchColumn {
  #[serde(rename = "id")]
  Id,
  #[serde(rename = "name")]
  Name,
  #[serde(rename = "kodepos")]
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
