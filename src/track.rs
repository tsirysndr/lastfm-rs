use crate::artist::Artist;
use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct TrackResponse {
  pub track: Track<String, Streamable, u32>,
}

#[derive(Debug, Deserialize)]
pub struct Track<T, U, V> {
  pub streamable: U,
  pub duration: Option<T>,
  pub url: String,
  pub name: String,
  #[serde(rename = "@attr")]
  pub attr: Option<Attr<V>>,
  pub artist: Artist,
}

#[derive(Debug, Deserialize)]
pub struct Tracks<T, U, V> {
  pub track: Vec<Track<T, U, V>>,
}

#[derive(Debug, Deserialize)]
pub struct Streamable {
  pub fulltrack: String,
  #[serde(rename = "#text")]
  pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Attr<T> {
  pub rank: Option<T>,
  pub r#for: Option<String>,
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

  pub async fn get_info(&self, artist: &str, title: &str) -> Result<TrackResponse, surf::Error> {
    let res = self
      .client
      .get(format!(
        "?method=track.getinfo&artist={}&track={}&api_key={}&format=json",
        artist, title, self.api_key
      ))
      .recv_json::<TrackResponse>()
      .await?;
    Ok(res)
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
