use crate::model::alias::Alias;
use crate::model::area::Area;
use crate::model::lifespan::LifeSpan;
use crate::model::recording::Recording;
use crate::model::relations::Relation;
use crate::model::release::Release;
use crate::model::release_group::ReleaseGroup;
use crate::model::work::Work;
use crate::Include as IncludeInto;

/// An artist is generally a musician (or musician persona), group of musicians, or other music
/// professional (like a producer or engineer). Occasionally, it can also be a non-musical person
/// (like a photographer, an illustrator, or a poet whose writings are set to music), or even a
/// fictional character. For some other special cases, see special purpose artists.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Artist {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,

    /// The official name of an artist, be it a person or a band.
    pub name: String,

    /// The sort name is a variant of the artist name which would be used when sorting artists by
    /// name, such as in record shops or libraries. Among other things, sort names help to ensure
    /// that all the artists that start with "The" don't end up up under "T". The guidelines for
    /// sort names are the best place to check for more specific usage info.
    pub sort_name: String,

    pub disambiguation: String,

    /// The type is used to state whether an artist is a person, a group, or something else.
    #[serde(rename = "type")]
    pub artist_type: Option<ArtistType>,

    /// The gender is used to explicitly state whether a person or character identifies as male,
    /// female or neither. Groups do not have genders.
    pub gender: Option<Gender>,

    /// The artist area, as the name suggests, indicates the area with which an artist is primarily
    /// identified with. It is often, but not always, its birth/formation country.
    pub area: Option<Area>,

    ///The artist begin area, as the name suggests, indicates the area with which an artist started
    /// to perform.

    #[serde(rename = "begin_area")] // Forcing camel_case here since
    pub begin_area: Option<Area>, // all other field are deserialized in kebab-case

    pub relations: Option<Vec<Relation>>,
    pub releases: Option<Vec<Release>>,
    pub works: Option<Vec<Work>>,
    pub release_groups: Option<Vec<ReleaseGroup>>,
    pub recordings: Option<Vec<Recording>>,
    pub aliases: Option<Vec<Alias>>,

    pub country: Option<String>,

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

    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ArtistType {
    /// This indicates an individual person.
    Person,
    /// This indicates a group of people that may or may not have a distinctive name.
    Group,
    /// This indicates an orchestra (a large instrumental ensemble).
    Orchestra,
    /// This indicates a choir/chorus (a large vocal ensemble).
    Choir,
    /// This indicates an individual fictional character.
    Character,
    /// Anything which does not fit into the above categories.
    Other,
}

/// The gender is used to explicitly state whether a person or character identifies as male,
/// female or neither. Groups do not have genders.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub name: String,
    pub count: u32,
}

#[derive(Debug, PartialEq)]
pub enum Include {
    ArtistRelations,
    Releases,
    ReleaseGroups,
    Recordings,
    Aliases,
    Works,
}

impl IncludeInto<Artist> for Include {
    fn as_str(&self) -> &str {
        match self {
            Include::Recordings => "recordings",
            Include::Releases => "releases",
            Include::ReleaseGroups => "release-groups",
            Include::Aliases => "aliases",
            Include::Works => "works",
            Include::ArtistRelations => "artist-rels",
        }
    }
}
