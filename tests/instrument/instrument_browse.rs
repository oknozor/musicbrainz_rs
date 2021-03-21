extern crate musicbrainz_rs;

use self::musicbrainz_rs::model::instrument::Instrument;
use self::musicbrainz_rs::Browse;
use std::thread;
use std::time::Duration;

// TODO: find non empty result
#[test]
fn browse_instrument_by_collection() {
    let instruments = Instrument::browse()
        .by_collection("6b6dc74a-f779-491a-a0eb-7c1d1ed56fe0")
        .execute();

    assert!(instruments.is_ok());
    let _instruments = instruments.unwrap();

    thread::sleep(Duration::from_secs(1));
}
