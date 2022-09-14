use musicbrainz_rs::entity::work::Work;
use musicbrainz_rs::prelude::*;

#[test]
fn should_get_work_tags() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_tags()
        .execute()
        .unwrap();

    assert!(hotel_california
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "ripped off"));
}

#[test]
fn should_get_work_aliases() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_aliases()
        .execute()
        .unwrap();

    assert!(hotel_california.aliases.is_some());
}

#[test]
fn should_get_work_rating() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_ratings()
        .execute()
        .unwrap();

    assert!(hotel_california.rating.is_some());
}

#[test]
fn should_get_work_genres() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_genres()
        .execute()
        .unwrap();

    assert!(hotel_california.genres.is_some());
}

#[test]
fn should_get_work_annotation() {
    let vater_unser_im_himmelreich = Work::fetch()
        .id("85ab2b66-cf0b-47e9-beee-34c64a5ddea1")
        .with_annotations()
        .execute()
        .unwrap();

    assert!(vater_unser_im_himmelreich.annotation.is_some());
}

#[test]
fn should_get_work_artist_relations() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_artist_relations()
        .execute()
        .unwrap();

    let relations = hotel_california.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "writer"));
}

#[test]
fn should_get_work_label_relations() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_label_relations()
        .execute()
        .unwrap();

    let relations = hotel_california.relations.unwrap();

    assert!(relations
        .iter()
        .any(|rel| rel.relation_type == "publishing"));
}

#[test]
fn should_get_work_recording_relations() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_recording_relations()
        .execute()
        .unwrap();

    let relations = hotel_california.relations.unwrap();

    assert!(relations
        .iter()
        .any(|rel| rel.relation_type == "performance"));
}

#[test]
fn should_get_work_url_relations() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_url_relations()
        .execute()
        .unwrap();

    let relations = hotel_california.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "wikidata"));
}

#[test]
fn should_get_work_work_relations() {
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .with_work_relations()
        .execute()
        .unwrap();

    let relations = hotel_california.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "based on"));
}
