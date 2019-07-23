extern crate musicbrainz_rs;
use musicbrainz_rs::model::series;
use musicbrainz_rs::model::series::Series;
use musicbrainz_rs::QueryAble;
use std::{thread, time};

#[test]
fn should_get_serie_tags() {
    let ultimate_breaks_and_beats = Series::fetch()
        .id("3e5979c8-5a78-4d0b-878a-0fb87853effe")
        .include(series::Include::Tags)
        .execute()
        .unwrap();

    assert!(ultimate_breaks_and_beats
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "beakbeat"));

    thread::sleep(time::Duration::from_secs(1));
}
