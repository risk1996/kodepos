use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

type Data = HashMap<String, AdministrationLevel1>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AdministrationLevel1 {
  #[serde(rename = "ID")]
  id: String,
  #[serde(rename = "Kabupaten/Kota")]
  cities: HashMap<String, AdministrationLevel2>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AdministrationLevel2 {
  #[serde(rename = "ID")]
  id: String,
  #[serde(rename = "Kecamatan")]
  districts: HashMap<String, AdministrationLevel3>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AdministrationLevel3 {
  #[serde(rename = "ID")]
  id: String,
  #[serde(rename = "Kelurahan/Desa")]
  villages: HashMap<String, AdministrationLevel4>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AdministrationLevel4 {
  #[serde(rename = "ID")]
  id: String,
  #[serde(rename = "Kode Pos")]
  postal_code: String,
}

fn main() {
  let data = fs::read_to_string("./data/kodepos.json").expect("Failed to master data");
  let data = serde_json::from_str::<Data>(&data).expect("Failed to parse data");

  println!("{:?}", data);
}
