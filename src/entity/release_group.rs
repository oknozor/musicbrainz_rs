use super::{Include, Relationship, Subquery};
use crate::date_format;
use crate::entity::alias::Alias;
use crate::entity::artist_credit::ArtistCredit;
use crate::entity::genre::Genre;
use crate::entity::rating::Rating;
use crate::entity::relations::Relation;
use crate::entity::release::Release;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;
use chrono::NaiveDate;
use lucene_query_builder::QueryBuilder;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
#[serde(default)]
pub struct ReleaseGroup {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,

    pub primary_type_id: Option<String>,

    /// The type describes what kind of releases the release group represents, for example album,
    /// single, soundtrack, compilation etc.
    /// See the Type subpage for a full list of release group types.
    pub primary_type: Option<String>,

    pub secondary_type_ids: Vec<String>,
    pub secondary_types: Vec<String>,

    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub first_release_date: Option<NaiveDate>,

    /// The title of a release group is usually very similar, if not the same, as the titles of the
    /// releases contained within it.
    pub title: String,
    /// See Disambiguation Comment.
    pub disambiguation: String,
    pub relations: Option<Vec<Relation>>,
    pub artist_credit: Option<Vec<ArtistCredit>>,
    pub releases: Option<Vec<Release>>,
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
    pub aliases: Option<Vec<Alias>>,
    pub genres: Option<Vec<Genre>>,
    pub annotation: Option<String>,
}

#[derive(Debug, QueryBuilder, Default)]
pub struct ReleaseGroupSearchQuery {
    /// (part of) any alias attached to the release group (diacritics are ignored)
    alias: String,
    /// the MBID of any of the release group artists
    arid: String,
    /// (part of) the combined credited artist name for the release group, including join phrases (e.g. "Artist X feat.")
    artist: String,
    /// (part of) the name of any of the release group artists
    #[query_builder_field = "artistname"]
    artist_name: String,
    /// (part of) the release group's disambiguation comment
    comment: String,
    /// (part of) the credited name of any of the release group artists on this particular release group
    #[query_builder_field = "creditname"]
    credit_name: String,
    /// the release date of the earliest release in this release group (e.g. "1980-01-22")
    #[query_builder_field = "firstreleasedate"]
    first_release_date: String,
    /// the primary type of the release group
    #[query_builder_field = "primarytype"]
    primary_type: String,
    /// the MBID of any of the releases in the release group
    reid: String,
    /// (part of) the title of any of the releases in the release group
    release: String,
    /// (part of) the release group's title (diacritics are ignored)
    #[query_builder_field = "releasegroup"]
    release_group: String,
    /// (part of) the release group's title (with the specified diacritics)
    #[query_builder_field = "releasegroupaccent"]
    release_group_accent: String,
    /// the number of releases in the release group
    releases: String,
    /// the release group's MBID
    rgid: String,
    /// any of the secondary types of the release group
    #[query_builder_field = "secondarytype"]
    secondary_type: String,
    /// the status of any of the releases in the release group
    status: String,
    /// the status of any of the releases in the release group
    tag: String,
    /// legacy release group type field that predates the ability to set multiple types (see calculation code)
    #[query_builder_field = "type"]
    release_type: String,
}

impl_browse! {
ReleaseGroup,
   (by_artist, BrowseBy::Artist),
   (by_release, BrowseBy::Release),
   (by_collection, BrowseBy::Collection)
}

impl_includes!(
    ReleaseGroup,
    (
        with_release_group_relations,
        Include::Relationship(Relationship::ReleaseGroup)
    ),
    (
        with_series_relations,
        Include::Relationship(Relationship::Series)
    ),
    (with_url_relations, Include::Relationship(Relationship::Url)),
    (with_artists, Include::Subquery(Subquery::Artists)),
    (with_releases, Include::Subquery(Subquery::Releases)),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_ratings, Include::Subquery(Subquery::Rating)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);
