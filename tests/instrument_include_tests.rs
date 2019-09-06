extern crate musicbrainz_rs;
use musicbrainz_rs::model::instrument;
use musicbrainz_rs::model::instrument::Instrument;
use musicbrainz_rs::Fetch;
use std::{thread, time};

#[test]
fn should_get_instrument_tags() {
    let guitar = Instrument::fetch()
        .id("63021302-86cd-4aee-80df-2270d54f4978")
        .include(instrument::Include::Tags)
        .execute()
        .unwrap();

    assert!(guitar.tags.unwrap().iter().any(|tag| tag.name == "wood"));

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_instrument_aliases() {
    let guitar = Instrument::fetch()
        .id("63021302-86cd-4aee-80df-2270d54f4978")
        .include(instrument::Include::Aliases)
        .execute()
        .unwrap();

    assert!(guitar
        .aliases
        .unwrap()
        .iter()
        .any(|alias| alias.name == "guitarras"));

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_instrument_genres() {
    let guitar = Instrument::fetch()
        .id("63021302-86cd-4aee-80df-2270d54f4978")
        .include(instrument::Include::Genres)
        .execute()
        .unwrap();

    assert!(guitar.genres.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

// TODO: find an actual resource with annotation
// #[test]
// fn should_get_instrument_annotation() {
//     let guitar = Instrument::fetch()
//         .id("63021302-86cd-4aee-80df-2270d54f4978")
//         .include(instrument::Include::Annotation)
//         .execute()
//         .unwrap();

//     assert!(guitar.annotation.is_some());

//     thread::sleep(time::Duration::from_secs(1));
// }
