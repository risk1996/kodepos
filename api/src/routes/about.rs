use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct About {
  name: String,
  version: String,
  description: String,
  author: String,
}

/* cSpell:disable */
pub async fn get_about() -> Json<About> {
  Json(About {
    name: "Kodepos".to_string(),
    version: "0.1.0".to_string(),
    description: "API untuk mencari daerah di Indonesia berdasarkan ID, nama, atau kode pos"
      .to_string(),
    author: "William Darian <williamdariansutedjo@gmail.com>".to_string(),
  })
}
/* cSpell:enable */
