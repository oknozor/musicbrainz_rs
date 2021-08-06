use crate::entity::area::Area;
use crate::entity::artist::Artist;
use crate::entity::event::Event;
use crate::entity::instrument::Instrument;
use crate::entity::release::Release;
use crate::entity::release_group::ReleaseGroup;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct SearchResult<T> {
    pub created: NaiveDateTime,
    pub count: i32,
    pub offset: i32,
    pub entities: Vec<T>,
}

pub trait Searchable {
    const CREATED_FIELD: &'static str;
    const COUNT_FIELD: &'static str;
    const OFFSET_FIELD: &'static str;
    const ENTITIES_FIELD: &'static str;
}

impl Searchable for Area {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "areas";
}

impl Searchable for Artist {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "artists";
}

impl Searchable for Event {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "events";
}

impl Searchable for Instrument {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "instruments";
}

impl Searchable for Release {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "releases";
}

impl Searchable for ReleaseGroup {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "release-groups";
}
