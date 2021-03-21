extern crate musicbrainz_rs;
use self::musicbrainz_rs::model::label::Label;
use musicbrainz_rs::Fetch;
use std::{thread, time};

#[test]
fn should_get_label_releases() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .with_releases()
        .execute();

    let releases = ninja_tune.unwrap().releases;

    assert!(releases
        .unwrap()
        .iter()
        .any(|release| release.title == "The Final Corporate Colonization of the Unconscious"));

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_label_aliases() {
    let motown = Label::fetch()
        .id("8e479e57-ef44-490c-b75d-cd28df89bf1b")
        .with_aliases()
        .execute();

    let aliases = motown.unwrap().aliases;

    assert!(aliases.unwrap().iter().any(|alias| alias.name == "Motown"));

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_label_tags() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .with_tags()
        .execute()
        .unwrap();

    assert!(ninja_tune
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "independent"));

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_label_rating() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .with_ratings()
        .execute()
        .unwrap();

    assert!(ninja_tune.rating.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_label_genres() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .with_genres()
        .execute()
        .unwrap();

    assert!(ninja_tune.genres.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_label_annotation() {
    let tokuma_japan_communications = Label::fetch()
        .id("040439f9-578b-45b6-b07b-d6c97e544859")
        .with_annotations()
        .execute()
        .unwrap();

    assert!(tokuma_japan_communications.annotation.is_some());

    thread::sleep(time::Duration::from_secs(1));
}
