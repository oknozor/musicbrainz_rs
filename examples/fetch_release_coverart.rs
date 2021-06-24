extern crate musicbrainz_rs;

use musicbrainz_rs::entity::release::*;
use musicbrainz_rs::entity::CoverartResponse;
use musicbrainz_rs::prelude::*;
use musicbrainz_rs::FetchCoverart;

fn main() {
    let in_utero_coverart = Release::fetch_coverart()
        .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
        .execute()
        .expect("Unable to get cover art");

    if let CoverartResponse::Json(coverart) = in_utero_coverart {
        println!("{}", coverart.images[0].back);
        println!("{}", coverart.images[0].image);
    }

    println!("");

    let in_utero = Release::fetch()
        .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
        .execute()
        .expect("Unable to get release");

    let in_utero_coverart = in_utero
        .get_coverart()
        .execute()
        .expect("Unable to get coverart");

    if let CoverartResponse::Json(coverart) = in_utero_coverart {
        println!("{}", coverart.images[0].back);
        println!("{}", coverart.images[0].image);
    }

    println!("");

    let in_utero_500px_front_coverart = Release::fetch_coverart()
        .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
        .res_500()
        .back()
        .execute()
        .expect("Unable to get cover art");

    if let CoverartResponse::Url(coverart_url) = in_utero_500px_front_coverart {
        println!("{}", coverart_url);
    }
}
