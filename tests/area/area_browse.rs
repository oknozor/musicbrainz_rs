extern crate musicbrainz_rs;

use self::musicbrainz_rs::Browse;
use musicbrainz_rs::model::area::*;
use std::thread;
use std::time::Duration;

// TODO: find non empty result
#[test]
fn browse_area_by_collection() {
    let areas = Area::browse()
        .by_collection("6b6dc74a-f779-491a-a0eb-7c1d1ed56fe0")
        .execute();

    assert!(areas.is_ok());
    thread::sleep(Duration::from_secs(1));
}
