use crate::date_format;
use crate::impl_includes;
use crate::model::alias::Alias;
use crate::model::artist_credit::ArtistCredit;
use crate::model::genre::Genre;
use crate::model::include::*;
use crate::model::rating::Rating;
use crate::model::release::Release;
use crate::model::tag::Tag;
use crate::BrowseBy;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct ReleaseGroup {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,

    pub primary_type_id: String,

    /// The type describes what kind of releases the release group represents, for example album,
    /// single, soundtrack, compilation etc.
    /// See the Type subpage for a full list of release group types.
    pub primary_type: String,

    pub secondary_type_ids: Vec<String>,

    pub secondary_types: Vec<String>,

    #[serde(deserialize_with = "date_format::deserialize")]
    pub first_release_date: NaiveDate,

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

#[derive(Debug, PartialEq, Clone)]
pub enum Browse {
    Artist,
    Release,
}

impl BrowseBy<ReleaseGroup> for Browse {
    fn as_str(&self) -> &str {
        match self {
            Browse::Artist => BROWSE_ARTIST_VALUE,
            Browse::Release => BROWSE_RELEASE_VALUE,
        }
    }
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
