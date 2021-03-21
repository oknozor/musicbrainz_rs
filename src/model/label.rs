use super::Include;
use crate::model::alias::Alias;
use crate::model::genre::Genre;
use crate::model::rating::Rating;
use crate::model::release::Release;
use crate::model::tag::Tag;
use crate::model::BrowseBy;

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
    (with_releases, Include::Releases),
    (with_tags, Include::Tags),
    (with_aliases, Include::Aliases),
    (with_ratings, Include::Rating),
    (with_genres, Include::Genres),
    (with_annotations, Include::Annotations)
);
