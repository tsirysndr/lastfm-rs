use surf::Client;

pub struct TrackService {
  client: Client,
}

impl TrackService {
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

  pub fn get_top_tags(&self) {}

  pub fn love(&self) {}

  pub fn remove_tag(&self) {}

  pub fn scrobble(&self) {}

  pub fn search(&self) {}

  pub fn unlove(&self) {}

  pub fn update_now_playing(&self) {}
}
