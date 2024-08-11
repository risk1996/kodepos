use std::fs;

use anyhow::Result;

use crate::data_structure::Data;

pub fn populate(input: String, output: String) -> Result<()> {
  let data = fs::read_to_string(&input)?;
  let data = serde_json::from_str::<Data>(&data)?;
  let data = data.into_models();
  println!("{:?}", data);
  println!("Length: {}", data.len());

  Ok(())
}
