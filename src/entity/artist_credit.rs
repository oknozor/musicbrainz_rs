use crate::entity::artist::Artist;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ArtistCredit {
    pub name: String,
    pub joinphrase: Option<String>,
    pub artist: Artist,
}
