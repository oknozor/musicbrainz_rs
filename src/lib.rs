extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use serde::de::DeserializeOwned;
use std::marker::PhantomData;

mod date_format;
pub mod model;

const BASE_URL: &str = "http://musicbrainz.org/ws/2";

pub struct Query<T> {
    path: String,
    include: Vec<Include>,
    phantom: PhantomData<T>,
}

impl<'a, T> Query<T> {
    pub fn execute(&mut self) -> Result<T, reqwest::Error>
    where
        T: QueryAble<'a> + DeserializeOwned,
    {
        let client = reqwest::Client::new();

        println!("{}", &self.path);
        self.path.push_str("?fmt=json");
        self.include_to_path();
        client.get(&self.path).send()?.json()
    }

    pub fn include(&mut self, include: Include) -> &mut Self {
        self.include.push(include);
        self
    }

    pub fn id(&mut self, id: &str) -> &mut Self {
        self.path.push_str(&format!("/{}", id));
        self
    }

    pub fn include_to_path(&mut self) {
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
pub trait QueryAble<'a> {
    fn path() -> &'static str;

    fn fetch() -> Query<Self>
    where
        Self: Sized,
    {
        Query {
            path: format!("{}/{}", BASE_URL, Self::path()),
            phantom: PhantomData,
            include: vec![],
        }
    }

    fn get_allowed_includes() -> Vec<Include>;
}

#[derive(Debug, PartialEq)]
pub enum Include {
    Artists,
    Recordings,
    ReleaseGroups,
    Releases,
    Works,
    UserCollections,
    Ircs,
    Collections,
    Labels,
    ArtistRelations,
    EventRelations,
    UrlRelations,
    AreaRelations,
    InstrumentRelations,
    LabelRelations,
    PlaceRelations,
    ReleaseRelations,
    ReleaseGroupRelations,
    SeriesRelations,
    WorkRelations,
    Annotations,
    Aliases,
    Tags,
    Rating,
    UserTags,
    UserRatings,
    Genres,
    UserGenres,
}

impl Include {
    pub fn as_str(&self) -> &str {
        use Include::*;
        match self {
            Artists => "artists",
            Recordings => "recording",
            Releases => "releases",
            ReleaseGroups => "release-groups",
            Works => "works",
            UserCollections => "user-collections",
            Ircs => "ircs",
            UrlRelations => "url-rels",
            Collections => "collectionts",
            Labels => "labels",
            ArtistRelations => "artist-rels",
            EventRelations => "event-rels",
            AreaRelations => "area-rels",
            InstrumentRelations => "instrument-rels",
            LabelRelations => "label-rels",
            PlaceRelations => "place-rels",
            ReleaseRelations => "release-rels",
            ReleaseGroupRelations => "release-group-rels",
            SeriesRelations => "series-rels",
            WorkRelations => "work-rels",
            Annotations => "annotation",
            Aliases => "aliases",
            Tags => "tags",
            Rating => "ratings",
            UserTags => "user-tags",
            UserRatings => "user-ratings",
            Genres => "genres",
            UserGenres => "user-genres",
        }
    }
}
