use crate::album::Image;
use crate::tag::Tags;
use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Artist {
  pub url: String,
  pub name: String,
  pub mbid: String,
  pub image: Option<Vec<Image>>,
  pub streamable: Option<String>,
  pub ontour: Option<String>,
  pub similar: Option<Similar>,
  pub stats: Option<Stats>,
  pub tags: Option<Tags>,
  pub bio: Option<Bio>,
}

#[derive(Debug, Deserialize)]
pub struct Stats {
  pub listeners: String,
  pub playcount: String,
}

#[derive(Debug, Deserialize)]
pub struct Similar {
  pub artist: Vec<Artist>,
}

#[derive(Debug, Deserialize)]
pub struct Bio {
  pub links: Links,
  pub published: String,
  pub summary: String,
  pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct Links {
  pub link: Vec<Link>,
}

#[derive(Debug, Deserialize)]
pub struct Link {
  #[serde(rename = "#text")]
  pub text: String,
  pub rel: String,
  pub href: String,
}

pub struct ArtistService {
  client: Client,
  api_key: String,
}

impl ArtistService {
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

  pub async fn get_top_albums(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_top_tags(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_top_tracks(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn remove_tag(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn search(&self) -> Result<(), surf::Error> {
    Ok(())
  }
}
