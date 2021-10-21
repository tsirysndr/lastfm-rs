use surf::Client;

pub struct ArtistService {
  client: Client,
}

impl ArtistService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn add_tags(&self) {}

  pub fn get_correction(&self) {}

  pub fn get_info(&self) {}

  pub fn get_similar(&self) {}

  pub fn get_tags(&self) {}

  pub fn get_top_albums(&self) {}

  pub fn get_top_tags(&self) {}

  pub fn get_top_tracks(&self) {}

  pub fn remove_tag(&self) {}

  pub fn search(&self) {}
}
