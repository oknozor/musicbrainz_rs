use crate::model::lifespan::LifeSpan;
use crate::Include as IncludeInto;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
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
}

#[derive(Debug, PartialEq)]
pub enum Include {
    ArtistRelations,
}

impl IncludeInto<Area> for Include {
    fn as_str(&self) -> &str {
        match self {
            Include::ArtistRelations => "artist-rels",
        }
    }
}
