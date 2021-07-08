extern crate musicbrainz_rs;

use musicbrainz_rs::entity::area::*;
use musicbrainz_rs::prelude::*;

#[test]
fn should_get_area_tags() {
    let aberdeen = Area::fetch()
        .id("a640b45c-c173-49b1-8030-973603e895b5")
        .with_tags()
        .execute()
        .unwrap();

    assert!(aberdeen.tags.is_some());
}

#[test]
fn should_get_area_aliases() {
    let aberdeen = Area::fetch()
        .id("a640b45c-c173-49b1-8030-973603e895b5")
        .with_aliases()
        .execute()
        .unwrap();

    assert!(aberdeen.aliases.is_some());
}

#[test]
fn should_get_area_genres() {
    let aberdeen = Area::fetch()
        .id("a640b45c-c173-49b1-8030-973603e895b5")
        .with_genres()
        .execute()
        .unwrap();

    assert!(aberdeen.genres.is_some());
}

#[test]
fn should_get_area_annotation() {
    let london = Area::fetch()
        .id("f03d09b3-39dc-4083-afd6-159e3f0d462f")
        .with_annotations()
        .execute()
        .unwrap();

    assert!(london.annotation.is_some());
}

#[test]
fn should_get_area_area_relations() {
    let london = Area::fetch()
        .id("f03d09b3-39dc-4083-afd6-159e3f0d462f")
        .with_area_relations()
        .execute()
        .unwrap();

    let relations = london.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "part of"));
}

#[test]
fn should_get_area_event_relations() {
    let london = Area::fetch()
        .id("f03d09b3-39dc-4083-afd6-159e3f0d462f")
        .with_event_relations()
        .execute()
        .unwrap();

    let relations = london.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "held in"));
}

#[test]
fn should_get_area_recording_relations() {
    let london = Area::fetch()
        .id("f03d09b3-39dc-4083-afd6-159e3f0d462f")
        .with_recording_relations()
        .execute()
        .unwrap();

    let relations = london.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "arranged in"));
}

#[test]
fn should_get_area_release_relations() {
    let london = Area::fetch()
        .id("f03d09b3-39dc-4083-afd6-159e3f0d462f")
        .with_release_relations()
        .execute()
        .unwrap();

    let relations = london.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "engineered in"));
}

#[test]
fn should_get_area_url_relations() {
    let london = Area::fetch()
        .id("f03d09b3-39dc-4083-afd6-159e3f0d462f")
        .with_url_relations()
        .execute()
        .unwrap();

    let relations = london.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "geonames"));
}

#[test]
fn should_get_area_work_relations() {
    let london = Area::fetch()
        .id("f03d09b3-39dc-4083-afd6-159e3f0d462f")
        .with_work_relations()
        .execute()
        .unwrap();

    let relations = london.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "anthem"));
}
