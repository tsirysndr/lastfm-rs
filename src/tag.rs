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
}
