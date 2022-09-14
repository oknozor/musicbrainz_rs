use musicbrainz_rs::entity::annotation::*;
use musicbrainz_rs::Search;

#[test]
fn should_search_annotation() {
    let query = AnnotationSearchQuery::query_builder()
        .text("Warner Classics International")
        .build();

    let result = Annotation::search(query).execute().unwrap();

    assert!(result
        .entities
        .iter()
        .any(|annotation| annotation.name == "Warner Classics"));
}
