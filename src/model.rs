/// An artist is generally a musician (or musician persona), group of musicians, or other music professional (like a producer or engineer).
///  Occasionally, it can also be a non-musical person (like a photographer, an illustrator, or a poet whose writings are set to music),
/// or even a fictional character. For some other special cases, see special purpose artists.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Artist {
    pub id: String,
    pub name: String,
    #[serde(rename = "sort-name")]
    pub sort_name: String,
    pub disambiguation: String,
    #[serde(rename = "type")]
    pub artist_type: Type,
    pub gender: Option<Gender>,
    // pub area: Area,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Type {
    Group,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Gender {}
#[derive(Debug, Serialize, Deserialize, PartialEq)] 
pub struct Area {}