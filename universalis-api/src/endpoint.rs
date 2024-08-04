use crate::{worlds::DataCenter, worlds::World, Client, Error, Value, USER_AGENT};
const BASE_URL: &str = "";

pub async fn get_data_from_dc(endpoint: u32, dc: DataCenter) -> Result<Vec<Value>, Box<dyn Error>> {
  let mut datas: Vec<Value> = Vec::new();
  for world in dc.worlds {
    let data: Value = get_data_from_world(endpoint, world).await?;
    datas.push(data);
  }

  Ok(datas)
}

pub async fn get_data_from_world(endpoint: u32, world: World) -> Result<Value, Box<dyn Error>> {
  let endpoint_str: String = endpoint.to_string();
  let url = format!("{}/{}/{}", BASE_URL, endpoint_str, world.name);
  let data: Value = get_data(url).await?;
  Ok(data)
}

pub async fn get_data(url: String) -> Result<Value, Box<dyn Error>> {
  let client = Client::new();
  let response = client
    .get(&url)
    .header(USER_AGENT, "<User-Agent>")
    .send()
    .await?;

  let res = response.text().await?;
  let data: Value = serde_json::from_str(&res)?;
  Ok(data)
}
