use crate::model::artist::Artist;
use crate::model::event::Event;

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct BrowseResult<T> {
    pub count: i32,
    pub offset: i32,
    pub entities: Vec<T>,
}

pub trait Browsable {
    const COUNT_FIELD: &'static str;
    const OFFSET_FIELD: &'static str;
    const ENTITIES_FIELD: &'static str;
}

impl Browsable for Artist {
    const COUNT_FIELD: &'static str = "artist-count";
    const OFFSET_FIELD: &'static str = "artist-offset";
    const ENTITIES_FIELD: &'static str = "artists";
}

impl Browsable for Event {
    const COUNT_FIELD: &'static str = "event-count";
    const OFFSET_FIELD: &'static str = "event-offset";
    const ENTITIES_FIELD: &'static str = "events";
}
