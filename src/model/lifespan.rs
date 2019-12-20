use crate::date_format;
use crate::deserialization::default;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LifeSpan {
    #[serde(default = "default::ended")]
    pub ended: Option<bool>,
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    #[serde(default = "default::date")]
    pub begin: Option<NaiveDate>,
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    #[serde(default = "default::date")]
    pub end: Option<NaiveDate>,
}

impl Default for LifeSpan {
    fn default() -> Self {
        LifeSpan {
            ended: None,
            begin: None,
            end: None
        }
    }
}
