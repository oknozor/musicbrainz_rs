extern crate chrono;
extern crate musicbrainz_rs;

use musicbrainz_rs::model::release;
use musicbrainz_rs::model::release::*;
use musicbrainz_rs::Browse;
use std::{thread, time};

#[test]
fn should_browse_release_by_artist() {
    let recording_by_svinkels = Release::browse()
        .by(
            release::Browse::Artist,
            "770d490e-c89b-4775-8508-aca7c75142cd",
        )
        .execute();

    assert!(recording_by_svinkels.is_ok());

    let recording_by_svinkels = recording_by_svinkels.unwrap();

    assert!(recording_by_svinkels.count > 1);
    assert_eq!(recording_by_svinkels.offset, 0);
    assert!(!recording_by_svinkels.entities.is_empty());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_browse_release_by_area() {
    let releases_france = Release::browse()
        .by(
            release::Browse::Area,
            "08310658-51eb-3801-80de-5a0739207115",
        )
        .execute();

    assert!(releases_france.is_ok());

    let releases_france = releases_france.unwrap();

    assert!(releases_france.count > 1);
    assert_eq!(releases_france.offset, 0);
    assert!(!releases_france.entities.is_empty());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_browse_release_by_label() {
    let ninjatune_releases = Release::browse()
        .by(
            release::Browse::Label,
            "dc940013-b8a8-4362-a465-291026c04b42",
        )
        .execute();

    assert!(ninjatune_releases.is_ok());

    let ninjatune_releases = ninjatune_releases.unwrap();

    assert!(ninjatune_releases.count > 1);
    assert_eq!(ninjatune_releases.offset, 0);
    assert!(!ninjatune_releases.entities.is_empty());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_browse_release_by_recording() {
    let release_of_l_ecole_du_micro_d_argent = Release::browse()
        .by(
            release::Browse::Recording,
            "72984ccf-9edb-44d4-aad2-f41c9ef5414a",
        )
        .execute();

    assert!(release_of_l_ecole_du_micro_d_argent.is_ok());

    let release_of_l_ecole_du_micro_d_argent = release_of_l_ecole_du_micro_d_argent.unwrap();

    assert!(release_of_l_ecole_du_micro_d_argent.count > 1);
    assert_eq!(release_of_l_ecole_du_micro_d_argent.offset, 0);
    assert!(!release_of_l_ecole_du_micro_d_argent.entities.is_empty());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_browse_release_by_track() {
    let release_with_phantom_by_justice = Release::browse()
        .by(
            release::Browse::Track,
            "d5bbc037-eace-3712-9af8-ecb378b65dd6",
        )
        .execute();

    assert!(release_with_phantom_by_justice.is_ok());

    let release_with_phantom_by_justice = release_with_phantom_by_justice.unwrap();

    assert!(release_with_phantom_by_justice.count > 0);
    assert_eq!(release_with_phantom_by_justice.offset, 0);
    assert!(!release_with_phantom_by_justice.entities.is_empty());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_browse_release_by_track_artist() {
    let release_featuring_akhenaton = Release::browse()
        .by(
            release::Browse::TrackArtist,
            "55808e24-5150-4abd-b86b-7c3d7a080da3",
        )
        .execute();

    assert!(release_featuring_akhenaton.is_ok());

    let release_featuring_akhenaton = release_featuring_akhenaton.unwrap();

    assert!(release_featuring_akhenaton.count > 1);
    assert_eq!(release_featuring_akhenaton.offset, 0);
    assert!(!release_featuring_akhenaton.entities.is_empty());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_browse_release_by_release_group() {
    let neil_young_harvest_releases = Release::browse()
        .by(
            release::Browse::ReleaseGroup,
            "b25419cf-71bf-3a54-8cd4-2161c61056a0",
        )
        .execute();

    assert!(neil_young_harvest_releases.is_ok());

    let neil_young_harvest_releases = neil_young_harvest_releases.unwrap();

    assert!(neil_young_harvest_releases.count > 1);
    assert_eq!(neil_young_harvest_releases.offset, 0);
    assert!(!neil_young_harvest_releases.entities.is_empty());

    thread::sleep(time::Duration::from_secs(1));
}
