// FIXME: ultimately every const here shall be used by the lib so this annotation can be removed
#![allow(dead_code)]

// Browse
pub const BROWSE_AREA_VALUE: &str = "area";
pub const BROWSE_ARTIST_VALUE: &str = "artist";
pub const BROWSE_COLLECTION_VALUE: &str = "collection";
pub const BROWSE_RECORDING_VALUE: &str = "recording";
pub const BROWSE_RELEASE_VALUE: &str = "release";
pub const BROWSE_RELEASE_GROUP_VALUE: &str = "release-group";
pub const BROWSE_WORK_VALUE: &str = "work";
pub const BROWSE_PLACE_VALUE: &str = "place";
pub const BROWSE_LABEL_VALUE: &str = "label";
pub const BROWSE_TRACK_VALUE: &str = "track";
pub const BROWSE_TRACK_ARTIST_VALUE: &str = "track_artist";

// area, collection, recording, release, release-group, work

impl Include {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Include::Labels => "labels",
            Include::Recordings => "recordings",
            Include::Tags => "tags",
            Include::Rating => "ratings",
            Include::Aliases => "aliases",
            Include::Genres => "genres",
            Include::Annotations => "annotation",
            Include::ArtistRelations => "artist-rels",
            Include::EventRelations => "event-rels",
            Include::Releases => "releases",
            Include::ReleaseGroups => "release-groups",
            Include::Works => "works",
            Include::Artists => "artists",
            Include::Places => "places",
            Include::Events => "events",
            Include::Urls => "urls",
            Include::Areas => "areas",
            Include::ArtistCredits => "artist-credits",
            Include::DiscIds => "discids",
            Include::ReleasesWithDiscIds => "releases+discids",
            Include::Instruments => "instruments",
            Include::Series => "series",
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Include {
    Urls,
    Areas,
    ArtistCredits,
    Labels,
    Events,
    Places,
    DiscIds,
    ArtistRelations,
    EventRelations,
    Releases,
    ReleasesWithDiscIds,
    ReleaseGroups,
    Recordings,
    Aliases,
    Works,
    Tags,
    Rating,
    Genres,
    Annotations,
    Artists,
    Series,
    Instruments,
}
