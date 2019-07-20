extern crate chrono;
extern crate musicbrainz_rs;

use musicbrainz_rs::model::artist;
use musicbrainz_rs::model::artist::*;
use musicbrainz_rs::QueryAble;

#[test]
fn should_get_artist_releases() {
    let john_lee_hooker = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .include(artist::Include::Releases)
        .execute()
        .unwrap();

    let releases = john_lee_hooker.releases;

    assert!(releases.is_some());
    assert!(!releases.unwrap().is_empty());
}

#[test]
fn should_get_artist_works() {
    let john_lee_hooker = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .include(artist::Include::Works)
        .execute()
        .unwrap();

    let works = john_lee_hooker.works;

    assert!(works.is_some());
    assert!(!works.unwrap().is_empty());
}

#[test]
fn should_get_artist_release_groups() {
    let john_lee_hooker = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .include(artist::Include::ReleaseGroups)
        .execute()
        .unwrap();

    let release_groups = john_lee_hooker.release_groups;

    assert!(release_groups.is_some());
    assert!(!release_groups.unwrap().is_empty());
}

#[test]
fn should_get_artist_recordings() {
    let john_lee_hooker = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .include(artist::Include::Recordings)
        .execute()
        .unwrap();

    let recordings = john_lee_hooker.recordings;

    assert!(recordings.is_some());
    assert!(!recordings.unwrap().is_empty());
}

#[test]
fn should_get_artist_aliases() {
    let john_lee_hooker = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .include(artist::Include::Aliases)
        .execute()
        .unwrap();

    let aliases = john_lee_hooker.aliases;

    assert!(aliases.is_some());
    assert!(!aliases.unwrap().is_empty());
}

#[test]
fn should_get_artist_artist_relations() {
    let john_lee_hooker = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .include(artist::Include::ArtistRelations)
        .execute()
        .unwrap();

    let relations = john_lee_hooker.relations;

    assert!(relations.is_some());
    assert!(!relations.unwrap().is_empty());
}
