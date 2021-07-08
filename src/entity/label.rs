use super::{Include, Relationship, Subquery};
use crate::entity::alias::Alias;
use crate::entity::genre::Genre;
use crate::entity::rating::Rating;
use crate::entity::relations::Relation;
use crate::entity::release::Release;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Label {
    pub id: String,
    pub type_id: Option<String>,
    #[serde(rename = "type")]
    pub label_type: Option<String>,
    pub name: String,
    pub sort_name: String,
    pub disambiguation: String,
    pub relations: Option<Vec<Relation>>,
    pub country: Option<String>,
    pub label_code: Option<u32>,
    pub releases: Option<Vec<Release>>,
    pub aliases: Option<Vec<Alias>>,
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
    pub genres: Option<Vec<Genre>>,
    pub annotation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct LabelInfo {
    pub catalog_number: String,
    pub label: Label,
}

impl_browse! {
Label,
   (by_area, BrowseBy::Area),
   (by_release, BrowseBy::Release),
   (by_collection, BrowseBy::Collection)
}

impl_includes!(
    Label,
    (with_artist_relations, Include::Relationship(Relationship::Artist)),
    (with_label_relations, Include::Relationship(Relationship::Label)),
    (with_recording_relations, Include::Relationship(Relationship::Recording)),
    (with_release_relations, Include::Relationship(Relationship::Release)),
    (with_url_relations, Include::Relationship(Relationship::Url)),
    (with_releases, Include::Subquery(Subquery::Releases)),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_ratings, Include::Subquery(Subquery::Rating)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);
