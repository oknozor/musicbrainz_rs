#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Rating {
    pub vote_count: Option<u32>,
    pub value: Option<f32>,
}
