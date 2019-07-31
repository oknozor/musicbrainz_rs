use crate::model::alias::Alias;
use crate::model::genre::Genre;
use crate::model::tag::Tag;
use crate::Include as IncludeInto;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
}

#[derive(Debug, PartialEq)]
pub enum Include {
    ArtistRelations,
    Tags,
    Aliases,
    Genres,
}

impl IncludeInto<Series> for Include {
    fn as_str(&self) -> &str {
        match self {
            Include::ArtistRelations => "artist-rels",
            Include::Tags => "tags",
            Include::Aliases => "aliases",
            Include::Genres => "genres",
        }
    }
}
