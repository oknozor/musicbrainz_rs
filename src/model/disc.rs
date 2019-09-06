#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Disc {
    pub id: String, 
    pub offset_count: u32,
    pub sectors: u32, 
    pub offsets : Vec<u32>
}