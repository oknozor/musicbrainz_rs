use crate::model::artist::Artist;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ArtistCredit {
    pub name: String,
    pub joinphrase: String,
    pub artist: Artist,
}
