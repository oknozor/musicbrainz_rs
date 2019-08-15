extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use serde::de::DeserializeOwned;
use std::marker::PhantomData;

mod date_format;
pub mod model;

lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = { reqwest::Client::new() };
}

const BASE_URL: &str = "http://musicbrainz.org/ws/2";

pub struct Query<T, I: Include<T>> {
    path: String,
    include: Vec<I>,
    phantom: PhantomData<T>,
}

impl<'a, T, I> Query<T, I>
where
    I: Include<T> + PartialEq,
{
    pub fn execute(&mut self) -> Result<T, reqwest::Error>
    where
        T: QueryAble<'a, I> + DeserializeOwned,
    {
        self.path.push_str("?fmt=json");
        self.include_to_path();
        HTTP_CLIENT.get(&self.path).send()?.json()
    }

    pub fn include(&mut self, include: I) -> &mut Self {
        self.include.push(include);
        self
    }

    pub fn id(&mut self, id: &str) -> &mut Self {
        self.path.push_str(&format!("/{}", id));
        self
    }

    fn include_to_path(&mut self) {
        if !self.include.is_empty() {
            self.path.push_str("&inc=");
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

/// This trait provide a generic method to fetch music brainz resource
pub trait QueryAble<'a, I> {
    fn path() -> &'static str;

    fn fetch() -> Query<Self, I>
    where
        Self: Sized,
        I: Include<Self> + PartialEq,
    {
        Query {
            path: format!("{}/{}", BASE_URL, Self::path()),
            phantom: PhantomData,
            include: vec![],
        }
    }
}

pub trait Include<T> {
    fn as_str(&self) -> &str;
}
