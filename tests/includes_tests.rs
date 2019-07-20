
extern crate musicbrainz_rs; 
extern crate chrono; 

use musicbrainz_rs::model::artist::*;
use musicbrainz_rs::QueryAble;
use musicbrainz_rs::Include;

#[test]
fn should_get_artist_releases() {

    let john_lee_hooker = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .include(Include::Releases)
        .execute()
        .unwrap();

    let releases = john_lee_hooker.releases;

    assert!(releases.is_some());
    assert!(!releases.unwrap().is_empty());
}

#[test]
fn should_get_artist_works() {

    let john_lee_hooker = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .include(Include::Works)
        .execute()
        .unwrap();

    let works = john_lee_hooker.works;

    assert!(works.is_some());
    assert!(!works.unwrap().is_empty());
}
