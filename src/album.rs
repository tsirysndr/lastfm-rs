use crate::tag::Tags;
use crate::track::Attr;
use crate::track::Tracks;
use serde::{Deserialize, Serialize};
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct AlbumResponse {
  pub album: Album,
}

#[derive(Debug, Deserialize)]
pub struct Album {
  pub artist: String,
  pub mbid: String,
  pub tags: Option<Tags>,
  pub playcount: Option<String>,
  pub image: Vec<Image>,
  pub tracks: Option<Tracks>,
  pub url: String,
  pub name: String,
  pub listeners: Option<String>,
  pub wiki: Option<Wiki>,
  pub streamable: Option<String>,
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

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
  pub results: Results,
}

#[derive(Debug, Deserialize)]
pub struct SearchResults {
  pub results: Results,
}

#[derive(Debug, Deserialize)]
pub struct Results {
  #[serde(rename = "opensearch:Query")]
  pub opensearch_query: OpensearchQuery,
  #[serde(rename = "opensearch:totalResults")]
  pub opensearch_total_results: String,
  #[serde(rename = "opensearch:startIndex")]
  pub openserch_start_index: String,
  #[serde(rename = "opensearch:itemsPerPage")]
  pub opensearch_items_per_page: String,
  pub albummatches: Albummatches,
  #[serde(rename = "@attr")]
  pub attr: Attr,
}

#[derive(Debug, Deserialize)]
pub struct OpensearchQuery {
  #[serde(rename = "#text")]
  pub text: String,
  pub role: String,
  #[serde(rename = "searchTerms")]
  pub search_terms: String,
  #[serde(rename = "startPage")]
  pub start_page: String,
}

#[derive(Debug, Deserialize)]
pub struct Albummatches {
  pub album: Vec<Album>,
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

  pub async fn search(&self, title: &str) -> Result<SearchResults, surf::Error> {
    let res = self
      .client
      .get(format!(
        "?method=album.search&album={}&api_key={}&format=json",
        title, self.api_key
      ))
      .recv_json::<SearchResults>()
      .await?;
    Ok(res)
  }
}
