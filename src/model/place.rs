use super::Include;
use crate::model::alias::Alias;
use crate::model::area::Area;
use crate::model::genre::Genre;
use crate::model::lifespan::LifeSpan;
use crate::model::tag::Tag;
use crate::model::BrowseBy;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Place {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub place_type: Option<String>,
    pub type_id: Option<String>,
    pub life_span: LifeSpan,
    pub coordinates: Option<Coordinates>,
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
    (with_artist_relations, Include::ArtistRelations),
    (with_tags, Include::Tags),
    (with_aliases, Include::Aliases),
    (with_genres, Include::Genres),
    (with_annotations, Include::Annotations)
);
