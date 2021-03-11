extern crate chrono;
extern crate musicbrainz_rs;

use musicbrainz_rs::model::work;
use musicbrainz_rs::model::work::*;
use musicbrainz_rs::Browse;
use std::{thread, time};

#[test]
fn should_browse_work_by_artist() {
    let work_by_svinkels = Work::browse()
        .by(work::Browse::Artist, "770d490e-c89b-4775-8508-aca7c75142cd")
        .execute();

    assert!(work_by_svinkels.is_ok());

    let work_by_svinkels = work_by_svinkels.unwrap();

    assert!(work_by_svinkels.count > 1);
    assert_eq!(work_by_svinkels.offset, 0);
    assert!(!work_by_svinkels.entities.is_empty());

    thread::sleep(time::Duration::from_secs(1));
}
