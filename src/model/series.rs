use crate::model::alias::Alias;
use crate::model::genre::Genre;
use crate::model::include_const::*;
use crate::model::tag::Tag;
use crate::Include as IncludeInto;

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

#[derive(Debug, PartialEq, Clone)]
pub enum Include {
    ArtistRelations,
    Tags,
    Aliases,
    Genres,
    Annotation,
}

impl IncludeInto<Series> for Include {
    fn as_str(&self) -> &str {
        match self {
            Include::ArtistRelations => INC_ARTIST_REL_VALUE,
            Include::Tags => INC_TAGS_VALUE,
            Include::Aliases => INC_ALIASES_VALUE,
            Include::Genres => INC_GENRES_VALUE,
            Include::Annotation => INC_ANNOTATION_VALUE,
        }
    }
}
