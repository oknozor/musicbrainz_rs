extern crate musicbrainz_rs;

use musicbrainz_rs::model::area;
use musicbrainz_rs::model::area::*;
use musicbrainz_rs::Fetch;
use std::{thread, time};

#[test]
fn should_get_area_tags() {
    let aberdeen = Area::fetch()
        .id("a640b45c-c173-49b1-8030-973603e895b5")
        .include(area::Include::Tags)
        .execute()
        .unwrap();

    assert!(aberdeen.tags.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

// TODO : waiting for https://github.com/metabrainz/musicbrainz-server/pull/1223 to be released
// #[test]
fn should_get_area_aliases() {
    let aberdeen = Area::fetch()
        .id("a640b45c-c173-49b1-8030-973603e895b5")
        .include(area::Include::Aliases)
        .execute()
        .unwrap();

    assert!(aberdeen.aliases.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_area_genres() {
    let aberdeen = Area::fetch()
        .id("a640b45c-c173-49b1-8030-973603e895b5")
        .include(area::Include::Genres)
        .execute()
        .unwrap();

    assert!(aberdeen.genres.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_area_annotation() {
    let london = Area::fetch()
        .id("f03d09b3-39dc-4083-afd6-159e3f0d462f")
        .include(area::Include::Annotation)
        .execute()
        .unwrap();

    assert!(london.annotation.is_some());

    thread::sleep(time::Duration::from_secs(1));
}
