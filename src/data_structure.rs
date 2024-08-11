use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::{AdministrativeArea, AdministrativeLevel};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data(HashMap<String, AdministrationLevel1>);
impl Data {
  pub fn into_models(&self) -> Vec<AdministrativeArea> {
    self
      .0
      .iter()
      .flat_map(|(l1_key, l1_value)| l1_value.into_models(l1_key.clone()))
      .collect()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdministrationLevel1 {
  #[serde(rename = "ID")]
  pub id: String,
  #[serde(rename = "Kabupaten/Kota")]
  pub cities: HashMap<String, AdministrationLevel2>,
}
impl AdministrationLevel1 {
  pub fn into_model(&self, name: String) -> AdministrativeArea {
    AdministrativeArea {
      id: self.id.clone(),
      parent_id: None,
      level: AdministrativeLevel::Province,
      name,
      postal_code: None,
    }
  }

  pub fn into_models(&self, name: String) -> Vec<AdministrativeArea> {
    vec![self.into_model(name)]
      .into_iter()
      .chain(
        self
          .cities
          .iter()
          .flat_map(|(l2_key, l2_value)| l2_value.into_models(l2_key.clone(), self.id.clone()))
          .collect::<Vec<_>>(),
      )
      .collect()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdministrationLevel2 {
  #[serde(rename = "ID")]
  pub id: String,
  #[serde(rename = "Kecamatan")]
  pub districts: HashMap<String, AdministrationLevel3>,
}
impl AdministrationLevel2 {
  pub fn into_model(&self, name: String, parent_id: String) -> AdministrativeArea {
    AdministrativeArea {
      id: self.id.clone(),
      parent_id: Some(parent_id),
      level: AdministrativeLevel::City,
      name,
      postal_code: None,
    }
  }

  pub fn into_models(&self, name: String, parent_id: String) -> Vec<AdministrativeArea> {
    vec![self.into_model(name, parent_id)]
      .into_iter()
      .chain(
        self
          .districts
          .iter()
          .flat_map(|(l3_key, l3_value)| l3_value.into_models(l3_key.clone(), self.id.clone()))
          .collect::<Vec<_>>(),
      )
      .collect()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdministrationLevel3 {
  #[serde(rename = "ID")]
  pub id: String,
  #[serde(rename = "Kelurahan/Desa")]
  pub villages: HashMap<String, AdministrationLevel4>,
}
impl AdministrationLevel3 {
  pub fn into_model(&self, name: String, parent_id: String) -> AdministrativeArea {
    AdministrativeArea {
      id: self.id.clone(),
      parent_id: Some(parent_id),
      level: AdministrativeLevel::District,
      name,
      postal_code: None,
    }
  }

  pub fn into_models(&self, name: String, parent_id: String) -> Vec<AdministrativeArea> {
    vec![self.into_model(name, parent_id)]
      .into_iter()
      .chain(
        self
          .villages
          .iter()
          .map(|(l4_key, l4_value)| l4_value.into_model(l4_key.clone(), self.id.clone()))
          .collect::<Vec<_>>(),
      )
      .collect()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdministrationLevel4 {
  #[serde(rename = "ID")]
  pub id: String,
  #[serde(rename = "Kode Pos")]
  pub postal_code: String,
}
impl AdministrationLevel4 {
  pub fn into_model(&self, name: String, parent_id: String) -> AdministrativeArea {
    AdministrativeArea {
      id: self.id.clone(),
      parent_id: Some(parent_id),
      level: AdministrativeLevel::Village,
      name,
      postal_code: Some(self.postal_code.clone()),
    }
  }
}
