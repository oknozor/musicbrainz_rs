use crate::date_format;
use chrono::NaiveDate;

/// Aliases are used to store alternate names or misspellings. For more information and examples,
/// see the page about [aliases](https://musicbrainz.org/doc/Aliases).
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[serde(rename_all(deserialize = "kebab-case"))]
#[serde(default)]
pub struct Alias {
    pub name: String,
    pub sort_name: String,
    pub ended: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub begin: Option<NaiveDate>,
    #[serde(default)]
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub end: Option<NaiveDate>,
    #[serde(rename = "type")]
    pub alias_type: Option<String>,
    pub primary: Option<bool>,
    pub type_id: Option<String>,
}
