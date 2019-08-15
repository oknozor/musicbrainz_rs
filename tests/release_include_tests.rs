extern crate musicbrainz_rs;
use musicbrainz_rs::model::release;
use musicbrainz_rs::model::release::Media;
use musicbrainz_rs::model::release::Release;
use musicbrainz_rs::QueryAble;
use std::{thread, time};

#[test]
fn should_get_release_release_groups() {
    let justice_cross = Release::fetch()
        .id("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .include(release::Include::ReleaseGroup)
        .execute()
        .unwrap();

    assert_eq!(justice_cross.release_group.unwrap().title, "✝");

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_release_media() {
    let justice_cross = Release::fetch()
        .id("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .include(release::Include::Recordings)
        .execute()
        .unwrap();

    assert!(justice_cross
        .media
        .unwrap()
        .iter()
        .any(|media| media.format.as_ref().unwrap() == "CD"));

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_release_recordings() {
    let justice_cross = Release::fetch()
        .id("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .include(release::Include::Recordings)
        .execute()
        .unwrap();

    let medias: Vec<Media> = justice_cross.media.unwrap();
    let cd = medias.iter().next().unwrap();

    assert!(cd
        .tracks
        .as_ref()
        .unwrap()
        .iter()
        .any(|track| track.title == "D.A.N.C.E."));

    thread::sleep(time::Duration::from_secs(1));
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

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_release_tags() {
    let l_ecole_du_micro_d_argent = Release::fetch()
        .id("cba0035e-d8c9-4390-8569-02bdadaf87d3")
        .include(release::Include::Tags)
        .execute()
        .unwrap();

    assert!(l_ecole_du_micro_d_argent
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "hip hop"));

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_release_aliases() {
    let l_ecole_du_micro_d_argent = Release::fetch()
        .id("cba0035e-d8c9-4390-8569-02bdadaf87d3")
        .include(release::Include::Aliases)
        .execute()
        .unwrap();

    assert!(l_ecole_du_micro_d_argent.aliases.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_release_genres() {
    let l_ecole_du_micro_d_argent = Release::fetch()
        .id("cba0035e-d8c9-4390-8569-02bdadaf87d3")
        .include(release::Include::Genres)
        .execute()
        .unwrap();

    assert!(l_ecole_du_micro_d_argent.genres.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

// TODO: find an actual resource with annotation
// #[test]
// fn should_get_release_annotation() {
//     let l_ecole_du_micro_d_argent = Release::fetch()
//         .id("cba0035e-d8c9-4390-8569-02bdadaf87d3")
//         .include(release::Include::Annotation)
//         .execute()
//         .unwrap();

//     assert!(l_ecole_du_micro_d_argent.annotation.is_some());

//     thread::sleep(time::Duration::from_secs(1));
// }
