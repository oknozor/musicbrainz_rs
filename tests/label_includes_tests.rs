extern crate musicbrainz_rs;
use musicbrainz_rs::model::label;
use musicbrainz_rs::model::label::Label;
use musicbrainz_rs::QueryAble;

#[test]
fn should_get_label_releases() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .include(label::Include::Releases)
        .execute();

    let releases = ninja_tune.unwrap().releases;

    assert!(releases.is_some());
    assert!(!releases.unwrap().is_empty());
}
