extern crate musicbrainz_rs;

use musicbrainz_rs::model::label;
use musicbrainz_rs::model::label::*;
use musicbrainz_rs::Fetch;

fn main() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .include(label::Include::Tags)
        .include(label::Include::Rating)
        .execute()
        .unwrap();

    assert!(ninja_tune
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "independent"));

    assert!(ninja_tune.rating.is_some());
}
