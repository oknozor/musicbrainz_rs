use musicbrainz_rs::entity::artist::{Artist, ArtistSearchQuery};
use musicbrainz_rs::prelude::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    musicbrainz_rs::config::set_user_agent("my_awesome_app/1.0");

    let query = ArtistSearchQuery::query_builder()
        .artist("Miles Davis")
        .and()
        .country("US")
        .build();

    let query_result = Artist::search(query).execute().await.unwrap();
    let query_result: Vec<String> = query_result
        .entities
        .iter()
        .map(|artist| artist.name.clone())
        .collect();

    assert!(query_result.contains(&"Miles Davis".to_string()));
    assert!(query_result.contains(&"Miles Davis Quintet".to_string()));
}
