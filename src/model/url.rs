use crate::model::include_const::*;
use crate::model::tag::Tag;
use crate::Include as IncludeInto;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Url {
    pub id: String,
    pub resource: String,
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Include {
    ArtistRelations,
}

impl IncludeInto<Url> for Include {
    fn as_str(&self) -> &str {
        match self {
            Include::ArtistRelations => INC_ARTIST_REL_VALUE,
        }
    }
}
