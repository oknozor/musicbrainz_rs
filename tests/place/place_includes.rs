extern crate musicbrainz_rs;
use musicbrainz_rs::entity::place::*;
use musicbrainz_rs::prelude::*;
use std::{thread, time};

#[test]
fn should_get_place_aliases() {
    let blue_note = Place::fetch()
        .id("327c29c6-da63-4dc9-a117-1917ee691ce4")
        .with_aliases()
        .execute()
        .unwrap();

    assert!(blue_note.aliases.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_place_tags() {
    let olympia = Place::fetch()
        .id("36678fc4-2fee-46be-b084-4c4e2314ce71")
        .with_tags()
        .execute()
        .unwrap();

    assert!(olympia.tags.is_some());

    thread::sleep(time::Duration::from_secs(1));
}
#[test]
fn should_get_place_genres() {
    let olympia = Place::fetch()
        .id("36678fc4-2fee-46be-b084-4c4e2314ce71")
        .with_genres()
        .execute()
        .unwrap();

    assert!(olympia.genres.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_place_annotation() {
    let osaka_kosei_nenkin_kaikan = Place::fetch()
        .id("751f998a-60ca-4d48-954f-b101d59ad89a")
        .with_annotations()
        .execute()
        .unwrap();

    assert!(osaka_kosei_nenkin_kaikan.annotation.is_some());

    thread::sleep(time::Duration::from_secs(1));
}
