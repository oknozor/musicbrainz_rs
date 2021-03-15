extern crate musicbrainz_rs;
use musicbrainz_rs::model::work;
use musicbrainz_rs::model::work::Work;
use musicbrainz_rs::Fetch;
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

#[test]
fn should_get_work_aliases() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .include(work::Include::Aliases)
        .execute()
        .unwrap();

    assert!(hotel_california.aliases.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_work_rating() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .include(work::Include::Rating)
        .execute()
        .unwrap();

    assert!(hotel_california.rating.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_work_genres() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .include(work::Include::Genres)
        .execute()
        .unwrap();

    assert!(hotel_california.genres.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_work_annotation() {
    let vater_unser_im_himmelreich = Work::fetch()
        .id("85ab2b66-cf0b-47e9-beee-34c64a5ddea1")
        .include(work::Include::Annotation)
        .execute()
        .unwrap();

    assert!(vater_unser_im_himmelreich.annotation.is_some());

    thread::sleep(time::Duration::from_secs(1));
}
