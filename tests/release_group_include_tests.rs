use musicbrainz_rs::model::release_group;
use musicbrainz_rs::model::release_group::*;
use musicbrainz_rs::QueryAble;

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
        .any(|credit| credit.artist.name == "Neil Young"))
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
        .any(|release| release.title == "Harvest" && release.country == Some("CA".to_string())))
}
