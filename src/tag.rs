use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Tag {
  pub name: String,
  pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Tags {
  pub tag: Vec<Tag>,
}

pub struct TagService {
  client: Client,
  api_key: String,
}

impl TagService {
  pub fn new(client: &Client, api_key: &str) -> Self {
    Self {
      client: client.clone(),
      api_key: api_key.to_string(),
    }
  }

  pub async fn get_info(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_similar(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_top_albums(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_top_artists(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_top_tags(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_top_tracks(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_weekly_chart_list(&self) -> Result<(), surf::Error> {
    Ok(())
  }
}
