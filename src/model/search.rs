use chrono::Date;

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct SearchResult<T> {
    pub created:Date,
    pub count: i32,
    pub offset: i32,
    pub entities: Vec<T>,
}