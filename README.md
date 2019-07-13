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

## Status 

### Model implementation : 

- [ ] Artist  
    - [x] MBID
    - [x] Name
    - [x] Sort name
    - [x] Type
    - [x] Gender
    - [x] Area
    - [x] Begin and end dates
    - [ ] IPI code
    - [ ] ISNI code
    - [ ] Alias
    - [x] Disambiguation comment
    - [ ] Annotation
- Recording
    - [x] MBID
    - [x] Title
    - [ ] Artist
    - [x] Length
    - [ ] ISRC
    - [x] Disambiguation comment
    - [ ] Annotation
- Release group
    - [x] MBID
    - [x] Title
    - [ ] Artist
    - [x] Type
    - [x] Disambiguation comment
    - [ ] Annotation

### Available method : 

- Artist : 
    - [x] artist by id
    - [ ] artist search
- Recording :
    - [x] recording by id
    - [ ] recording search
- Release group
    - [x] release group by id
    - [ ] release group search

## TODO : 

- [ ] Implement all the musicbrainz model
- [ ] Implement search and relations query
- [ ] Provide some examples using the crate

#### Credits

Most of this crate documentation is taken from the official [MusicBrainz doc](https://musicbrainz.org/doc/MusicBrainz_Documentation), 
thanks to the [MetaBrainz](https://metabrainz.org/) Foundation and its sponsors and supporters. 
Cover Art provided by the [Cover Art Archive](https://coverartarchive.org/).
