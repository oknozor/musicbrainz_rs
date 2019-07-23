extern crate musicbrainz_rs;
use musicbrainz_rs::model::recording;
use musicbrainz_rs::model::recording::*;
use musicbrainz_rs::QueryAble;
use std::{thread, time};

#[test]
fn should_get_recording_artists() {
    let association_de_gens_normal = Recording::fetch()
        .id("f5f10cee-5d84-41d0-805d-3503872c151d")
        .include(recording::Include::Artists)
        .execute();

    let artist_credit = association_de_gens_normal.unwrap().artist_credit.unwrap();

    assert!(artist_credit.iter().any(|credit| credit.name == "TTC"));
    assert!(artist_credit.iter().any(|credit| credit.name == "Svinkels"));

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_recording_releases() {
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .include(recording::Include::Releases)
        .execute();

    let releases = you_talk_too_much.unwrap().releases;

    assert!(releases
        .unwrap()
        .iter()
        .any(|release| release.title == "Hooker ’n Heat"));

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_recording_aliases() {
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .include(recording::Include::Aliases)
        .execute();

    let aliases = you_talk_too_much.unwrap().aliases;

    assert!(aliases.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_recording_tags() {
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .include(recording::Include::Tags)
        .execute()
        .unwrap();

    assert!(you_talk_too_much.tags.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)

    thread::sleep(time::Duration::from_secs(1));
}
