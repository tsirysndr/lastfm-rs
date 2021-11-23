use crate::tag::Tags;
use crate::track::Tracks;
use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct AlbumResponse {
  pub album: Album,
}

#[derive(Debug, Deserialize)]
pub struct Album {
  pub artist: String,
  pub mbid: String,
  pub tags: Tags,
  pub playcount: String,
  pub image: Vec<Image>,
  pub tracks: Tracks,
  pub url: String,
  pub name: String,
  pub listeners: String,
  pub wiki: Wiki,
}

#[derive(Debug, Deserialize)]
pub struct Image {
  pub size: String,
  #[serde(rename = "#text")]
  pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Wiki {
  pub published: String,
  pub summary: String,
  pub content: String,
}

pub struct AlbumService {
  client: Client,
  api_key: String,
}

impl AlbumService {
  pub fn new(client: &Client, api_key: &str) -> Self {
    Self {
      client: client.clone(),
      api_key: api_key.to_string(),
    }
  }

  pub async fn add_tags(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_info(&self, artist: &str, title: &str) -> Result<AlbumResponse, surf::Error> {
    let res = self
      .client
      .get(format!(
        "?method=album.getinfo&artist={}&album={}&api_key={}&format=json",
        artist, title, self.api_key
      ))
      .recv_json::<AlbumResponse>()
      .await?;
    Ok(res)
  }

  pub async fn get_tags(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_top_tags(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn remove_tag(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn search(&self) -> Result<(), surf::Error> {
    Ok(())
  }
}
