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
  let top_tracks = client.artist.get_top_tracks("Linkin Park").await;

  println!("{:#?}", album.unwrap());
  println!("{:#?}", album_results.unwrap());
  println!("{:#?}", artist.unwrap());
  println!("{:#?}", artist_results.unwrap());
  println!("{:#?}", track.unwrap());
  println!("{:#?}", similar_artists.unwrap());
  println!("{:#?}", top_albums.unwrap());
  println!("{:#?}", top_tracks.unwrap());
}
