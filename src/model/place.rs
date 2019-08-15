use crate::model::alias::Alias;
use crate::model::area::Area;
use crate::model::genre::Genre;
use crate::model::lifespan::LifeSpan;
use crate::model::tag::Tag;
use crate::Include as IncludeInto;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Place {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub place_type: String,
    pub type_id: String,
    pub life_span: LifeSpan,
    pub coordinates: Coordinates,
    pub area: Area,
    pub address: String,
    pub disambiguation: String,
    pub aliases: Option<Vec<Alias>>,
    pub tags: Option<Vec<Tag>>,
    pub genres: Option<Vec<Genre>>,
    pub annotation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Coordinates {
    pub latitude: String,
    pub longitude: String,
}

#[derive(Debug, PartialEq)]
pub enum Include {
    ArtistRelations,
    Aliases,
    Tags,
    Genres,
    Annotation,
}

impl IncludeInto<Place> for Include {
    fn as_str(&self) -> &str {
        match self {
            Include::ArtistRelations => "artist-rels",
            Include::Aliases => "aliases",
            Include::Tags => "tags",
            Include::Genres => "genres",
            Include::Annotation => "annotation",
        }
    }
}
