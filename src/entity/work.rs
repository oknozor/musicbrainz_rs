use super::{Include, Relationship, Subquery};
use crate::entity::alias::Alias;
use crate::entity::genre::Genre;
use crate::entity::rating::Rating;
use crate::entity::relations::Relation;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;

/// In MusicBrainz terminology, a work is a distinct intellectual or artistic creation, which can be
/// expressed in the form of one or more audio recordings. While a work in MusicBrainz is usually
/// musical in nature, it is not necessarily so. For example, a work could be a novel, play,
/// poem or essay, later recorded as an oratory or audiobook.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Work {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,
    pub title: String,
    pub type_id: Option<String>,
    /// Works are represented predominantly at two levels: Discrete works, Aggregate works.
    // FIXME: Can we use a `WorkType` enum here?
    #[serde(rename = "type")]
    pub work_type: Option<String>,
    pub language: Option<String>,
    pub languages: Option<Vec<String>>,
    /// The disambiguation comments are fields in the database used to help distinguish identically
    /// named artists, labels and other entities.
    pub disambiguation: Option<String>,
    pub relations: Option<Vec<Relation>>,
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
    /// If a discrete work is known by name(s) or in language(s) other than its canonical name,
    /// these are specified in the work’s aliases.
    pub aliases: Option<Vec<Alias>>,
    /// Genres are currently supported in MusicBrainz as part of the tag system.
    pub genres: Option<Vec<Genre>>,
    /// Annotations are text fields, functioning like a miniature wiki, that can be added to any
    /// existing artists, labels, recordings, releases, release groups and works.
    pub annotation: Option<String>,
}

impl_browse! {
Work,
   (by_artist, BrowseBy::Artist),
   (by_collection, BrowseBy::Collection)
}

impl_includes!(
    Work,
    (
        with_artist_relations,
        Include::Relationship(Relationship::Artist)
    ),
    (
        with_label_relations,
        Include::Relationship(Relationship::Label)
    ),
    (
        with_recording_relations,
        Include::Relationship(Relationship::Recording)
    ),
    (with_url_relations, Include::Relationship(Relationship::Url)),
    (
        with_work_relations,
        Include::Relationship(Relationship::Work)
    ),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_ratings, Include::Subquery(Subquery::Rating)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);
