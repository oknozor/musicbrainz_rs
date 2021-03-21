extern crate musicbrainz_rs;
use musicbrainz_rs::model::series::Series;
use musicbrainz_rs::Fetch;
use std::{thread, time};

#[test]
fn should_get_serie_tags() {
    let breaks_loop_n_edits = Series::fetch()
        .id("0c66e70d-5f23-4579-8fe5-6bc0007428a2")
        .with_tags()
        .execute()
        .unwrap();

    assert!(breaks_loop_n_edits
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "breakbeat"));

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_serie_aliases() {
    let ultimate_breaks_and_beats = Series::fetch()
        .id("3e5979c8-5a78-4d0b-878a-0fb87853effe")
        .with_aliases()
        .execute()
        .unwrap();

    assert!(ultimate_breaks_and_beats.aliases.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_serie_genres() {
    let ultimate_breaks_and_beats = Series::fetch()
        .id("3e5979c8-5a78-4d0b-878a-0fb87853effe")
        .with_genres()
        .execute()
        .unwrap();

    assert!(ultimate_breaks_and_beats.genres.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_serie_annotation() {
    let record_store_day_2020 = Series::fetch()
        .id("c1071cec-48f1-4231-ac8e-8c64e15ec7cd")
        .with_annotations()
        .execute()
        .unwrap();

    assert!(record_store_day_2020.annotation.is_some());

    thread::sleep(time::Duration::from_secs(1));
}
