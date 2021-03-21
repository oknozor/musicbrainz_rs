use crate::impl_includes;
use crate::model::alias::Alias;
use crate::model::genre::Genre;
use crate::model::include::*;
use crate::model::tag::Tag;

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

impl_includes!(
    Series,
    (with_artist_relations, Include::ArtistRelations),
    (with_tags, Include::Tags),
    (with_aliases, Include::Aliases),
    (with_genres, Include::Genres),
    (with_annotations, Include::Annotations)
);
