use musicbrainz_rs::entity::instrument::Instrument;
use musicbrainz_rs::prelude::*;

#[test]
fn should_get_instrument_tags() {
    let guitar = Instrument::fetch()
        .id("63021302-86cd-4aee-80df-2270d54f4978")
        .with_tags()
        .execute()
        .unwrap();

    assert!(guitar.tags.unwrap().iter().any(|tag| tag.name == "wood"));
}

#[test]
fn should_get_instrument_aliases() {
    let guitar = Instrument::fetch()
        .id("63021302-86cd-4aee-80df-2270d54f4978")
        .with_aliases()
        .execute()
        .unwrap();

    assert!(guitar
        .aliases
        .unwrap()
        .iter()
        .any(|alias| alias.name == "guitarras"));
}

#[test]
fn should_get_instrument_genres() {
    let guitar = Instrument::fetch()
        .id("63021302-86cd-4aee-80df-2270d54f4978")
        .with_genres()
        .execute()
        .unwrap();

    assert!(guitar.genres.is_some());
}

#[test]
fn should_get_instrument_annotation() {
    let gusli = Instrument::fetch()
        .id("bb08cebd-ff6c-49e8-8f8f-914cc2d68c27")
        .with_annotations()
        .execute()
        .unwrap();

    assert!(gusli.annotation.is_some());
}

#[test]
fn should_get_instrument_url_relations() {
    let guitar = Instrument::fetch()
        .id("63021302-86cd-4aee-80df-2270d54f4978")
        .with_url_relations()
        .execute()
        .unwrap();

    let relations = guitar.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "wikidata"));
}
