# Music brainz Rust

This is still WIP. 

Music brainz rust is a utility crate for the the [music brainz](https://musicbrainz.org/doc/Development/XML_Web_Service/Version_2) API. 

## Status 

Model implementation : 

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

Disambiguation comment
Annotation


Available method : 

- Artist : 
    - [x] artist by id
    - [ ] artist search
- Recording :
    - [x] recording by id
    - [ ] recording search 
