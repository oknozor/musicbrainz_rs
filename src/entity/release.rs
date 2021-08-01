use chrono::NaiveDate;
use lucene_query_builder::QueryBuilder;

use super::{Include, Relationship, Subquery};
use crate::date_format;
use crate::entity::alias::Alias;
use crate::entity::artist_credit::ArtistCredit;
use crate::entity::genre::Genre;
use crate::entity::label::LabelInfo;
use crate::entity::recording::Recording;
use crate::entity::relations::Relation;
use crate::entity::release_group::ReleaseGroup;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;

/// A MusicBrainz release represents the unique release (i.e. issuing) of a product on a specific
/// date with specific release information such as the country, label, barcode and packaging.
/// If you walk into a store and purchase an album or single, they are each represented in
/// MusicBrainz as one release.
///
/// Each release belongs to a release group and contains at least one medium (commonly referred to
/// as a disc when talking about a CD release). Each medium has a tracklist.
/// A medium is the actual physical medium that stores the audio content. This means that each CD
/// in a multi-disc release will be entered as separate mediums within the release, and that both
/// sides of a vinyl record or cassette will exist on one medium. Mediums have a format (e.g. CD,
/// DVD, vinyl, and cassette) and can optionally also have a title. Sometimes a medium can be a
/// side of a disc. For example, the two sides of a hybrid SACD (the CD side and the SACD side)
/// should be entered as two mediums.
/// Tracklists represent the set and ordering of tracks as listed on a liner, and the same tracklist
/// can appear on more than one release. For example, a boxset compilation that contains previously
/// released CDs would share the same tracklists as the separate releases.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Release {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,

    /// The title of the release.
    pub title: String,

    #[serde(rename = "status-id")]
    pub status_id: Option<String>,

    /// The status describes how "official" a release is.
    pub status: Option<ReleaseStatus>,

    /// The date the release was issued.
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    #[serde(default)]
    pub date: Option<NaiveDate>,

    /// The country the release was issued in.
    pub country: Option<String>,

    ///  Data quality indicates how good the data for a release is. It is not a mark of how good or
    /// bad the music itself is - for that, use ratings.
    pub quality: Option<ReleaseQuality>,

    /// The barcode, if the release has one. The most common types found on releases are 12-digit
    /// UPCs and 13-digit EANs.
    pub barcode: Option<String>,

    /// The disambiguation comments are fields in the database used to help distinguish identically
    /// named artists, labels and other entities.
    pub disambiguation: Option<String>,

    #[serde(rename = "packaging-id")]
    pub packaging_id: Option<String>,

    /// The physical packaging that accompanies the release. See the
    /// [list of packaging](https://musicbrainz.org/doc/Release/Packaging) for more information.
    pub packaging: Option<String>, //TODO: This might be an enum needs to test all against all possible values

    pub relations: Option<Vec<Relation>>,
    /// The release group associated with this release.
    pub release_group: Option<ReleaseGroup>,
    /// Artist credits indicate who is the main credited artist (or artists) for releases, release
    /// groups, tracks and recordings, and how they are credited.
    pub artist_credit: Option<Vec<ArtistCredit>>,
    pub media: Option<Vec<Media>>,
    /// The label which issued the release. There may be more than one.
    pub label_info: Option<Vec<LabelInfo>>,
    pub tags: Option<Vec<Tag>>,
    /// Aliases are alternate names for a release.
    pub aliases: Option<Vec<Alias>>,
    /// Genres are currently supported in MusicBrainz as part of the tag system.
    pub genres: Option<Vec<Genre>>,
    /// Annotations are text fields, functioning like a miniature wiki, that can be added to any
    /// existing artists, labels, recordings, releases, release groups and works.
    pub annotation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ReleaseTextRepresentation {
    /// The language a release's track list is written in. The possible values are taken from the ISO
    /// 639-3 standard.
    pub language: Language,
    /// The script used to write the release's track list. The possible values are taken from the
    /// ISO 15924 standard.
    pub script: ReleaseScript,
}

/// The script used to write the release's track list. The possible values are taken from the
/// [ISO 15924](https://en.wikipedia.org/wiki/ISO_15924) standard.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ReleaseScript {
    /* TODO: we need to test all posible values to build the enum see https://musicbrainz.org/doc/Release */
    /// ## Latin (also known as Roman or, incorrectly, "English")
    /// Latin is the most common script, and usually the correct choice. It is used
    /// for all Western European languages, and many others. It is also the most common script used for transliterations.
    Latn,
}

/* TODO: we need to test all posible values to build the enum see https://musicbrainz.org/doc/Release */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Language {
    Eng,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum ReleaseQuality {
    /// The release needs serious fixes, or its existence is hard to prove (but it's not clearly fake).
    Low,

    /// All available data has been added, if possible including cover art with liner info that
    /// proves it.
    High,

    /// This is the default setting - technically "unknown" if the quality has never been modified,
    /// "normal" if it has.
    Normal,

    Unknown,

    None,
}

/// The release status describes how "official" a release is.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ReleaseStatus {
    /// Any release officially sanctioned by the artist and/or their record company. Most releases
    /// will fit into this category.
    Official,

    /// A give-away release or a release intended to promote an upcoming official release (e.g.
    /// pre-release versions, releases included with a magazine, versions supplied to radio DJs
    /// for air-play).
    Promotion,

    /// An unofficial/underground release that was not sanctioned by the artist and/or the record
    /// company. This includes unofficial live recordings and pirated releases.
    Bootleg,

    /// An alternate version of a release where the titles have been changed. These don't correspond
    /// to any real release and should be linked to the original release using the
    /// [transl(iter)ation relationship](https://musicbrainz.org/relationship/fc399d47-23a7-4c28-bfcf-0607a562b644).
    PseudoRelease,

    None,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Media {
    pub title: Option<String>,
    pub position: Option<u32>,
    pub track_count: u32,
    pub disc_count: Option<u32>,
    pub format_id: Option<String>,
    pub format: Option<String>,
    pub tracks: Option<Vec<Track>>,
}

/// A track is the way a recording is represented on a particular release (or, more exactly, on a
/// particular medium).
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Track {
    pub recording: Recording,
    pub title: String,
    pub number: String,
    pub length: Option<u32>,
    pub position: u32,
    pub id: String,
}

#[derive(Debug, Default, Serialize, Deserialize, QueryBuilder)]
pub struct ReleaseSearchQuery {
    /// (part of) any alias attached to the release group (diacritics are ignored)
    alias: String,
    /// the MBID of any of the release group artists
    arid: String,
    /// (part of) the combined credited artist name for the release group, including join phrases (e.g. "Artist X feat.")
    artist: String,
    /// (part of) the name of any of the release group artists
    #[query_builder_field = "artistname"]
    artist_name: String,
    /// an Amazon ASIN for the release
    asin: String,
    /// the barcode for the release
    barcode: String,
    /// any catalog number for this release (insensitive to case, spaces and separators)
    #[query_builder_field = "catno"]
    catalog_number: String,
    /// (part of) the release group's disambiguation comment
    comment: String,
    /// the 2-letter code (ISO 3166-1 alpha-2) for any country the release was released in
    country: String,
    /// (part of) the credited name of any of the release group artists on this particular release group
    #[query_builder_field = "creditname"]
    credit_name: String,
    /// a release date for the release (e.g. "1980-01-22")
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    #[serde(default)]
    date: Option<NaiveDate>,
    /// the total number of disc IDs attached to all mediums on the release
    discids: u32,
    /// the number of disc IDs attached to any one medium on the release
    #[query_builder_field = "discidsmedium"]
    discids_medium: u32,
    /// the format of any medium in the release (insensitive to case, spaces, and separators)
    format: String,
    /// the MBID of any of the release labels
    laid: String,
    /// (part of) the name of any of the release labels
    label: String,
    /// the ISO 639-3 code for the release language
    lang: String,
    /// the number of mediums on the release
    mediums: u32,
    /// the format of the release (insensitive to case, spaces, and separators)
    packaging: String,
    /// the primary type of the release group
    #[query_builder_field = "primarytype"]
    primary_type: String,
    /// the listed quality of the data for the release (one of "low", "normal", "high")
    quality: String,
    /// the MBID of any of the releases in the release group
    reid: String,
    /// (part of) the title of any of the releases in the release group
    release: String,
    /// (part of) the release's title (with the specified diacritics)
    #[query_builder_field = "releaseaccent"]
    release_accent: String,
    /// the release group's MBID
    rgid: String,
    /// the ISO 15924 code for the release script
    script: String,
    /// any of the secondary types of the release group
    #[query_builder_field = "secondarytype"]
    secondary_type: String,
    /// the status of any of the releases in the release group
    status: String,
    /// the status of any of the releases in the release group
    tag: String,
    /// the total number of tracks on the release
    tracks: u32,
    /// the number of tracks on any one medium on the release
    #[query_builder_field = "tracksmedium"]
    tracks_medium: u32,
    /// legacy release group type field that predates the ability to set multiple types (see calculation code)
    #[query_builder_field = "type"]
    release_type: String,
}

impl_browse! {
Release,
   (by_area, BrowseBy::Area),
   (by_artist, BrowseBy::Artist),
   (by_label, BrowseBy::Label),
   (by_track, BrowseBy::Track),
   (by_track_artist, BrowseBy::TrackArtist),
   (by_recording, BrowseBy::Recording),
   (by_release_group, BrowseBy::ReleaseGroup),
   (by_collection, BrowseBy::Collection)
}

impl_includes!(
    Release,
    (with_artists, Include::Subquery(Subquery::Artists)),
    (with_labels, Include::Subquery(Subquery::Labels)),
    (
        with_artist_relations,
        Include::Relationship(Relationship::Artist)
    ),
    (
        with_work_relations,
        Include::Relationship(Relationship::Work)
    ),
    (with_url_relations, Include::Relationship(Relationship::Url)),
    (
        with_work_level_relations,
        Include::Relationship(Relationship::WorkLevel)
    ),
    (
        with_recording_level_relations,
        Include::Relationship(Relationship::RecordingLevel)
    ),
    (with_recordings, Include::Subquery(Subquery::Recordings)),
    (
        with_release_groups,
        Include::Subquery(Subquery::ReleaseGroups)
    ),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_ratings, Include::Subquery(Subquery::Rating)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);
