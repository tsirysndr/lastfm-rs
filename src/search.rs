use surf::Client;

pub struct SearchService {
  client: Client,
  api_key: String,
}

impl SearchService {
  pub fn new(client: &Client, api_key: &str) -> Self {
    Self {
      client: client.clone(),
      api_key: api_key.to_string(),
    }
  }
}
