pub enum XIVType {
  Item,
}

impl XIVType {
  pub fn get_type_str(&self) -> &str {
    match self {
      XIVType::Item => "Item"
    }
  }
}