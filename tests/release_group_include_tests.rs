use musicbrainz_rs::model::release_group;
use musicbrainz_rs::model::release_group::*;
use musicbrainz_rs::QueryAble;
use std::{thread, time};

#[test]
fn should_get_release_group_artists() {
    let harvest = ReleaseGroup::fetch()
        .id("b25419cf-71bf-3a54-8cd4-2161c61056a0")
        .include(release_group::Include::Artists)
        .execute();

    assert!(harvest
        .unwrap()
        .artist_credit
        .unwrap()
        .iter()
        .any(|credit| credit.artist.name == "Neil Young"));

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_release_group_releases() {
    let harvest = ReleaseGroup::fetch()
        .id("b25419cf-71bf-3a54-8cd4-2161c61056a0")
        .include(release_group::Include::Releases)
        .execute();

    assert!(harvest
        .unwrap()
        .releases
        .unwrap()
        .iter()
        .any(|release| release.title == "Harvest" && release.country == Some("CA".to_string())));

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_release_group_tags() {
    let in_utero = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .include(release_group::Include::Tags)
        .execute()
        .unwrap();

    assert!(in_utero
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "rock_grunge"));
    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_release_group_aliases() {
    let in_utero = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .include(release_group::Include::Aliases)
        .execute()
        .unwrap();

    assert!(in_utero.aliases.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_release_group_rating() {
    let in_utero = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .include(release_group::Include::Rating)
        .execute()
        .unwrap();

    assert!(in_utero.rating.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_release_group_genres() {
    let in_utero = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .include(release_group::Include::Genres)
        .execute()
        .unwrap();

    assert!(in_utero.genres.is_some());

    thread::sleep(time::Duration::from_secs(1));
}
