extern crate musicbrainz_rs;

use musicbrainz_rs::entity::area::*;
use musicbrainz_rs::prelude::*;
use std::{thread, time};

#[test]
fn should_get_area_tags() {
    let aberdeen = Area::fetch()
        .id("a640b45c-c173-49b1-8030-973603e895b5")
        .with_tags()
        .execute()
        .unwrap();

    assert!(aberdeen.tags.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_area_aliases() {
    let aberdeen = Area::fetch()
        .id("a640b45c-c173-49b1-8030-973603e895b5")
        .with_aliases()
        .execute()
        .unwrap();

    assert!(aberdeen.aliases.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_area_genres() {
    let aberdeen = Area::fetch()
        .id("a640b45c-c173-49b1-8030-973603e895b5")
        .with_genres()
        .execute()
        .unwrap();

    assert!(aberdeen.genres.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_area_annotation() {
    let london = Area::fetch()
        .id("f03d09b3-39dc-4083-afd6-159e3f0d462f")
        .with_annotations()
        .execute()
        .unwrap();

    assert!(london.annotation.is_some());

    thread::sleep(time::Duration::from_secs(1));
}
