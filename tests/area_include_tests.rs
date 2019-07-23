extern crate musicbrainz_rs;

use musicbrainz_rs::model::area;
use musicbrainz_rs::model::area::*;
use musicbrainz_rs::QueryAble;
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
