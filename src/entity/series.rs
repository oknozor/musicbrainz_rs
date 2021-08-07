use super::{Include, Relationship, Subquery};
use crate::entity::alias::Alias;
use crate::entity::genre::Genre;
use crate::entity::relations::Relation;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;

use lucene_query_builder::QueryBuilder;

/// A series is a sequence of separate release groups, releases, recordings, works, artists or
/// events with a common theme.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Series {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,
    /// The series name is the official name of the series.
    pub name: String,
    #[serde(rename = "type")]
    /// The type primarily describes what type of entity the series contains. The possible values are:
    /// Release group series, Release series, Recording series, Work series (with further subtypes:
    /// Catalogue), Artist series (with further subtypes: Artist award), Event series (with further
    /// subtypes: Tour, Festival, Run, Residency)
    // FIXME: Can we use a `SeriesType` enum here?
    pub series_type: String,
    /// The disambiguation comments are fields in the database used to help distinguish identically
    /// named artists, labels and other entities.
    pub disambiguation: String,
    pub type_id: String,
    /// Relationships are a way to represent all the different ways in which entities are connected
    /// to each other and to URLs outside MusicBrainz.
    pub relations: Option<Vec<Relation>>,
    pub tags: Option<Vec<Tag>>,
    /// Aliases are alternate names for a series, which currently have two main functions: localised
    /// names and search hints.
    pub aliases: Option<Vec<Alias>>,
    /// Genres are currently supported in MusicBrainz as part of the tag system.
    pub genres: Option<Vec<Genre>>,
    /// Annotations are text fields, functioning like a miniature wiki, that can be added to any
    /// existing artists, labels, recordings, releases, release groups and works.
    pub annotation: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, QueryBuilder)]
pub struct SeriesSearchQuery {
    /// (part of) any alias attached to the series (diacritics are ignored)
    pub alias: String,
    /// (part of) the series' disambiguation comment
    pub comment: String,
    /// (part of) the series' name (diacritics are ignored)
    pub series: String,
    /// (part of) the series' name (with the specified diacritics)
    #[query_builder_field = "seriesaccent"]
    pub series_accent: String,
    /// the series' MBID
    pub sid: String,
    /// (part of) a tag attached to the series
    pub tag: String,
    /// the series' type
    #[query_builder_field = "type"]
    pub series_type: String,
}

impl_browse!(Series, (by_collection, BrowseBy::Collection));

impl_includes!(
    Series,
    (
        with_artist_relations,
        Include::Relationship(Relationship::Artist)
    ),
    (
        with_release_group_relations,
        Include::Relationship(Relationship::ReleaseGroup)
    ),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);
