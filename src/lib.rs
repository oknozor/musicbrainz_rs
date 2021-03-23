//! MusicBrainz rust is a utility crate for the the
//! [MusicBrainz API](https://musicbrainz.org/doc/Development/XML_Web_Service/Version_2).
//! It strives to provide a simple and easy to use API to query the Musicbrainz database.
//!
//! All query are performed via a builder pattern fashioned syntax on musicbrainz entities available
//! in the [`entity`] module.
//!
//! ## Example
//!
//! The most simple usage would be to lookup an entity, knowing its [Musicbrainz ID](https://musicbrainz.org/doc/MusicBrainz_Identifier).
//!
//!  ```rust
//! use musicbrainz_rs::entity::artist::Artist;
//! use musicbrainz_rs::prelude::*;
//!
//! fn main() -> Result<(), Error> {
//!
//!     let nirvana = Artist::fetch()
//!         .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
//!         .execute();
//!
//!     assert_eq!(nirvana?.name, "Nirvana".to_string());
//!     Ok(())
//! }
//! ```
//!
//! Note that you need to either directly to bring the [`Fetch`] trait in scope or use the
//! [`prelude`] module to make the fetch method accessible.
//!
//! [musicbrainz::prelude]: musicbrainz_rs::prelude
//! [entity]: musicbrainz_rs::entity

extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use serde::de::DeserializeOwned;
use std::marker::PhantomData;

use crate::config::*;

/// Configure the HTTP client global state
pub mod config;
mod deserialization;
/// All Musicbrainz entities
pub mod entity;
/// Brings trait and type needed to perform any API query in scope
pub mod prelude;

use crate::entity::search::{SearchResult, Searchable};
use deserialization::date_format;
use entity::Browsable;
use entity::BrowseResult;
use entity::Include;

/// Type alias for [reqwest::Error]
pub type Error = reqwest::Error;

#[derive(Clone, Debug)]
struct Query<T> {
    path: String,
    include: Vec<Include>,
    phantom: PhantomData<T>,
}

/// perform a lookup of an entity when you have the MBID for that entity
///
/// # Lookups
///
/// You can perform a lookup of an entity when you have the MBID for that entity.
///
/// ## EXample
/// ```rust
/// # use musicbrainz_rs::prelude::*;
/// # fn main() -> Result<(), Error> {
/// # use musicbrainz_rs::entity::artist::Artist;
/// let nirvana = Artist::fetch()
///         .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
///         .execute();
///
/// assert_eq!(nirvana?.name, "Nirvana".to_string());
/// #   Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct FetchQuery<T>(Query<T>);

/// Direct lookup of all the entities directly linked to another entity
///
/// # Browse
///
/// Browse requests are a direct lookup of all the entities directly linked to another entity
/// ("directly linked" here meaning it does not include entities linked by a relationship).
///
/// ## Example
/// ```rust
/// # use musicbrainz_rs::prelude::*;
/// # fn main() -> Result<(), Error> {
/// # use musicbrainz_rs::entity::artist::Artist;
/// # use musicbrainz_rs::entity::release::Release;
/// let ubiktune_releases = Release::browse()
///         .by_label("47e718e1-7ee4-460c-b1cc-1192a841c6e5")
///         .execute();
///
/// assert!(!ubiktune_releases?.entities.is_empty());
/// #   Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct BrowseQuery<T>(Query<T>);

/// Search requests provide a way to search for MusicBrainz entities based on different
/// sorts of queries.
///
///# Search
///
/// The MusicBrainz API search requests provide a way to search for MusicBrainz entities
/// based on different sorts of queries.
/// ## Example
///
/// ```rust
/// # use musicbrainz_rs::prelude::*;
/// # fn main() -> Result<(), Error> {
/// # use musicbrainz_rs::entity::artist::Artist;
/// let query = Artist::query_builder()
///         .name("Miles Davis")
///         .and()
///         .country("US")
///         .build();
///
///     let query_result = Artist::search(query).execute()?;
///     let query_result: Vec<String> = query_result
///         .entities
///         .iter()
///         .map(|artist| artist.name.clone())
///         .collect();
///
///     assert!(query_result.contains(&"Miles Davis".to_string()));
///     assert!(query_result.contains(&"Miles Davis Quintet".to_string()));
/// #   Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct SearchQuery<T>(Query<T>);

impl<'a, T> FetchQuery<T>
where
    T: Clone,
{
    pub fn id(&mut self, id: &str) -> &mut Self {
        self.0.path.push_str(&format!("/{}", id));
        self
    }

    pub fn execute(&mut self) -> Result<T, Error>
    where
        T: Fetch<'a> + DeserializeOwned,
    {
        self.0.path.push_str(FMT_JSON);
        self.include_to_path();
        HTTP_CLIENT.get(&self.0.path).send()?.json()
    }

    fn include_to_path(&mut self) {
        self.0.include_to_path()
    }
}

impl<'a, T> BrowseQuery<T>
where
    T: Clone,
{
    pub fn execute(&mut self) -> Result<BrowseResult<T>, Error>
    where
        T: Fetch<'a> + DeserializeOwned + Browsable,
    {
        self.include_to_path();
        HTTP_CLIENT.get(&self.0.path).send()?.json()
    }

    fn include_to_path(&mut self) {
        self.0.include_to_path()
    }
}

impl<'a, T> SearchQuery<T>
where
    T: Search<'a> + Clone,
{
    pub fn execute(&mut self) -> Result<SearchResult<T>, Error>
    where
        T: Search<'a> + DeserializeOwned + Searchable,
    {
        self.include_to_path();
        HTTP_CLIENT.get(&self.0.path).send()?.json()
    }

    fn include_to_path(&mut self) {
        self.0.include_to_path()
    }
}

impl<'a, T> Query<T> {
    fn include(&mut self, include: Include) -> &mut Self {
        self.include.push(include);
        self
    }

    fn include_to_path(&mut self) {
        if !self.include.is_empty() {
            self.path.push_str(PARAM_INC);
        }

        for inc in self.include.iter() {
            if Some(inc) != self.include.last() {
                self.path.push_str(inc.as_str());
                self.path.push('+');
            } else {
                self.path.push_str(inc.as_str());
            }
        }
    }
}

/// Provide the entity HTTP api path, do not use this trait directly
pub trait Path<'a> {
    fn path() -> &'static str;
}

/// Implemented by all fetchable entities (see [`FetchQuery`])
pub trait Fetch<'a> {
    fn fetch() -> FetchQuery<Self>
    where
        Self: Sized + Path<'a>,
    {
        FetchQuery(Query {
            path: format!("{}/{}", BASE_URL, Self::path()),
            phantom: PhantomData,
            include: vec![],
        })
    }
}

/// Implemented by all browsable entities (see [`BrowseQuery`])
pub trait Browse<'a> {
    fn browse() -> BrowseQuery<Self>
    where
        Self: Sized + Path<'a>,
    {
        BrowseQuery(Query {
            path: format!("{}/{}", BASE_URL, Self::path()),
            phantom: PhantomData,
            include: vec![],
        })
    }
}

/// Implemented by all searchable entities (see [`SearchQuery`])
pub trait Search<'a> {
    fn search(query: String) -> SearchQuery<Self>
    where
        Self: Sized + Path<'a>,
    {
        SearchQuery(Query {
            path: format!("{}/{}{}&{}", BASE_URL, Self::path(), FMT_JSON, query),
            phantom: PhantomData,
            include: vec![],
        })
    }
}
