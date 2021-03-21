extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use serde::de::DeserializeOwned;
use std::marker::PhantomData;

use crate::config::*;

pub mod config;
mod deserialization;

use deserialization::date_format;

pub mod model;

use crate::model::include::Include;
use crate::model::search::{SearchResult, Searchable};
use model::browse::Browsable;
use model::browse::BrowseResult;

#[macro_export]
macro_rules! impl_includes {
    ($ty: ty, $(($args:ident, $inc: expr)),+) => {
        use crate::{FetchQuery, BrowseQuery, SearchQuery};
        impl FetchQuery<$ty> {
               $(pub fn $args(&mut self) -> &mut Self  {
                     self.0.include = self.0.include($inc).include.to_owned();
                   self
               })*
            }

        impl BrowseQuery<$ty> {
               $(pub fn $args(&mut self) -> &mut Self  {
                     self.0.include = self.0.include($inc).include.to_owned();
                   self
               })*
            }

        impl SearchQuery<$ty> {
               $(pub fn $args(&mut self) -> &mut Self  {
                     self.0.include = self.0.include($inc).include.to_owned();
                   self
               })*
            }
        }
}

#[derive(Clone, Debug)]
pub struct Query<T> {
    path: String,
    include: Vec<Include>,
    phantom: PhantomData<T>,
}

#[derive(Clone, Debug)]
pub struct FetchQuery<T>(Query<T>);

#[derive(Clone, Debug)]
pub struct BrowseQuery<T>(Query<T>);

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

    pub fn execute(&mut self) -> Result<T, reqwest::Error>
    where
        T: Fetch<'a> + DeserializeOwned,
    {
        self.0.path.push_str(FMT_JSON);
        self.include_to_path();
        HTTP_CLIENT.get(&self.0.path).send()?.json()
    }

    pub fn include(&mut self, include: Include) -> &mut Self {
        self.0.include = self.0.include(include).include.to_owned();
        self
    }

    fn include_to_path(&mut self) {
        self.0.include_to_path()
    }
}

impl<'a, T> BrowseQuery<T>
where
    T: Clone,
{
    pub fn by<B>(&mut self, browse_by: B, id: &str) -> &mut Self
    where
        B: BrowseBy<T> + PartialEq + Clone,
    {
        self.0.path.push_str(FMT_JSON);
        self.0
            .path
            .push_str(&format!("&{}={}", browse_by.as_str(), id));
        self
    }

    pub fn execute(&mut self) -> Result<BrowseResult<T>, reqwest::Error>
    where
        T: Fetch<'a> + DeserializeOwned + Browsable,
    {
        self.include_to_path();
        HTTP_CLIENT.get(&self.0.path).send()?.json()
    }

    pub fn include(&mut self, include: Include) -> &mut Self {
        self.0.include = self.0.include(include).include.to_owned();
        self
    }

    fn include_to_path(&mut self) {
        self.0.include_to_path()
    }
}

impl<'a, T> SearchQuery<T>
where
    T: Search<'a> + Clone,
{
    pub fn execute(&mut self) -> Result<SearchResult<T>, reqwest::Error>
    where
        T: Search<'a> + DeserializeOwned + Searchable,
    {
        self.include_to_path();
        HTTP_CLIENT.get(&self.0.path).send()?.json()
    }

    pub fn include(&mut self, include: Include) -> &mut Self {
        self.0.include = self.0.include(include).include.to_owned();
        self
    }

    fn include_to_path(&mut self) {
        self.0.include_to_path()
    }
}

impl<'a, T> Query<T> {
    pub fn include(&mut self, include: Include) -> &mut Self {
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

pub trait Path<'a> {
    fn path() -> &'static str;
}

/// This trait provide utility methods for musicbrainz Fetch resources
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

/// This trait provide utility methods for musicbrainz Browse resources
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

/// This trait provide utility methods for musicbrainz Search Query
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

/// Generic trait object to get allowable browse value on <T>
pub trait BrowseBy<T> {
    fn as_str(&self) -> &str;
}
