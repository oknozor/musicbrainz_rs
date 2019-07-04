
extern crate music_brainz;

use music_brainz::get_artist_by_id;

fn main() {
    let nirvana  = get_artist_by_id("5b11f4ce-a62d-471e-81fc-a69a8278c7da"); 
    println!("{:?}" ,nirvana);
}