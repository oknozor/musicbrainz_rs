use crate::impl_includes;
use crate::model::alias::Alias;
use crate::model::genre::Genre;
use crate::model::include::*;
use crate::model::lifespan::LifeSpan;
use crate::model::tag::Tag;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all(deserialize = "kebab-case"))]
#[serde(default)]
pub struct Area {
    pub id: String,
    #[serde(rename = "type")]
    pub area_type: Option<String>,
    pub type_id: Option<String>,
    pub disambiguation: String,
    pub name: String,
    pub sort_name: String,
    pub iso_3166_1_codes: Option<Vec<String>>,
    pub life_span: Option<LifeSpan>,
    pub tags: Option<Vec<Tag>>,
    pub aliases: Option<Vec<Alias>>,
    pub genres: Option<Vec<Genre>>,
    pub annotation: Option<String>,
}

impl_includes!(
    Area,
    (with_artists_relations, Include::ArtistRelations),
    (with_tags, Include::Tags),
    (with_aliases, Include::Aliases),
    (with_genres, Include::Genres),
    (with_annotations, Include::Annotations)
);
