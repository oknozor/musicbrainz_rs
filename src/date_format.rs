use chrono::{NaiveDate};
use serde::{self, Deserialize, Deserializer};

const FORMAT: &'static str = "%Y-%m-%d";

/// Some times, music brainz dates doesn't have a month or day
/// This artificially add them before deserializing the API response
pub fn deserialize<'de, D>(
    deserializer: D,
) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, FORMAT)
        .or_else(|err| NaiveDate::parse_from_str(&format!("{}-01", &s), FORMAT))
        .or_else(|err| NaiveDate::parse_from_str(&format!("{}-01-01", &s), FORMAT))
        .map_err(serde::de::Error::custom)
}


/// This is the Option<NaiveDate> version of the previous deserializer
pub fn deserialize_opt<'de, D>(
    deserializer: D,
) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
{
    if let Ok(date) = deserialize(deserializer) {
        Ok(Some(date))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::artist::LifeSpan;
    use chrono::NaiveDate;

    #[test]
    fn deserialise_life_span() {
        let input = r#"
            {
                "ended": true,
                "end": "1994-04-05",
                "begin": "1988-01"
            }"#;

        let life_span: LifeSpan = serde_json::from_str(input).unwrap();
        assert_eq!(life_span, LifeSpan {
            ended: true,
            begin: NaiveDate::from_ymd(1988,01,01),
            end: Some(NaiveDate::from_ymd(1994,04,05)),
        })
    }
}