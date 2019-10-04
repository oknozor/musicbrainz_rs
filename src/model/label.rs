use crate::model::alias::Alias;
use crate::model::genre::Genre;
use crate::model::include_const::*;
use crate::model::rating::Rating;
use crate::model::release::Release;
use crate::model::tag::Tag;
use crate::BrowseBy;
use crate::Include as IncludeInto;

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

#[derive(Debug, PartialEq, Clone)]
pub enum Browse {
    Area,
    Release,
}

impl BrowseBy<Label> for Browse {
    fn as_str(&self) -> &str {
        match self {
            Browse::Area => BROWSE_AREA_VALUE,
            Browse::Release => BROWSE_RELEASE_VALUE,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Include {
    Releases,
    Aliases,
    Tags,
    Rating,
    Genres,
    Annotation,
}

impl IncludeInto<Label> for Include {
    fn as_str(&self) -> &str {
        match self {
            Include::Releases => INC_RELEASES_VALUE,
            Include::Aliases => INC_ALIASES_VALUE,
            Include::Tags => INC_TAGS_VALUE,
            Include::Rating => INC_RATINGS_VALUE,
            Include::Genres => INC_GENRES_VALUE,
            Include::Annotation => INC_ANNOTATION_VALUE,
        }
    }
}
