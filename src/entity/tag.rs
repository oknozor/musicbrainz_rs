#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Tag {
    pub name: String,
    pub count: i32,
}
