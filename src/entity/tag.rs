#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Tag {
    pub name: String,
    pub count: i32,
}
