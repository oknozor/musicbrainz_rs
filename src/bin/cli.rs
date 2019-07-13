
extern crate music_brainz;

use music_brainz::model::artist::Artist;
use music_brainz::QueryAble;

fn main() {
    let nirvana  = Artist::by_id("5b11f4ce-a62d-471e-81fc-a69a8278c7da");
    println!("{:?}" ,nirvana);
}