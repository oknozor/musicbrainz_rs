extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;


pub mod model;
mod date_format;
use model::*;

const BASE_URL: &str = "http://musicbrainz.org/ws/2";
const PARAMS: [(&str, &str); 1] = [("fmt", "json")];

pub fn get_artist_by_id(artist_id: &str) -> Result<Artist, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .get(&format!("{}/artist/{}?fmt=json", BASE_URL, artist_id))
        .send()?
        .json()
}


#[cfg(test)]
mod tests {
    use crate::get_artist_by_id;
    use crate::model::*;
    use crate::model::Area;
    use crate::model::ArtistType::*;
    use chrono::{NaiveDate, DateTime, Utc};
    use chrono::offset::TimeZone;


    #[test]
    fn get_artist() {
        let nirvana = get_artist_by_id("5b11f4ce-a62d-471e-81fc-a69a8278c7da");

        assert_eq!(
            nirvana.unwrap(),
            Artist {
                id: String::from("5b11f4ce-a62d-471e-81fc-a69a8278c7da"),
                name: String::from("Nirvana"),
                sort_name: String::from("Nirvana"),
                disambiguation: String::from("90s US grunge band"),
                artist_type: Group,
                gender: None,
                area: Area {
                    id: "489ce91b-6658-3307-9877-795b68554c98".to_string(),
                    disambiguation: "".to_string(),
                    name: "United States".to_string(),
                    sort_name: "United States".to_string(),
                    iso_3166_codes: Some(vec![
                        "US".to_string(),
                    ]),
                    life_span: None
                },
                begin_area: Area {
                    id: "a640b45c-c173-49b1-8030-973603e895b5".to_string(),
                    disambiguation: "".to_string(),
                    name: "Aberdeen".to_string(),
                    sort_name: "Aberdeen".to_string(),
                    iso_3166_codes: None,
                    life_span: None
                },
                life_span: LifeSpan {
                    ended: true,
                    begin: NaiveDate::from_ymd(1988, 01, 01),
                    end: Some(NaiveDate::from_ymd(1994, 04, 05))
                },
                country: "US".to_string(),
            }
        );
    }
}
