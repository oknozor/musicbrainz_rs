extern crate musicbrainz_rs;

use musicbrainz_rs::entity::artist::Artist;
use musicbrainz_rs::prelude::*;

fn main() {
    let artists_on_in_utero_release = Artist::browse()
        .by_release("18d4e9b4-9247-4b44-914a-8ddec3502103")
        .execute();

    let artists_on_in_utero_release = artists_on_in_utero_release.unwrap();
    artists_on_in_utero_release
        .entities
        .iter()
        .for_each(|artist| println!("{:?}", artist.name));
}
