use super::{Include, Subquery};
use crate::date_format;
use crate::entity::alias::Alias;
use crate::entity::artist_credit::ArtistCredit;
use crate::entity::genre::Genre;
use crate::entity::rating::Rating;
use crate::entity::release::Release;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;
use chrono::NaiveDate;
use lucene_query_builder::QueryBuilder;

#[derive(Debug, Serialize, Deserialize, PartialEq, QueryBuilder, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct ReleaseGroup {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,

    pub primary_type_id: Option<String>,

    /// The type describes what kind of releases the release group represents, for example album,
    /// single, soundtrack, compilation etc.
    /// See the Type subpage for a full list of release group types.
    pub primary_type: Option<String>,

    #[serde(default)]
    pub secondary_type_ids: Vec<String>,

    #[serde(default)]
    pub secondary_types: Vec<String>,

    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub first_release_date: Option<NaiveDate>,

    /// The title of a release group is usually very similar, if not the same, as the titles of the
    /// releases contained within it.
    pub title: String,

    /// See Disambiguation Comment.
    #[serde(default)]
    pub disambiguation: String,

    #[query_builder_rename = "artist"]
    pub artist_credit: Option<Vec<ArtistCredit>>,
    #[query_builder_rename = "release"]
    pub releases: Option<Vec<Release>>,
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
    pub aliases: Option<Vec<Alias>>,
    pub genres: Option<Vec<Genre>>,
    pub annotation: Option<String>,
}

impl_browse! {
ReleaseGroup,
   (by_artist, BrowseBy::Artist),
   (by_release, BrowseBy::Release),
   (by_collection, BrowseBy::Collection)
}

impl_includes!(
    ReleaseGroup,
    (with_artists, Include::Subquery(Subquery::Artists)),
    (with_releases, Include::Subquery(Subquery::Releases)),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_ratings, Include::Subquery(Subquery::Rating)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);
