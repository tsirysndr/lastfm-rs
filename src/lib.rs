use std::convert::TryInto;
use std::time::Duration;
use surf::{Client, Config, Url};

pub mod album;
pub mod artist;
pub mod library;
pub mod search;
pub mod tag;
pub mod track;

pub struct Lastfm {
  pub album: album::AlbumService,
  pub artist: artist::ArtistService,
  pub library: library::LibraryService,
  pub search: search::SearchService,
  pub tag: tag::TagService,
  pub track: track::TrackService,
}

const BASE_URL: &str = "http://ws.audioscrobbler.com/2.0/";

impl Lastfm {
  pub fn new(api_key: &str) -> Self {
    let client: Client = Config::new()
      .set_base_url(Url::parse(BASE_URL).unwrap())
      .set_timeout(Some(Duration::from_secs(5)))
      .try_into()
      .unwrap();
    Self {
      album: album::AlbumService::new(&client, api_key),
      artist: artist::ArtistService::new(&client, api_key),
      library: library::LibraryService::new(&client, api_key),
      search: search::SearchService::new(&client, api_key),
      tag: tag::TagService::new(&client, api_key),
      track: track::TrackService::new(&client, api_key),
    }
  }
}
