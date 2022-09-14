use musicbrainz_rs::entity::recording::*;
use musicbrainz_rs::prelude::*;

#[test]
fn should_get_recording_artists() {
    let association_de_gens_normal = Recording::fetch()
        .id("f5f10cee-5d84-41d0-805d-3503872c151d")
        .with_artists()
        .execute();

    let artist_credit = association_de_gens_normal.unwrap().artist_credit.unwrap();

    assert!(artist_credit.iter().any(|credit| credit.name == "TTC"));
    assert!(artist_credit.iter().any(|credit| credit.name == "Svinkels"));
}

#[test]
fn should_get_recording_releases() {
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .with_releases()
        .execute();

    let releases = you_talk_too_much.unwrap().releases;

    assert!(releases
        .unwrap()
        .iter()
        .any(|release| release.title == "Hooker ’n Heat"));
}

#[test]
fn should_get_recording_aliases() {
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .with_aliases()
        .execute();

    let aliases = you_talk_too_much.unwrap().aliases;

    assert!(aliases.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[test]
fn should_get_recording_tags() {
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .with_tags()
        .execute()
        .unwrap();

    assert!(you_talk_too_much.tags.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[test]
fn should_get_recording_rating() {
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .with_ratings()
        .execute()
        .unwrap();

    assert!(you_talk_too_much.rating.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[test]
fn should_get_recording_genres() {
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .with_genres()
        .execute()
        .unwrap();

    assert!(you_talk_too_much.genres.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[test]
fn should_get_recording_annotation() {
    let isolina = Recording::fetch()
        .id("2edf7653-2287-4408-8e7a-20e001a60847")
        .with_annotations()
        .execute()
        .unwrap();

    assert!(isolina.annotation.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[test]
fn should_get_recording_isrcs() {
    let senorita = Recording::fetch()
        .id("62f09fd2-144a-439a-96f9-ce93f05b48ae")
        .with_isrcs()
        .execute()
        .unwrap();

    assert!(senorita.isrcs.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[test]
fn should_get_recording_url_relations() {
    let senorita = Recording::fetch()
        .id("62f09fd2-144a-439a-96f9-ce93f05b48ae")
        .with_url_relations()
        .execute()
        .unwrap();

    let relations = senorita.relations.unwrap();

    assert!(relations
        .iter()
        .any(|rel| rel.relation_type == "free streaming")); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[test]
fn should_get_recording_work_relations() {
    let senorita = Recording::fetch()
        .id("62f09fd2-144a-439a-96f9-ce93f05b48ae")
        .with_work_relations()
        .execute()
        .unwrap();

    let relations = senorita.relations.unwrap();

    assert!(relations
        .iter()
        .any(|rel| rel.relation_type == "performance")); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[test]
fn should_get_recording_level_relations() {
    let polly = Recording::fetch()
        .id("af40d6b8-58e8-4ca5-9db8-d4fca0b899e2")
        .with_work_relations()
        .with_work_level_relations()
        .execute()
        .unwrap();

    let relations = polly.relations.unwrap();

    assert!(relations
        .iter()
        .any(|rel| rel.target_type.as_ref().unwrap() == "work"));
}
