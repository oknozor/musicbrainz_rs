use super::Include;
use crate::entity::alias::Alias;
use crate::entity::genre::Genre;
use crate::entity::lifespan::LifeSpan;
use crate::entity::rating::Rating;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Event {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub type_id: Option<String>,
    pub life_span: Option<LifeSpan>,
    pub disambiguation: String,
    pub cancelled: bool,
    pub time: String,
    // need some info on that value, current IT test returns ""
    pub setlist: String,
    // same here
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
    pub aliases: Option<Vec<Alias>>,
    pub genres: Option<Vec<Genre>>,
    pub annotation: Option<String>,
}

impl_browse! {
Event,
   (by_area, BrowseBy::Area),
   (by_collection, BrowseBy::Collection),
   (by_artist, BrowseBy::Artist),
   (by_place, BrowseBy::Place)
}

impl_includes!(
    Event,
    (with_artist_relations, Include::ArtistRelations),
    (with_tags, Include::Tags),
    (with_aliases, Include::Aliases),
    (with_ratings, Include::Rating),
    (with_genres, Include::Genres),
    (with_annotations, Include::Annotations)
);
