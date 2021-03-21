extern crate chrono;
extern crate musicbrainz_rs;

use musicbrainz_rs::model::place::*;
use musicbrainz_rs::Browse;
use std::{thread, time};

#[test]
fn should_browse_place_by_area() {
    let places_in_paris = Place::browse()
        .by_area("dc10c22b-e510-4006-8b7f-fecb4f36436e")
        .execute();

    assert!(places_in_paris.is_ok());

    let places_in_paris = places_in_paris.unwrap();

    assert!(places_in_paris.count > 1);
    assert_eq!(places_in_paris.offset, 0);
    assert!(!places_in_paris.entities.is_empty());

    thread::sleep(time::Duration::from_secs(1));
}

// TODO: find non empty result
#[test]
fn should_browse_place_by_collection() {
    let places_in_collection = Place::browse()
        .by_collection("91565a03-bce8-47e9-ab70-e4d4e1684d7f")
        .execute();

    assert!(places_in_collection.is_ok());
    thread::sleep(time::Duration::from_secs(1));
}
