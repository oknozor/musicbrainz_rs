extern crate musicbrainz_rs;

use musicbrainz_rs::model::artist;
use musicbrainz_rs::model::artist::Artist;
use musicbrainz_rs::Browse;

fn main() {
    let artists_on_in_utero_release = Artist::browse()
        .by(
            artist::Browse::Release,
            "18d4e9b4-9247-4b44-914a-8ddec3502103",
        )
        .execute();

    let artists_on_in_utero_release = artists_on_in_utero_release.unwrap();
    artists_on_in_utero_release
        .entities
        .iter()
        .for_each(|artist| println!("{:?}", artist.name));
}
