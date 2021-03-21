use crate::impl_includes;
use crate::model::alias::Alias;
use crate::model::artist_credit::ArtistCredit;
use crate::model::genre::Genre;
use crate::model::include::Include;
use crate::model::include::*;
use crate::model::rating::Rating;
use crate::model::relations::Relation;
use crate::model::release::Release;
use crate::model::tag::Tag;
use crate::BrowseBy;

/// A recording is an entity in MusicBrainz which can be linked to tracks on releases. Each track
/// must always be associated with a single recording, but a recording can be linked to any number
/// of tracks.
/// A recording represents distinct audio that has been used to produce at least one released track
/// through copying or mastering. A recording itself is never produced solely through copying or
/// mastering.
/// Generally, the audio represented by a recording corresponds to the audio at a stage in the
/// production process before any final mastering but after any editing or mixing.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Recording {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,

    /// The title of the recording.
    pub title: String,

    pub video: Option<bool>,

    /// The length of the recording. It's only entered manually for
    /// [standalone recordings](https://musicbrainz.org/doc/Standalone_Recording). For recordings
    /// that are being used on releases, the recording length is the median length of all tracks
    /// (that have a track length) associated with that recording. If there is an even number of
    /// track lengths, the smaller median candidate is used.
    pub length: Option<u32>, // TODO: CUSTOM Deserialized to make this a duration

    /// See Disambiguation Comment.
    pub disambiguation: Option<String>,

    pub relations: Option<Vec<Relation>>,
    pub releases: Option<Vec<Release>>,
    pub artist_credit: Option<Vec<ArtistCredit>>,
    pub aliases: Option<Vec<Alias>>,
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
    pub genres: Option<Vec<Genre>>,
    pub annotation: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Browse {
    Artist,
    Release,
    Work,
}

impl BrowseBy<Recording> for Browse {
    fn as_str(&self) -> &str {
        match self {
            Browse::Artist => BROWSE_ARTIST_VALUE,
            Browse::Release => BROWSE_RELEASE_VALUE,
            Browse::Work => BROWSE_WORK_VALUE,
        }
    }
}

impl_includes!(
    Recording,
    (with_artists, Include::Artists),
    (with_releases, Include::Releases),
    (with_tags, Include::Tags),
    (with_aliases, Include::Aliases),
    (with_genres, Include::Genres),
    (with_ratings, Include::Rating),
    (with_annotations, Include::Annotations)
);
