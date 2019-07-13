use crate::QueryAble;
use crate::model::artist::Artist;
use crate::model::recording::Recording;

pub mod artist;
pub mod recording;

impl QueryAble<'_> for Artist {
    fn path() -> &'static str {
        "artist"
    }
}

impl QueryAble<'_> for Recording{
    fn path() -> &'static str {
        "recording"
    }
}