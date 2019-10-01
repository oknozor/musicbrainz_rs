extern crate chrono;
extern crate musicbrainz_rs;

use musicbrainz_rs::model::artist;
use musicbrainz_rs::model::artist::*;
use musicbrainz_rs::Browse;
use std::{thread, time};

#[test]
fn should_browse_artist_by_release_groups() {
    let artistss_on_in_rainbows_rg = Artist::browse()
        .by(
            artist::Browse::ReleaseGroup,
            "6e335887-60ba-38f0-95af-fae7774336bf",
        )
        .execute();

    assert!(artistss_on_in_rainbows_rg.is_ok());

    let artistss_on_in_rainbows_rg = artistss_on_in_rainbows_rg.unwrap();

    assert_eq!(artistss_on_in_rainbows_rg.count, 1);
    assert_eq!(artistss_on_in_rainbows_rg.offset, 0);
    assert!(!artistss_on_in_rainbows_rg.entities.is_empty());

    thread::sleep(time::Duration::from_secs(1));
}
