extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use serde::de::DeserializeOwned;
use std::marker::PhantomData;

use crate::config::*;

mod browse_deserializer;
pub mod config;
mod date_format;
pub mod model;
use model::browse::Browsable;
use model::browse::BrowseResult;

#[derive(Clone, Debug)]
pub struct Query<T, I: Include<T>> {
    path: String,
    include: Vec<I>,
    phantom: PhantomData<T>,
}

#[derive(Clone, Debug)]
pub struct FetchQuery<T, I: Include<T>>(Query<T, I>);

#[derive(Clone, Debug)]
pub struct BrowseQuery<T, I: Include<T>>(Query<T, I>);

impl<'a, T, I> FetchQuery<T, I>
where
    I: Include<T> + PartialEq + Clone,
    T: Clone,
{
    pub fn id(&mut self, id: &str) -> &mut Self {
        self.0.path.push_str(&format!("/{}", id));
        self
    }

    pub fn execute(&mut self) -> Result<T, reqwest::Error>
    where
        T: Fetch<'a, I> + DeserializeOwned,
    {
        self.0.path.push_str(FMT_JSON);
        self.include_to_path();
        HTTP_CLIENT.get(&self.0.path).send()?.json()
    }

    pub fn include(&mut self, include: I) -> &mut Self {
        self.0.include = self.0.include(include).include.to_owned();
        self
    }

    fn include_to_path(&mut self) {
        self.0.include_to_path()
    }
}

impl<'a, T, I> BrowseQuery<T, I>
where
    I: Include<T> + PartialEq + Clone,
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
        T: Fetch<'a, I> + DeserializeOwned + Browsable,
    {
        self.include_to_path();
        HTTP_CLIENT.get(&self.0.path).send()?.json()
    }

    pub fn include(&mut self, include: I) -> &mut Self {
        self.0.include = self.0.include(include).include.to_owned();
        self
    }

    fn include_to_path(&mut self) {
        self.0.include_to_path()
    }
}

impl<'a, T, I> Query<T, I>
where
    I: Include<T> + PartialEq,
{
    pub fn include(&mut self, include: I) -> &mut Self {
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
                self.path.push_str("+");
            } else {
                self.path.push_str(inc.as_str());
            }
        }
    }
}

pub trait Path<'a> {
    fn path() -> &'static str;
}

/// This trait provide utily methods for music brainz Fetch resources
pub trait Fetch<'a, I> {
    fn fetch() -> FetchQuery<Self, I>
    where
        Self: Sized + Path<'a>,
        I: Include<Self> + PartialEq,
    {
        FetchQuery(Query {
            path: format!("{}/{}", BASE_URL, Self::path()),
            phantom: PhantomData,
            include: vec![],
        })
    }
}

/// This trait provide utily methods for music brainz Browse resources
pub trait Browse<'a, I> {
    fn browse() -> BrowseQuery<Self, I>
    where
        Self: Sized + Path<'a>,
        I: Include<Self> + PartialEq,
    {
        BrowseQuery(Query {
            path: format!("{}/{}", BASE_URL, Self::path()),
            phantom: PhantomData,
            include: vec![],
        })
    }
}

/// Generic trait object to get allowable include on <T>
pub trait Include<T> {
    fn as_str(&self) -> &str;
}

/// Generic trait object to get allowable browse value on <T>
pub trait BrowseBy<T> {
    fn as_str(&self) -> &str;
}
