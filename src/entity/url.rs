use super::{Include, Relationship};
use crate::entity::tag::Tag;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Url {
    pub id: String,
    pub resource: String,
    pub tags: Option<Vec<Tag>>,
}

impl_includes!(
    Url,
    (
        with_artist_relations,
        Include::Relationship(Relationship::Artist)
    ),
    (with_url_relations, Include::Relationship(Relationship::Url))
);
