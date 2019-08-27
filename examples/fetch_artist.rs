extern crate musicbrainz_rs;

use musicbrainz_rs::model::artist::*;
use musicbrainz_rs::QueryAble;

fn main() {
    let nirvana = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .execute();

    assert_eq!(nirvana.unwrap().name, "Nirvana".to_string());
}
