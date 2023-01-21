use chrono::NaiveDate;
use serde::{self, Deserialize, Deserializer};

const FORMAT: &str = "%Y-%m-%d";

/// Some times, music brainz dates doesn't have a month or day
/// This artificially add january 1st before deserializing the API response
pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, FORMAT)
        .or_else(|_err| NaiveDate::parse_from_str(&format!("{}-01", &s), FORMAT))
        .or_else(|_err| NaiveDate::parse_from_str(&format!("{}-01-01", &s), FORMAT))
        .map_err(serde::de::Error::custom)
}

/// This is the Option<NaiveDate> version of the previous deserializer
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn deserialize_opt<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(deserialize(deserializer).ok())
}

#[cfg(test)]
mod tests {
    use crate::entity::lifespan::LifeSpan;
    use chrono::NaiveDate;

    #[test]
    fn should_deserialize_life_span() {
        let input = r#"
            {
                "ended": true,
                "end": "1994-04-05",
                "begin": "1988-01"
            }"#;

        let life_span: LifeSpan = serde_json::from_str(input).unwrap();
        assert_eq!(
            life_span,
            LifeSpan {
                ended: Some(true),
                begin: Some(NaiveDate::from_ymd_opt(1988, 1, 1).unwrap()),
                end: Some(NaiveDate::from_ymd_opt(1994, 4, 5).unwrap()),
            }
        )
    }

    #[test]
    fn should_deserialize_empty_date() {
        let input = r#"
            {
                "ended": true,
                "end": "",
                "begin": "1988-01"
            }"#;

        let life_span: LifeSpan = serde_json::from_str(input).unwrap();

        assert_eq!(
            life_span,
            LifeSpan {
                ended: Some(true),
                begin: Some(NaiveDate::from_ymd_opt(1988, 1, 1).unwrap()),
                end: None,
            }
        )
    }

    #[test]
    fn should_deserialize_missing_field() {
        let input = r#"
            {
                "ended": true,
                "begin": "1988-01"
            }"#;

        let life_span: LifeSpan = serde_json::from_str(input).unwrap();

        assert_eq!(
            life_span,
            LifeSpan {
                ended: Some(true),
                begin: Some(NaiveDate::from_ymd_opt(1988, 1, 1).unwrap()),
                end: None,
            }
        )
    }
}
