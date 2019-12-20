extern crate musicbrainz_rs;

use musicbrainz_rs::model::artist::*;
use musicbrainz_rs::Search;
use lucene_query_builder::QueryBuilder;
use std::{thread, time};

#[test]
fn should_search_artist() {

    let query = Artist::query_builder()
        .name("Nirvana")
        .and()
        .artist_type("Group")
        .build();

    let result = Artist::search(query).execute().unwrap();

    assert!(!result.entities.is_empty());

    thread::sleep(time::Duration::from_secs(1));
}
