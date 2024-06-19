use crate::xiv::{types::XIVType, Client, Error, Value, USER_AGENT};
const BASE_URL: &str = "https://beta.xivapi.com/api/1/sheet";

pub fn get_data(endpoint: u32, xiv_type: XIVType) -> Result<Value, Box<dyn Error>> {
  let client = Client::new();
  let endpoint_str: String = endpoint.to_string();
  let url = format!("{}/{}/{}", BASE_URL, xiv_type.get_str(), endpoint_str);

  let response = client
    .get(&url)
    .header(USER_AGENT, "<User-Agent>")
    .send()?
    .text()?;

  let data: Value = serde_json::from_str(&response)?;
  Ok(data)
}