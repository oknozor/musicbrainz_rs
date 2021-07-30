use musicbrainz_rs::entity::series::Series;
use musicbrainz_rs::prelude::*;

// TODO: find non empty result
#[test]
fn browse_series_by_collection() {
    let series_in_collection = Series::browse()
        .by_collection("91565a03-bce8-47e9-ab70-e4d4e1684d7f")
        .execute();

    assert!(series_in_collection.is_ok());
}
