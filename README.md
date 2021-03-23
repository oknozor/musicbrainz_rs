# ![MusicBrainz] Rust &emsp; 

[![Latest Version]][crates.io] [![Build Status]][Action] [![codecov](https://codecov.io/gh/oknozor/musicbrainz_rs/branch/master/graph/badge.svg)](https://codecov.io/gh/oknozor/musicbrainz_rs) ![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/oknozor/musicbrainz_rs) [![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org) ![License](https://img.shields.io/github/license/oknozor/musicbrainz_rs)

[Build Status]: https://github.com/oknozor/musicbrainz_rs/actions/workflows/CI.yaml/badge.svg
[Action]: https://github.com/oknozor/musicbrainz_rs/actions/workflows/CI.yaml
[Latest Version]: https://img.shields.io/crates/v/musicbrainz_rs.svg
[crates.io]: https://www.crates.io/crates/musicbrainz_rs
[MusicBrainz]: https://staticbrainz.org/MB/header-logo-791fb3f.svg

**MusicBrainz rust is a utility crate for the the [MusicBrainz API](https://musicbrainz.org/doc/Development/XML_Web_Service/Version_2).**

---

you may be looking for : 
- [Api documention](https://docs.rs/musicbrainz_rs)
- [The crate](https://www.crates.io/crates/musicbrainz_rs)

## Features

### Fetch query

To perform a [lookups](https://musicbrainz.org/doc/Development/XML_Web_Service/Version_2#Lookups) via fetch queries, 
you need to import the `Fetch` trait. This can be done using `musicbrainz_rs::prelude`

```rust
extern crate musicbrainz_rs;

use musicbrainz_rs::model::artist;
use musicbrainz_rs::model::artist::*;
use musicbrainz_rs::prelude::*;

fn main() {
    let nirvana = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .execute();

    assert_eq!(nirvana.unwrap().name, "Nirvana".to_string());
}
```

### Include parameters

You can also use includes to get more detail about a resource :

Every Musicbrainz resource has [allowed include parameters](https://musicbrainz.org/doc/Development/XML_Web_Service/Version_2#Subqueries).

```rust
extern crate musicbrainz_rs;

use musicbrainz_rs::model::label::*;
use musicbrainz_rs::prelude::*;

fn main() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .with_tags()
        .with_ratings()
        .execute()
        .unwrap();

    assert!(ninja_tune
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "independent"));

    assert!(ninja_tune.rating.is_some());
}
```

### Browse query

Use `musicbrainz_rs::Browse` or bring it in scope using `musicbrainz_rs::prelude` to perform a 
[browse query](https://musicbrainz.org/doc/Development/XML_Web_Service/Version_2#Browse).
Just like `Include` every muscibrainz resource has allowable linked entities for such queries.

```rust
extern crate musicbrainz_rs;

use musicbrainz_rs::model::artist;
use musicbrainz_rs::model::artist::Artist;
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
```

### Search query

Use `musicbrainz_rs::Search` to perform a [search query](https://musicbrainz.org/doc/MusicBrainz_API/Search).

```rust
extern crate musicbrainz_rs;

use musicbrainz_rs::model::artist::Artist;
use musicbrainz_rs::prelude::*;

fn main() {
    musicbrainz_rs::config::set_user_agent("my_awesome_app/1.0");

    let query = Artist::query_builder()
        .name("Miles Davis")
        .and()
        .country("US")
        .build();

    let query_result = Artist::search(query).execute().unwrap();
    let query_result: Vec<String> = query_result.entities
        .iter()
        .map(|artist| artist.name.clone()).collect();

    assert!(query_result.contains(&"Miles Davis".to_string()));
    assert!(query_result.contains(&"Miles Davis Quintet".to_string()));
}
```

### Custom user agent
You can set your application user-agent as recommended in the 
[musicbrainz documentation](https://musicbrainz.org/doc/XML_Web_Service/Rate_Limiting#User-Agent) :

```rust
extern crate musicbrainz_rs;

use musicbrainz_rs::model::artist::Artist;
use musicbrainz_rs::prelude::*;

fn main() {
    musicbrainz_rs::config::set_user_agent("my_awesome_app/1.0");

    let nirvana = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .execute();

    assert_eq!(nirvana.unwrap().name, "Nirvana".to_string());
}
```

## Examples

To see what is currently implemented in the crate you can look at the `tests` directory.

You can run examples with `cargo run --example example_name`

## Contributing

All contributions are welcome, if find a bug or have a feature request don't hesitate to open an issue!

#### Credits

Most of this crate documentation is taken from the official [MusicBrainz doc](https://musicbrainz.org/doc/MusicBrainz_Documentation),
thanks to the [MetaBrainz](https://metabrainz.org/) Foundation and its sponsors and supporters.
Cover Art provided by the [Cover Art Archive](https://coverartarchive.org/).
