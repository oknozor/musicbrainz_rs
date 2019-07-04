
use super::model::*;

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

    use crate::model::Type::Group;
    use crate::model::Artist;

    use crate::music_brainz::get_artist_by_id;

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
                country: String::from("US")
            }
        );
    }

}
