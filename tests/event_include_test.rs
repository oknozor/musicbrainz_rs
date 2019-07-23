extern crate musicbrainz_rs;
use musicbrainz_rs::model::event;
use musicbrainz_rs::model::event::Event;
use musicbrainz_rs::QueryAble;
use std::{thread, time};

#[test]
fn should_get_event_tags() {
    let dour_festival_1989 = Event::fetch()
        .id("73df2f48-383b-4930-bad3-05ba938be578")
        .include(event::Include::Tags)
        .execute()
        .unwrap();

    assert!(dour_festival_1989.tags.is_some());

    thread::sleep(time::Duration::from_secs(1));
}
