use crate::model::area::Area;
use crate::model::lifespan::LifeSpan;
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
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Coordinates {
    pub latitude: String,
    pub longitude: String,
}

#[derive(Debug, PartialEq)]
pub enum Include {
    ArtistRelations,
}

impl IncludeInto<Place> for Include {
    fn as_str(&self) -> &str {
        match self {
            Include::ArtistRelations => "artist-rels",
        }
    }
}
