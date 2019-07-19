extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;


use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub mod model;
mod date_format;

const BASE_URL: &str = "http://musicbrainz.org/ws/2";
const _PARAMS: [(&str, &str); 1] = [("fmt", "json")];

pub struct Query<T> {
    pub path: String, 
    phantom: PhantomData<T>,
}

impl<'de, T> Query<T> {
    pub fn fetch(&self) 
        -> Result<T, reqwest::Error> 
            where T: QueryAble<'de> + DeserializeOwned
    {
        let client = reqwest::Client::new();
        client
            .get(&self.path)
            .send()?
            .json()
    }
}

/// This trait provide a generic method to fetch music brainz resource
pub trait QueryAble<'de> {

    fn path() -> &'static str;

    fn id(id: &str) -> Query<Self> where Self : Sized {
        Query {
            path: format!("{}/{}/{}?fmt=json", BASE_URL, Self::path(), id),
            phantom: PhantomData
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::model::area::*;
    use crate::model::artist::*;
    use crate::model::artist::ArtistType::*;
    use crate::model::label::*;
    use crate::model::lifespan::*;
    use crate::model::recording::Recording;
    use crate::model::release::*;
    use crate::model::release_group::*;
    use crate::model::work::*;
    use crate::QueryAble;

    #[test]
    fn should_get_artist_by_id() {
        let nirvana = Artist::id("5b11f4ce-a62d-471e-81fc-a69a8278c7da").fetch();

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
                    area_type: None,
                    type_id: None,
                    disambiguation: "".to_string(),
                    name: "United States".to_string(),
                    sort_name: "United States".to_string(),
                    iso_3166_1_codes: Some(vec![
                        "US".to_string(),
                    ]),
                    life_span: None,
                },
                begin_area: Area {
                    id: "a640b45c-c173-49b1-8030-973603e895b5".to_string(),
                    area_type: None,
                    type_id: None,
                    disambiguation: "".to_string(),
                    name: "Aberdeen".to_string(),
                    sort_name: "Aberdeen".to_string(),
                    iso_3166_1_codes: None,
                    life_span: None,
                },
                life_span: LifeSpan {
                    ended: true,
                    begin: Some(NaiveDate::from_ymd(1988, 01, 01)),
                    end: Some(NaiveDate::from_ymd(1994, 04, 05)),
                },
                tags: None,
                relations: None,
            }
        );
    }

    #[test]
    fn should_get_recording_by_id() {
        let polly = Recording::id("af40d6b8-58e8-4ca5-9db8-d4fca0b899e2").fetch();

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
    fn should_get_release_group_by_id() {
        let in_utero = ReleaseGroup::id("2a0981fb-9593-3019-864b-ce934d97a16e").fetch();

        assert_eq!(
            in_utero.unwrap(),
            ReleaseGroup {
                id: "2a0981fb-9593-3019-864b-ce934d97a16e".to_string(),
                primary_type_id: "f529b476-6e62-324f-b0aa-1f3e33d313fc".to_string(),
                primary_type: "Album".to_string(),
                secondary_type_ids: vec![],
                secondary_types: vec![],
                first_release_date: NaiveDate::from_ymd(1993, 09, 21),
                title: "In Utero".to_string(),
                disambiguation: "".to_string(),
            });
    }

    #[test]
    fn should_get_release() {
        let in_utero = Release::id("18d4e9b4-9247-4b44-914a-8ddec3502103").fetch();

        assert_eq!(
            in_utero.unwrap(),
            Release {
                id: "18d4e9b4-9247-4b44-914a-8ddec3502103".to_string(),
                title: "In Utero".to_string(),
                status_id: "4e304316-386d-3409-af2e-78857eec5cfe".to_string(),
                status: ReleaseStatus::Official,
                date: NaiveDate::from_ymd(1993, 01, 01),
                country: "US".to_string(),
                quality: ReleaseQuality::Normal,
                barcode: "0208314671259".to_string(),
                disambiguation: "".to_string(),
                packaging_id: "ec27701a-4a22-37f4-bfac-6616e0f9750a".to_string(),
                packaging: "Jewel Case".to_string(),
            }
        )
    }

    #[test]
    fn should_get_work_by_id() {
        let hotel_california = Work::id("22457dc0-ecbf-38f5-9056-11c858530a50").fetch();

        assert_eq!(
            hotel_california.unwrap(),
            Work {
                id: "22457dc0-ecbf-38f5-9056-11c858530a50".to_string(),
                type_id: "f061270a-2fd6-32f1-a641-f0f8676d14e6".to_string(),
                work_type: "Song".to_string(),
                language: "eng".to_string(),
                disambiguation: "".to_string(),
            }
        )
    }

    #[test]
    fn should_get_label_by_id() {
        let ninja_tune = Label::id("dc940013-b8a8-4362-a465-291026c04b42").fetch();

        assert_eq!(
        ninja_tune.unwrap(),
           Label {
               id: "dc940013-b8a8-4362-a465-291026c04b42".to_string(),
               type_id: "7aaa37fe-2def-3476-b359-80245850062d".to_string(),
               label_type: "Original Production".to_string(),
               name: "Ninja Tune".to_string(),
               sort_name: "Ninja Tune".to_string(),
               disambiguation: "".to_string(),
               country: "GB".to_string(),
               label_code: 12885
           }
        )
    }

    #[test]
    fn should_get_area_by_id(){
        let aberdeen = Area::id("a640b45c-c173-49b1-8030-973603e895b5").fetch();

        assert_eq!(
            aberdeen.unwrap(),
            Area {
                id: "a640b45c-c173-49b1-8030-973603e895b5".to_string(),
                area_type: Some("City".to_string()),
                type_id: Some("6fd8f29a-3d0a-32fc-980d-ea697b69da78".to_string()),
                disambiguation: "".to_string(),
                name: "Aberdeen".to_string(),
                sort_name: "Aberdeen".to_string(),
                iso_3166_1_codes: None,
                life_span: Some(LifeSpan {
                    ended: false,
                    begin: None,
                    end: None,
                })
            }
        )
    }
}
