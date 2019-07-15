use crate::model::artist::Artist;
use crate::model::label::Label;
use crate::model::recording::Recording;
use crate::model::release::Release;
use crate::model::release_group::ReleaseGroup;
use crate::model::work::Work;
use crate::QueryAble;
use crate::model::area::Area;

pub mod artist;
pub mod recording;
pub mod release_group;
pub mod release;
pub mod work;
pub mod label;
pub mod area;
pub mod lifespan;

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

impl QueryAble<'_> for Release {
    fn path() -> &'static str {
        "release"
    }
}

impl QueryAble<'_> for Work {
    fn path() -> &'static str {
        "work"
    }
}

impl QueryAble<'_> for Label {
    fn path() -> &'static str {
        "label"
    }
}

impl QueryAble<'_> for Area {
    fn path() -> &'static str {
        "area"
    }
}


