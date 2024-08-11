use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type Data = HashMap<String, AdministrationLevel1>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdministrationLevel1 {
  #[serde(rename = "ID")]
  id: String,
  #[serde(rename = "Kabupaten/Kota")]
  cities: HashMap<String, AdministrationLevel2>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdministrationLevel2 {
  #[serde(rename = "ID")]
  id: String,
  #[serde(rename = "Kecamatan")]
  districts: HashMap<String, AdministrationLevel3>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdministrationLevel3 {
  #[serde(rename = "ID")]
  id: String,
  #[serde(rename = "Kelurahan/Desa")]
  villages: HashMap<String, AdministrationLevel4>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdministrationLevel4 {
  #[serde(rename = "ID")]
  id: String,
  #[serde(rename = "Kode Pos")]
  postal_code: String,
}
