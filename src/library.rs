use surf::Client;

pub struct LibraryService {
  client: Client,
  api_key: String,
}

impl LibraryService {
  pub fn new(client: &Client, api_key: &str) -> Self {
    Self {
      client: client.clone(),
      api_key: api_key.to_string(),
    }
  }

  pub async fn get_artists(&self) -> Result<(), surf::Error> {
    Ok(())
  }
}
