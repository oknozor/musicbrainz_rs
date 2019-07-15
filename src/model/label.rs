#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Label {
    pub id: String,
    pub type_id: String,
    #[serde(rename = "type")]
    pub label_type: String,
    pub name: String,
    pub sort_name: String,
    pub disambiguation: String,
    pub country: String,
    pub label_code: u32,
}