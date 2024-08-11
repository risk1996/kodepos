mod data_structure;

use std::fs;

use data_structure::Data;

fn main() {
  let data = fs::read_to_string("./data/kodepos.json").expect("Failed to master data");
  let data = serde_json::from_str::<Data>(&data).expect("Failed to parse data");

  println!("{:?}", data);
}
