extern crate musicbrainz_rs;

use musicbrainz_rs::entity::release::*;
use musicbrainz_rs::FetchCoverart;
use std::{thread, time};

#[test]
fn should_get_release_coverart() {
    let in_utero_coverart = Release::fetch_coverart()
        .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
        .execute()
        .expect("Unable to get cover art");

    assert_eq!(in_utero_coverart.images[0].front, true);
    assert_eq!(in_utero_coverart.images[0].back, false);

    thread::sleep(time::Duration::from_secs(1));
}
