extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;


pub mod model;
mod date_format;

use model::artist::*;
use crate::model::recording::Recording;
use serde::Deserialize;
use serde::de::DeserializeOwned;

const BASE_URL: &str = "http://musicbrainz.org/ws/2";
const PARAMS: [(&str, &str); 1] = [("fmt", "json")];


/// This trait provide a generic method to fetch music brainz resource
pub trait QueryAble<'de> {
    fn by_id(id: &str)
             -> Result<Self, reqwest::Error>
        where Self: DeserializeOwned {
        let client = reqwest::Client::new();
        client
            .get(&format!("{}/{}/{}?fmt=json", BASE_URL, Self::path(), id))
            .send()?
            .json()
    }

    fn path() -> &'static str;
}

#[cfg(test)]
mod tests {
    use crate::model::artist::*;
    use crate::model::artist::Area;
    use crate::model::artist::ArtistType::*;
    use chrono::NaiveDate;
    use crate::model::recording::Recording;
    use crate::QueryAble;


    #[test]
    fn should_get_recording_by_id() {
        let polly = Recording::by_id("af40d6b8-58e8-4ca5-9db8-d4fca0b899e2");

        assert_eq!(
            polly.unwrap(),
            Recording {
                id: "af40d6b8-58e8-4ca5-9db8-d4fca0b899e2".to_string(),
                title: "(New Wave) Polly".to_string(),
                video: false,
                length: 246000,
                disambiguation: "".to_string(),
            });
    }

    #[test]
    fn should_get_artist_by_id() {
        let nirvana = Artist::by_id("5b11f4ce-a62d-471e-81fc-a69a8278c7da");

        assert_eq!(
            nirvana.unwrap(),
            Artist {
                id: String::from("5b11f4ce-a62d-471e-81fc-a69a8278c7da"),
                name: String::from("Nirvana"),
                sort_name: String::from("Nirvana"),
                disambiguation: String::from("90s US grunge band"),
                artist_type: Group,
                gender: None,
                country: "US".to_string(),
                area: Area {
                    id: "489ce91b-6658-3307-9877-795b68554c98".to_string(),
                    disambiguation: "".to_string(),
                    name: "United States".to_string(),
                    sort_name: "United States".to_string(),
                    iso_3166_codes: Some(vec![
                        "US".to_string(),
                    ]),
                    life_span: None,
                },
                begin_area: Area {
                    id: "a640b45c-c173-49b1-8030-973603e895b5".to_string(),
                    disambiguation: "".to_string(),
                    name: "Aberdeen".to_string(),
                    sort_name: "Aberdeen".to_string(),
                    iso_3166_codes: None,
                    life_span: None,
                },
                life_span: LifeSpan {
                    ended: true,
                    begin: NaiveDate::from_ymd(1988, 01, 01),
                    end: Some(NaiveDate::from_ymd(1994, 04, 05)),
                },
                tags: None,
            }
        );
    }
}
