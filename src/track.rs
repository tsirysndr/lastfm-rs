use crate::artist::Artist;
use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Track {
  pub streamable: Streamable,
  pub duration: u32,
  pub url: String,
  pub name: String,
  #[serde(rename = "@attr")]
  pub attr: Attr,
  pub artist: Artist,
}

#[derive(Debug, Deserialize)]
pub struct Tracks {
  pub track: Vec<Track>,
}

#[derive(Debug, Deserialize)]
pub struct Streamable {
  pub fulltrack: String,
  #[serde(rename = "#text")]
  pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Attr {
  pub rank: u32,
}

pub struct TrackService {
  client: Client,
  api_key: String,
}

impl TrackService {
  pub fn new(client: &Client, api_key: &str) -> Self {
    Self {
      client: client.clone(),
      api_key: api_key.to_string(),
    }
  }

  pub async fn add_tags(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_correction(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_info(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_similar(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_tags(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_top_tags(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn love(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn remove_tag(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn scrobble(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn search(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn unlove(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn update_now_playing(&self) -> Result<(), surf::Error> {
    Ok(())
  }
}
