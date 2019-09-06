use crate::model::alias::Alias;
use crate::model::genre::Genre;
use crate::model::include_const::*;
use crate::model::lifespan::LifeSpan;
use crate::model::rating::Rating;
use crate::model::tag::Tag;
use crate::Include as IncludeInto;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Event {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub type_id: Option<String>,
    pub life_span: Option<LifeSpan>,
    pub disambiguation: String,
    pub cancelled: bool,
    pub time: String,    // need some info on that value, current IT test returns ""
    pub setlist: String, // same here
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
    pub aliases: Option<Vec<Alias>>,
    pub genres: Option<Vec<Genre>>,
    pub annotation: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Include {
    ArtistRelations,
    Tags,
    Rating,
    Aliases,
    Genres,
    Annotation,
}

impl IncludeInto<Event> for Include {
    fn as_str(&self) -> &str {
        match self {
            Include::ArtistRelations => INC_ARTIST_REL_VALUE,
            Include::Tags => INC_TAGS_VALUE,
            Include::Aliases => INC_ALIASES_VALUE,
            Include::Rating => INC_RATINGS_VALUE,
            Include::Genres => INC_GENRES_VALUE,
            Include::Annotation => INC_ANNOTATION_VALUE,
        }
    }
}
