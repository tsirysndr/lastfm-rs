use surf::Client;

pub struct AlbumService {
  client: Client,
}

impl AlbumService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn add_tags(&self) {}

  pub fn get_info(&self) {}

  pub fn get_tags(&self) {}

  pub fn get_top_tags(&self) {}

  pub fn remove_tag(&self) {}

  pub fn search(&self) {}
}
