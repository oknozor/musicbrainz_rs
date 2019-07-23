extern crate musicbrainz_rs;
use musicbrainz_rs::model::work;
use musicbrainz_rs::model::work::Work;
use musicbrainz_rs::QueryAble;
use std::{thread, time};

#[test]
fn should_get_work_tags() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .include(work::Include::Tags)
        .execute()
        .unwrap();

    assert!(hotel_california
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "ripped off"));

    thread::sleep(time::Duration::from_secs(1));
}
