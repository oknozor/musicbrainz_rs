use super::Include;
use crate::entity::alias::Alias;
use crate::entity::genre::Genre;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Series {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub serie_type: String,
    pub disambiguation: String,
    pub type_id: String,
    pub tags: Option<Vec<Tag>>,
    pub aliases: Option<Vec<Alias>>,
    pub genres: Option<Vec<Genre>>,
    pub annotation: Option<String>,
}

impl_browse!(Series, (by_collection, BrowseBy::Collection));

impl_includes!(
    Series,
    (with_artist_relations, Include::ArtistRelations),
    (with_tags, Include::Tags),
    (with_aliases, Include::Aliases),
    (with_genres, Include::Genres),
    (with_annotations, Include::Annotations)
);
