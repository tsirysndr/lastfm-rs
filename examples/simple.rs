use lastfm_rs::Lastfm;

#[tokio::main]
async fn main() {
  let client = Lastfm::new("3721a0792cecc9d0981a1a08a2ed15d1");
  let album = client.album.get_info("Linkin Park", "Meteora").await;
  let album_results = client.album.search("Meteora").await;
  let artist = client.artist.get_info("Linkin Park").await;
  let artist_results = client.artist.search("Linkin Park").await;

  println!("{:#?}", album.unwrap());
  println!("{:#?}", album_results.unwrap());
  println!("{:#?}", artist.unwrap());
  println!("{:#?}", artist_results.unwrap());
}
