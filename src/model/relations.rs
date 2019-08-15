use crate::date_format;
use crate::model::area::Area;
use crate::model::artist::Artist;
use crate::model::event::Event;
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Relation {
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub end: Option<NaiveDate>,
    pub attributes: Vec<String>,
    #[serde(flatten)]
    pub content: RelationContent,
    pub attribute_values: HashMap<String, String>,
    pub attribute_ids: HashMap<String, String>,
    pub target_credit: String,
    pub source_credit: String,
    pub ended: bool,
    pub type_id: String,
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub begin: Option<NaiveDate>,
    pub direction: String,
    #[serde(rename = "type")]
    pub relation_type: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum RelationContent {
    Artist(Artist),
    Area(Area),
    Event(Event),
}
