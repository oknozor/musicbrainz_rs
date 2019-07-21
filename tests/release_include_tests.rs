extern crate musicbrainz_rs;
use musicbrainz_rs::model::release;
use musicbrainz_rs::model::release::Release;
use musicbrainz_rs::QueryAble;

#[test]
fn should_get_release_release_groups() {
    let justice_cross = Release::fetch()
        .id("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .include(release::Include::ReleaseGroup)
        .execute()
        .unwrap();

    assert_eq!(justice_cross.release_group.unwrap().title, "✝");
}

#[test]
fn should_get_release_recordings() {
    let justice_cross = Release::fetch()
        .id("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .include(release::Include::Recordings)
        .execute()
        .unwrap();

    assert!(justice_cross
        .media
        .unwrap()
        .get(0)
        .unwrap()
        .tracks
        .iter()
        .any(|track| track.title == "D.A.N.C.E."));
}

#[test]
fn should_get_release_label() {
    let justice_cross = Release::fetch()
        .id("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .include(release::Include::Labels)
        .execute()
        .unwrap();

    assert!(justice_cross
        .label_info
        .unwrap()
        .iter()
        .any(|label_info| label_info.label.name == "Ed Banger Records"));
}
