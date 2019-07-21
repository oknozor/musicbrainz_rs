# ![MusicBrainz] Rust &emsp; [![Latest Version]][crates.io] [![Build Status]][travis]

[Build Status]: https://travis-ci.org/oknozor/musicbrainz_rs.svg?branch=master
[travis]: https://travis-ci.org/oknozor/musicbrainz_rs
[Latest Version]: https://img.shields.io/crates/v/musicbrainz_rs.svg
[crates.io]: https://www.crates.io/crates/musicbrainz_rs
[MusicBrainz]: https://staticbrainz.org/MB/header-logo-791fb3f.svg

**MusicBrainz rust is a utility crate for the the [MusicBrainz API](https://musicbrainz.org/doc/Development/XML_Web_Service/Version_2).**

---

This is still WIP. 

you may be looking for : 
- [Api documention](https://docs.rs/musicbrainz_rs)
- [The crate](https://www.crates.io/crates/musicbrainz_rs)

## Example : 


All queries look like this one: 

```rust
extern crate musicbrainz_rs;

use musicbrainz_rs::model::artist;
use musicbrainz_rs::model::artist::*;
use musicbrainz_rs::QueryAble;

fn main() {
    let nirvana = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .execute();

    assert_eq!(nirvana.unwrap().name, "Nirvana".to_string());
}
```

You can also use includes to get more detail about a resource : 

```rust

extern crate musicbrainz_rs;

use musicbrainz_rs::model::artist;
use musicbrainz_rs::model::artist::*;
use musicbrainz_rs::QueryAble;

fn main() {
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .include(artist::Include::Recordings)
        .execute()
        .unwrap();

    let recordings = john_lee_hooker.recordings.unwrap();

    assert!(recordings
        .iter()
        .any(|recording| recording.title == "A Little Bit Higher"));
}
```
To see what is currently implemented in the crate you can look at the `tests` directory. 

## TODO : 

- [x] Implement all the musicbrainz model
- [ ] Add non core-resources : `rating, tag, collection`
- [ ] Add alternative lookups : ` discid, isrc, iswc`
- [ ] Implement search and relations query
- [ ] Add pagination
- [ ] Provide some examples using the crate

#### Credits

Most of this crate documentation is taken from the official [MusicBrainz doc](https://musicbrainz.org/doc/MusicBrainz_Documentation), 
thanks to the [MetaBrainz](https://metabrainz.org/) Foundation and its sponsors and supporters. 
Cover Art provided by the [Cover Art Archive](https://coverartarchive.org/).
