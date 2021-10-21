use surf::Client;

pub struct TagService {
  client: Client,
}

impl TagService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn get_info(&self) {}

  pub fn get_similar(&self) {}

  pub fn get_top_albums(&self) {}

  pub fn get_top_artists(&self) {}

  pub fn get_top_tags(&self) {}

  pub fn get_top_tracks(&self) {}

  pub fn get_weekly_chart_list(&self) {}
}
