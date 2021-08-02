use crate::date_format;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(default)]
pub struct LifeSpan {
    pub ended: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub begin: Option<NaiveDate>,
    #[serde(default)]
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub end: Option<NaiveDate>,
}

impl Default for LifeSpan {
    fn default() -> Self {
        LifeSpan {
            ended: None,
            begin: None,
            end: None,
        }
    }
}
