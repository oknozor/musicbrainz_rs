
extern crate music_brainz_rs;

use music_brainz_rs::model::artist::Artist;
use music_brainz_rs::QueryAble;

fn main() {
    let nirvana  = Artist::by_id("5b11f4ce-a62d-471e-81fc-a69a8278c7da");
    println!("{:?}" ,nirvana);
}