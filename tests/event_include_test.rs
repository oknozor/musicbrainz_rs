extern crate musicbrainz_rs;
use musicbrainz_rs::model::event;
use musicbrainz_rs::model::event::Event;
use musicbrainz_rs::Fetch;
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

#[test]
fn should_get_event_aliases() {
    let dour_festival_1989 = Event::fetch()
        .id("73df2f48-383b-4930-bad3-05ba938be578")
        .include(event::Include::Aliases)
        .execute()
        .unwrap();

    assert!(dour_festival_1989.aliases.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_event_rating() {
    let dour_festival_1989 = Event::fetch()
        .id("73df2f48-383b-4930-bad3-05ba938be578")
        .include(event::Include::Rating)
        .execute()
        .unwrap();

    assert!(dour_festival_1989.rating.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_event_genres() {
    let dour_festival_1989 = Event::fetch()
        .id("73df2f48-383b-4930-bad3-05ba938be578")
        .include(event::Include::Genres)
        .execute()
        .unwrap();

    assert!(dour_festival_1989.genres.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

// TODO: find an actual resource with annotation

// #[test]
// fn should_get_event_annotation() {
//     let dour_festival_1989 = Event::fetch()
//         .id("73df2f48-383b-4930-bad3-05ba938be578")
//         .include(event::Include::Annotation)
//         .execute()
//         .unwrap();

//     assert!(dour_festival_1989.annotation.is_some());

//     thread::sleep(time::Duration::from_secs(1));
// }
