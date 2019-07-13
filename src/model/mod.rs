use crate::QueryAble;
use crate::model::artist::Artist;
use crate::model::recording::Recording;
use crate::model::release_group::ReleaseGroup;

pub mod artist;
pub mod recording;
pub mod release_group;

impl QueryAble<'_> for Artist {
    fn path() -> &'static str {
        "artist"
    }
}

impl QueryAble<'_> for Recording {
    fn path() -> &'static str {
        "recording"
    }
}

impl QueryAble<'_> for ReleaseGroup {
    fn path() -> &'static str {
        "release-group"
    }
}