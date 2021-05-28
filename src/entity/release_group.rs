use super::Include;
use crate::date_format;
use crate::entity::alias::Alias;
use crate::entity::artist_credit::ArtistCredit;
use crate::entity::genre::Genre;
use crate::entity::rating::Rating;
use crate::entity::release::Release;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
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

    pub artist_credit: Option<Vec<ArtistCredit>>,
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
    (with_artists, Include::Artists),
    (with_releases, Include::Releases),
    (with_tags, Include::Tags),
    (with_aliases, Include::Aliases),
    (with_genres, Include::Genres),
    (with_ratings, Include::Rating),
    (with_annotations, Include::Annotations)
);
