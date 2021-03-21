use super::Include;
use crate::model::alias::Alias;
use crate::model::genre::Genre;
use crate::model::tag::Tag;
use crate::model::BrowseBy;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Instrument {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub instrument_type: String,
    pub type_id: String,
    pub description: String,
    pub disambiguation: String,
    pub tags: Option<Vec<Tag>>,
    pub aliases: Option<Vec<Alias>>,
    pub genres: Option<Vec<Genre>>,
    pub annotation: Option<String>,
}

impl_browse!(Instrument, (by_collection, BrowseBy::Collection));

impl_includes!(
    Instrument,
    (with_artist_relations, Include::ArtistRelations),
    (with_tags, Include::Tags),
    (with_aliases, Include::Aliases),
    (with_genres, Include::Genres),
    (with_annotations, Include::Annotations)
);
