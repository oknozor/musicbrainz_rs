use crate::entity::artist::Artist;
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

impl Searchable for Artist {
    const CREATED_FIELD: &'static str = "created";
    const COUNT_FIELD: &'static str = "count";
    const OFFSET_FIELD: &'static str = "offset";
    const ENTITIES_FIELD: &'static str = "artists";
}
