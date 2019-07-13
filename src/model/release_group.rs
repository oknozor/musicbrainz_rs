use chrono::NaiveDate;
use crate::{date_format, QueryAble};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReleaseGroup {

    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,

    #[serde(rename = "primary-type-id")]
    pub primary_type_id: String,

    /// The type describes what kind of releases the release group represents, for example album,
    /// single, soundtrack, compilation etc.
    /// See the Type subpage for a full list of release group types.
    #[serde(rename = "primary-type")]
    pub primary_type: String,

    #[serde(rename = "secondary-type-ids")]
    pub secondary_type_ids: Vec<String>,

    #[serde(rename = "secondary-types")]
    pub secondary_types: Vec<String>,

    #[serde(deserialize_with = "date_format::deserialize")]
    #[serde(rename = "first-release-date")]
    pub first_release_date: NaiveDate,

    /// The title of a release group is usually very similar, if not the same, as the titles of the
    /// releases contained within it.
    pub title: String,

    /// See Disambiguation Comment.
    pub disambiguation: String,
}