use crate::{endpoint, Error};
use super::types::XIVType;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Item {
  pub id: u32,
  pub name: String,
  pub can_be_hq: bool,
}

impl Default for Item {
  fn default() -> Self {
    Self {
      id: 0,
      name: "".to_owned(),
      can_be_hq: false,
    }
  }
}

#[allow(dead_code)]
impl Item {
  pub fn new(id: u32, name: String, can_be_hq: bool) -> Self {
    Self { id, name, can_be_hq}
  }

  pub fn default_item() -> Self {
    Self::default()
  }

  pub async fn get_item_from_id(id: u32) -> Result<Item, Box<dyn Error>> {
    let data = endpoint::get_data(id, XIVType::Item).await?;
    let fields = data["fields"].as_object().ok_or("No 'fields' found in JSON")?;

    let id = id;
    let name_value = fields["Name"].as_str().ok_or("No 'Name' found in JSON")?;
    let can_be_hq_value = fields["CanBeHq"].as_bool().unwrap_or(false);

    let item = Item {
      id,
      name: name_value.to_string(),
      can_be_hq: can_be_hq_value
    };

    Ok(item)
  }
}