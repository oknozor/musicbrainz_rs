use crate::entity::artist::Artist;

/// Artist credits indicate who is the main credited artist (or artists) for releases, release groups,
/// tracks and recordings, and how they are credited. They consist of artists, with (optionally)
/// their names as credited in the specific release, track, etc., and join phrases between them.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ArtistCredit {
    pub name: String,
    pub joinphrase: Option<String>,
    pub artist: Artist,
}
