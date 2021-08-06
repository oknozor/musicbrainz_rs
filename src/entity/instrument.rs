use super::{Include, Relationship, Subquery};
use crate::entity::alias::Alias;
use crate::entity::genre::Genre;
use crate::entity::relations::Relation;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;

use lucene_query_builder::QueryBuilder;

/// Instruments are devices created or adapted to make musical sounds. Instruments are primarily
/// used in relationships between two other entities and for that, each instrument entity has a
/// parallel relationship attribute with the same MBID. Instruments, like relationship attributes,
/// can only be edited by relationship editors.
/// See [Instrument List](https://musicbrainz.org/instruments) for the full list.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Instrument {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,
    /// The instrument name is the name of the instrument, typically the most common name in English.
    pub name: String,
    /// The type categorises the instrument by the way the sound is created, similar to the
    /// Hornbostel-Sachs classification. The possible values are: Wind instrument, String instrument,
    /// Precussion instrument, Electronic instrument, Family, Ensemble, Other instrument.
    #[serde(rename = "type")]
    pub instrument_type: InstrumentType,
    pub type_id: String,
    /// The description is a brief description of the main characteristics of the instrument.
    pub description: Option<String>,
    /// The disambiguation comments are fields in the database used to help distinguish identically
    /// named artists, labels and other entities.
    pub disambiguation: Option<String>,
    pub relations: Option<Vec<Relation>>,
    pub tags: Option<Vec<Tag>>,
    /// Aliases are alternate names for an instrument, which currently have two main functions:
    /// localised names and search hints.
    pub aliases: Option<Vec<Alias>>,
    pub genres: Option<Vec<Genre>>,
    /// Annotations are text fields, functioning like a miniature wiki, that can be added to any
    /// existing artists, labels, recordings, releases, release groups and works.
    pub annotation: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, QueryBuilder)]
pub struct InstrumentSearchQuery {
    /// (part of) any alias attached to the instrument (diacritics are ignored)
    pub alias: String,
    /// (part of) the instrument's disambiguation comment
    pub comment: String,
    /// (part of) the description of the instrument (in English)
    pub description: String,
    /// the MBID of the instrument
    pub iid: String,
    /// (part of) the instrument's name (diacritics are ignored)
    pub instrument: String,
    /// (part of) the instrument's name (with the specified diacritics)
    #[query_builder_field = "instrumentaccent"]
    pub instrument_accent: String,
    /// (part of) a tag attached to the instrument
    pub tag: String,
    /// the instrument's type
    #[query_builder_field = "type"]
    pub instrument_type: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum InstrumentType {
    /// An aerophone, i.e. an instrument where the sound is created by vibrating air. The instrument
    /// itself does not vibrate.
    #[serde(rename = "Wind instrument")]
    WindInstrument,
    /// A chordophone, i.e. an instrument where the sound is created by the vibration of strings.
    #[serde(rename = "String instrument")]
    StringInstrument,
    /// An idiophone, i.e. an instrument where the sound is produced by the body of the instrument
    /// vibrating, or a drum (most membranophones) where the sound is produced by a stretched membrane
    /// which is struck or rubbed.
    #[serde(rename = "Percussion instrument")]
    PercussionInstrument,
    /// An electrophone, i.e. an instrument where the sound is created with electricity.
    #[serde(rename = "Electronic instrument")]
    ElectronicInstrument,
    /// A grouping of related but different instruments, like the different violin-like instruments.
    Family,
    /// A standard grouping of instruments often played together, like a string quartet.
    Ensemble,
    /// An instrument which doesn't fit in the categories above.
    #[serde(rename = "Other instrument")]
    OtherInstrument,
}

impl_browse!(Instrument, (by_collection, BrowseBy::Collection));

impl_includes!(
    Instrument,
    (with_url_relations, Include::Relationship(Relationship::Url)),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);
