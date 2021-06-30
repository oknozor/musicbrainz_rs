use super::{Include, Relationship, Subquery};
use crate::entity::alias::Alias;
use crate::entity::genre::Genre;
use crate::entity::rating::Rating;
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
    #[serde(rename = "type")]
    pub work_type: Option<String>,
    pub language: Option<String>,
    pub languages: Option<Vec<String>>,
    pub disambiguation: Option<String>,
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
    pub aliases: Option<Vec<Alias>>,
    pub genres: Option<Vec<Genre>>,
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
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_ratings, Include::Subquery(Subquery::Rating)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);
