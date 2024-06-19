use crate::{endpoint, xiv::Error};
use super::types::XIVType;

pub struct Item {
  pub id: u32,
  pub name: String,
}

impl Default for Item {
  fn default() -> Self {
    Self {
      id: 0,
      name: "".to_owned(),
    }
  }
}

#[allow(dead_code)]
impl Item {
  pub fn new(id: u32, name: String) -> Self {
    Self { id, name }
  }

  pub fn default_item() -> Self {
    Self::default()
  }

  pub fn get_item_from_id(id: u32) -> Result<Item, Box<dyn Error>> {
    let data = endpoint::get_data(id, XIVType::Item)?;
    let fields = data["fields"].as_object().ok_or("No 'fields' found in JSON")?;
    let name_value = fields["Name"].as_str().ok_or("No 'Name' found in JSON")?;
    let id = id;
    let item = Item {
      id,
      name: name_value.to_string(),
    };

    Ok(item)
  }
}