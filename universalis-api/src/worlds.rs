#[derive(serde::Deserialize, serde::Serialize)]
pub struct DataCenter {
  pub name: String,
  pub worlds: Vec<World>,
}

impl Default for DataCenter {
  fn default() -> Self {
    Self {
      name: "".to_owned(),
      worlds: Vec::new()
    }
  }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct World {
  pub name: String,
}

impl Default for World {
  fn default() -> Self {
    Self {
      name: "".to_owned(),
    }
  }
}
