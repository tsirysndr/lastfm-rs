use lastfm_rs::Lastfm;

#[tokio::main]
async fn main() {
  let client = Lastfm::new("3721a0792cecc9d0981a1a08a2ed15d1");
  let album = client.album.get_info("Linkin Park", "Meteora").await;
  println!("{:#?}", album.unwrap());
}
