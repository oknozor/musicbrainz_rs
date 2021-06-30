use super::{Include, Relationship, Subquery};
use crate::entity::alias::Alias;
use crate::entity::genre::Genre;
use crate::entity::lifespan::LifeSpan;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;

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

impl_browse!(Area, (by_collection, BrowseBy::Collection));

impl_includes!(
    Area,
    (
        with_artist_relations,
        Include::Relationship(Relationship::Artist)
    ),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);
