use serde::{Serialize, Deserialize};

/// Disc ID is the code number which MusicBrainz uses to link a physical CD to a release listing.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Disc {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,
    pub offset_count: u32,
    pub sectors: u32,
    pub offsets : Vec<u32>
}
