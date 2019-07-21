use crate::model::alias::Alias;
use crate::model::release::Release;
use crate::Include as IncludeInto;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Label {
    pub id: String,
    pub type_id: String,
    #[serde(rename = "type")]
    pub label_type: String,
    pub name: String,
    pub sort_name: String,
    pub disambiguation: String,
    pub country: String,
    pub label_code: u32,
    pub releases: Option<Vec<Release>>,
    pub aliases: Option<Vec<Alias>>,
}

#[derive(Debug, PartialEq)]
pub enum Include {
    Releases,
    Aliases,
}

impl IncludeInto<Label> for Include {
    fn as_str(&self) -> &str {
        match self {
            Include::Releases => "releases",
            Include::Aliases => "aliases",
        }
    }
}
