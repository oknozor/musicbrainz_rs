use crate::model::alias::Alias;
use crate::model::lifespan::LifeSpan;
use crate::model::rating::Rating;
use crate::model::tag::Tag;
use crate::Include as IncludeInto;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Event {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub type_id: Option<String>,
    pub life_span: LifeSpan,
    pub disambiguation: String,
    pub cancelled: bool,
    pub time: String,    // need some info on that value, current IT test returns ""
    pub setlist: String, // same here
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
    pub aliases: Option<Vec<Alias>>,
}

#[derive(Debug, PartialEq)]
pub enum Include {
    ArtistRelations,
    Tags,
    Rating,
    Aliases,
}

impl IncludeInto<Event> for Include {
    fn as_str(&self) -> &str {
        match self {
            Include::ArtistRelations => "artist-rels",
            Include::Tags => "tags",
            Include::Aliases => "aliases",
            Include::Rating => "ratings",
        }
    }
}
