use chrono::NaiveDate;
use crate::{date_format};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LifeSpan {
    pub ended: bool,
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub begin: Option<NaiveDate>,
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub end: Option<NaiveDate>,
}