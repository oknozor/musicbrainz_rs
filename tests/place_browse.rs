extern crate chrono;
extern crate musicbrainz_rs;

use musicbrainz_rs::model::place;
use musicbrainz_rs::model::place::*;
use musicbrainz_rs::Browse;
use std::{thread, time};

#[test]
fn should_browse_place_by_area() {
    let places_in_paris = Place::browse()
        .by(place::Browse::Area, "dc10c22b-e510-4006-8b7f-fecb4f36436e")
        .execute();

    println!("{:?}", places_in_paris);
    assert!(places_in_paris.is_ok());

    let places_in_paris = places_in_paris.unwrap();

    assert!(places_in_paris.count > 1);
    assert_eq!(places_in_paris.offset, 0);
    assert!(!places_in_paris.entities.is_empty());

    thread::sleep(time::Duration::from_secs(1));
}
