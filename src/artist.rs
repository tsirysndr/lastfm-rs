use crate::album::Albums;
use crate::album::Image;
use crate::album::OpensearchQuery;
use crate::tag::Tags;
use crate::track::Attr;
use crate::track::Tracks;
use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct ArtistResponse {
  pub artist: Artist,
}

#[derive(Debug, Deserialize)]
pub struct SimilarArtists {
  pub similarartists: Similar,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
  pub url: String,
  pub name: String,
  pub mbid: Option<String>,
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
  pub link: Link,
}

#[derive(Debug, Deserialize)]
pub struct Link {
  #[serde(rename = "#text")]
  pub text: String,
  pub rel: String,
  pub href: String,
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
  pub artistmatches: Artistmatches,
  #[serde(rename = "@attr")]
  pub attr: Attr<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Artistmatches {
  pub artist: Vec<Artist>,
}

#[derive(Debug, Deserialize)]
pub struct TopAlbums {
  pub topalbums: Albums<u32>,
}

#[derive(Debug, Deserialize)]
pub struct TopTracks {
  pub toptracks: Tracks<u32, String, String>,
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

  pub async fn get_info(&self, name: &str) -> Result<ArtistResponse, surf::Error> {
    let res = self
      .client
      .get(format!(
        "?method=artist.getinfo&artist={}&api_key={}&format=json",
        name, self.api_key
      ))
      .recv_json::<ArtistResponse>()
      .await?;
    Ok(res)
  }

  pub async fn get_similar(&self, name: &str) -> Result<SimilarArtists, surf::Error> {
    let res = self
      .client
      .get(format!(
        "?method=artist.getsimilar&artist={}&api_key={}&format=json",
        name, self.api_key
      ))
      .recv_json::<SimilarArtists>()
      .await?;
    Ok(res)
  }

  pub async fn get_tags(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_top_albums(&self, artist: &str) -> Result<TopAlbums, surf::Error> {
    let res = self
      .client
      .get(format!(
        "?method=artist.gettopalbums&artist={}&api_key={}&format=json",
        artist, self.api_key
      ))
      .recv_json::<TopAlbums>()
      .await?;
    Ok(res)
  }

  pub async fn get_top_tags(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn get_top_tracks(&self, artist: &str) -> Result<TopTracks, surf::Error> {
    let res = self
      .client
      .get(format!(
        "?method=artist.gettoptracks&artist={}&api_key={}&format=json",
        artist, self.api_key
      ))
      .recv_json::<TopTracks>()
      .await?;
    Ok(res)
  }

  pub async fn remove_tag(&self) -> Result<(), surf::Error> {
    Ok(())
  }

  pub async fn search(&self, name: &str) -> Result<SearchResults, surf::Error> {
    let res = self
      .client
      .get(format!(
        "?method=artist.search&artist={}&api_key={}&format=json",
        name, self.api_key
      ))
      .recv_json::<SearchResults>()
      .await?;
    Ok(res)
  }
}
