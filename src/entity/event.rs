use super::{Include, Relationship, Subquery};
use crate::entity::alias::Alias;
use crate::entity::genre::Genre;
use crate::entity::lifespan::LifeSpan;
use crate::entity::rating::Rating;
use crate::entity::relations::Relation;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;

/// An event refers to an organised event which people can attend, and is relevant to MusicBrainz.
/// Generally this means live performances, like concerts and festivals.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Event {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,

    /// The name is the official name of the event if it has one, or a descriptive name (like
    /// "Main Artist at Place") if not.
    pub name: String,

    /// The type describes what kind of event the event is. The possible values are: Concert,
    /// Festival, Launch event, Convention/Expo, Masterclass/Clinic
    // FIXME: Should we create an `EventType` enum and use it here? What do we then do about the
    // values containing slashes, such as "Convention/Expo"?
    // https://musicbrainz.org/ws/2/event/10b5e28b-95f5-482e-9524-c51be37f943d
    #[serde(rename = "type")]
    pub event_type: Option<String>,

    /// The cancelled field describes whether or not the event took place.
    pub cancelled: bool,

    /// The time is the start time of the event.
    pub time: String,

    /// The setlist stores a list of songs performed, optionally including links to artists and works.
    /// See the setlist documentation for syntax and examples.
    // TODO: need some info on that value, current IT test returns ""
    pub setlist: String,

    // same here
    pub tags: Option<Vec<Tag>>,
    pub relations: Option<Vec<Relation>>,

    pub rating: Option<Rating>,
    /// Aliases are alternate names for an event, which currently have two main functions: localised
    /// names and search hints. Localised names are used to store the official names used in different
    /// languages and countries. These use the locale field to identify which language or country the
    /// name is for. Search hints are used to help both users and the server when searching and can
    /// be a number of things including alternate names, nicknames or even misspellings.
    pub aliases: Option<Vec<Alias>>,

    /// Annotations are text fields, functioning like a miniature wiki, that can be added to any
    /// existing artists, labels, recordings, releases, release groups and works.
    pub annotation: Option<String>,

    /// Genres are currently supported in MusicBrainz as part of the tag system.
    pub genres: Option<Vec<Genre>>,

    /// The begin and end dates indicate when an artist started and finished its existence.
    /// Its exact meaning depends on the type of artist:
    ///
    ///  - For a person
    ///        Begin date represents date of birth, and end date represents date of death.
    ///
    ///    - For a group (or orchestra/choir)
    ///        Begin date represents the date when the group first formed: if a group dissolved and then
    ///        reunited, the date is still that of when they first formed. End date represents the date
    ///        when the group last dissolved: if a group dissolved and then reunited, the date is that
    ///        of when they last dissolved (if they are together, it should be blank!). For listing
    ///        other inactivity periods, just use the annotation and the "member of" relationships.
    ///
    ///    - For a character
    ///        Begin date represents the date (in real life) when the character concept was created.
    ///        The End date should not be set, since new media featuring a character can be created
    ///        at any time. In particular, the Begin and End date fields should not be used to hold
    ///        the fictional birth or death dates of a character.
    ///        (This information can be put in the annotation.)
    ///
    ///    - For others
    ///        There are no clear indications about how to use dates for artists of the type Other at
    ///        the moment.
    pub life_span: Option<LifeSpan>,

    /// The disambiguation comments are fields in the database used to help distinguish identically
    /// named artists, labels and other entities. They are visible in the pages for the entities, and
    /// also appear in the search results next to their names.
    pub disambiguation: String,

    pub type_id: Option<String>,
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
    (
        with_artist_relations,
        Include::Relationship(Relationship::Artist)
    ),
    (
        with_place_relations,
        Include::Relationship(Relationship::Place)
    ),
    (
        with_series_relations,
        Include::Relationship(Relationship::Series)
    ),
    (with_url_relations, Include::Relationship(Relationship::Url)),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_ratings, Include::Subquery(Subquery::Rating)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);
