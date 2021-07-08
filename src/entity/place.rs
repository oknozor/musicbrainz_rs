use super::{Include, Relationship, Subquery};
use crate::entity::alias::Alias;
use crate::entity::area::Area;
use crate::entity::genre::Genre;
use crate::entity::lifespan::LifeSpan;
use crate::entity::relations::Relation;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Place {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub place_type: Option<String>,
    pub type_id: Option<String>,
    pub life_span: Option<LifeSpan>,
    pub coordinates: Option<Coordinates>,
    pub relations: Option<Vec<Relation>>,
    pub area: Area,
    pub address: String,
    pub disambiguation: String,
    pub aliases: Option<Vec<Alias>>,
    pub tags: Option<Vec<Tag>>,
    pub genres: Option<Vec<Genre>>,
    pub annotation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl_browse! {
Place,
   (by_area, BrowseBy::Area),
   (by_collection, BrowseBy::Collection)
}

impl_includes!(
    Place,
    (
        with_event_relations,
        Include::Relationship(Relationship::Event)
    ),
    (
        with_recording_relations,
        Include::Relationship(Relationship::Recording)
    ),
    (
        with_release_relations,
        Include::Relationship(Relationship::Release)
    ),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);
