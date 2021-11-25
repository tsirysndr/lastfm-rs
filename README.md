<h1 align="left">lastfm-rs</h1>
<p>
  <a href="#" target="_blank">
    <img alt="License: BSD" src="https://img.shields.io/badge/License-BSD-green.svg" />
  </a>
  <a href="https://github.com/tsirysndr/lastfm-rs/commits/master">
    <img src="https://img.shields.io/github/last-commit/tsirysndr/lastfm-rs.svg" target="_blank" />
  </a>
  <a href="https://twitter.com/tsiry_sndr" target="_blank">
    <img alt="Twitter: tsiry_sndr" src="https://img.shields.io/twitter/follow/tsiry_sndr.svg?style=social" />
  </a>
</p>

> lastfm-rs is a Rust client library for accessing the [Last.fm API](https://www.last.fm/api)

## Install

```toml
[dependencies]
lastfm-rs = { git = "https://github.com/tsirysndr/lastfm-rs" }
```

## Usage

Construct a new Lastfm client, then use the various services on the client to access different parts of the Last.fm API. For example:

```rust
use lastfm_rs::Lastfm;

#[tokio::main]
async fn main() {
  let client = Lastfm::new("3721a0792cecc9d0981a1a08a2ed15d1");
  let album = client.album.get_info("Linkin Park", "Meteora").await;
  let album_results = client.album.search("Meteora").await;
  let artist = client.artist.get_info("Linkin Park").await;
  let artist_results = client.artist.search("Linkin Park").await;
  let track = client.track.get_info("Linkin Park", "papercut").await;
  let similar_artists = client.artist.get_similar("Linkin Park").await;
  let top_albums = client.artist.get_top_albums("Linkin Park").await;

  println!("{:#?}", album.unwrap());
  println!("{:#?}", album_results.unwrap());
  println!("{:#?}", artist.unwrap());
  println!("{:#?}", artist_results.unwrap());
  println!("{:#?}", track.unwrap());
  println!("{:#?}", similar_artists.unwrap());
  println!("{:#?}", top_albums.unwrap());
}
```

## Author

👤 **Tsiry sandratraina**

* Website: https://tsiry-sandratraina.netlify.com
* Twitter: [@tsiry_sndr](https://twitter.com/tsiry_sndr)
* Github: [@tsirysndr](https://github.com/tsirysndr)

## Show your support

Give a ⭐️ if this project helped you!
