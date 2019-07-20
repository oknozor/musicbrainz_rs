use crate::Include as IncludeInto;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Url {
    pub id: String,
    pub resource: String,
}

#[derive(Debug, PartialEq)]
pub enum Include {
    ArtistRelations,
}

impl IncludeInto<Url> for Include {
    fn as_str(&self) -> &str {
        match self {
            Include::ArtistRelations => "artist-rels",
        }
    }
}
