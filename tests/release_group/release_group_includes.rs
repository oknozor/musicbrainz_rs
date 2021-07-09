use musicbrainz_rs::entity::release_group::*;
use musicbrainz_rs::prelude::*;

#[test]
fn should_get_release_group_artists() {
    let harvest = ReleaseGroup::fetch()
        .id("b25419cf-71bf-3a54-8cd4-2161c61056a0")
        .with_artists()
        .execute();

    assert!(harvest
        .unwrap()
        .artist_credit
        .unwrap()
        .iter()
        .any(|credit| credit.artist.name == "Neil Young"));
}

#[test]
fn should_get_release_group_releases() {
    let harvest = ReleaseGroup::fetch()
        .id("b25419cf-71bf-3a54-8cd4-2161c61056a0")
        .with_releases()
        .execute();

    assert!(harvest
        .unwrap()
        .releases
        .unwrap()
        .iter()
        .any(|release| release.title == "Harvest" && release.country == Some("CA".to_string())));
}

#[test]
fn should_get_release_group_tags() {
    let in_utero = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .with_tags()
        .execute()
        .unwrap();

    assert!(in_utero
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "rock_grunge"));
}

#[test]
fn should_get_release_group_aliases() {
    let in_utero = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .with_aliases()
        .execute()
        .unwrap();

    assert!(in_utero.aliases.is_some());
}

#[test]
fn should_get_release_group_rating() {
    let in_utero = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .with_ratings()
        .execute()
        .unwrap();

    assert!(in_utero.rating.is_some());
}

#[test]
fn should_get_release_group_genres() {
    let in_utero = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .with_genres()
        .execute()
        .unwrap();

    assert!(in_utero.genres.is_some());
}

#[test]
fn should_get_release_group_annotation() {
    let minnesoda = ReleaseGroup::fetch()
        .id("100d2ce1-8ba4-43eb-afbe-93ca21867e16")
        .with_annotations()
        .execute()
        .unwrap();

    assert!(minnesoda.annotation.is_some());
}

// FIXME: This is currently panicking with:
// thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value:
// Error(Json(Error("no variant of enum RelationContent found in flattened data", line: 1, column: 576)))'
// #[test]
// fn should_get_release_group_release_group_relations() {
//     let in_utero = ReleaseGroup::fetch()
//         .id("2a0981fb-9593-3019-864b-ce934d97a16e")
//         .with_release_group_relations()
//         .execute()
//         .unwrap();

//     let relations = in_utero.relations.unwrap();

//     assert!(relations.iter().any(|rel| rel.relation_type == "cover"));
// }

#[test]
fn should_get_release_group_series_relations() {
    let in_utero = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .with_series_relations()
        .execute()
        .unwrap();

    let relations = in_utero.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "part of"));
}

#[test]
fn should_get_release_group_url_relations() {
    let in_utero = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .with_url_relations()
        .execute()
        .unwrap();

    let relations = in_utero.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "wikidata"));
}
