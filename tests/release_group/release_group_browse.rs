extern crate chrono;
extern crate musicbrainz_rs;

use musicbrainz_rs::model::release_group;
use musicbrainz_rs::model::release_group::*;
use musicbrainz_rs::Browse;
use std::{thread, time};

#[test]
fn should_browse_release_group_by_artist() {
    let release_groups_by_svinkels = ReleaseGroup::browse()
        .by(
            release_group::Browse::Artist,
            "770d490e-c89b-4775-8508-aca7c75142cd",
        )
        .execute();

    assert!(release_groups_by_svinkels.is_ok());

    let release_groups_by_svinkels = release_groups_by_svinkels.unwrap();

    assert!(release_groups_by_svinkels.count > 1);
    assert_eq!(release_groups_by_svinkels.offset, 0);
    assert!(!release_groups_by_svinkels.entities.is_empty());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_browse_release_group_by_release() {
    let release_groups_of_we_want_miles = ReleaseGroup::browse()
        .by(
            release_group::Browse::Release,
            "84928705-cb99-4c49-a820-73cc460c0bd2",
        )
        .execute();

    assert!(release_groups_of_we_want_miles.is_ok());

    let release_groups_of_we_want_miles = release_groups_of_we_want_miles.unwrap();

    assert!(release_groups_of_we_want_miles.count > 0);
    assert_eq!(release_groups_of_we_want_miles.offset, 0);
    assert!(!release_groups_of_we_want_miles.entities.is_empty());

    thread::sleep(time::Duration::from_secs(1));
}
