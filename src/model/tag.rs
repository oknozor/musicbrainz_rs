#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub name: String,
    pub count: i32,
}
