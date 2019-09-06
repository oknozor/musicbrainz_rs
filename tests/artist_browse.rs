extern crate chrono;
extern crate musicbrainz_rs;

use musicbrainz_rs::model::artist;
use musicbrainz_rs::model::artist::*;
use musicbrainz_rs::Fetch;
use std::{thread, time};

// #[test]
// fn should_browse_artist() {
//     let radiohead = Artist::browse(
//             Browse::Recording,
//             "6e335887-60ba-38f0-95af-fae7774336bf",
//         ).execute();

//     assert!(radiohead.name == "Radiohead");

//     thread::sleep(time::Duration::from_secs(1));
// }
