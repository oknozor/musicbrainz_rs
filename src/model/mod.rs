use crate::model::area::Area;
use crate::model::artist::Artist;
use crate::model::label::Label;
use crate::model::recording::Recording;
use crate::model::release::Release;
use crate::model::release_group::ReleaseGroup;
use crate::model::work::Work;
use crate::Include;
use crate::QueryAble;

pub mod alias;
pub mod area;
pub mod artist;
pub mod label;
pub mod lifespan;
pub mod recording;
pub mod relations;
pub mod release;
pub mod release_group;
pub mod work;

impl QueryAble<'_> for Artist {
    fn path() -> &'static str {
        "artist"
    }

    fn get_allowed_includes() -> Vec<Include> {
        vec![
            Include::ArtistRelations,
            Include::Recordings,
            Include::Releases,
            Include::ReleaseGroups,
            Include::Works,
            Include::Aliases,
            Include::Annotations,
        ]
    }
}

impl QueryAble<'_> for Recording {
    fn path() -> &'static str {
        "recording"
    }

    fn get_allowed_includes() -> Vec<Include> {
        vec![Include::Annotations]
    }
}

impl QueryAble<'_> for ReleaseGroup {
    fn path() -> &'static str {
        "release-group"
    }

    fn get_allowed_includes() -> Vec<Include> {
        vec![Include::Annotations]
    }
}

impl QueryAble<'_> for Release {
    fn path() -> &'static str {
        "release"
    }

    fn get_allowed_includes() -> Vec<Include> {
        vec![Include::Annotations]
    }
}

impl QueryAble<'_> for Work {
    fn path() -> &'static str {
        "work"
    }

    fn get_allowed_includes() -> Vec<Include> {
        vec![Include::Annotations]
    }
}

impl QueryAble<'_> for Label {
    fn path() -> &'static str {
        "label"
    }

    fn get_allowed_includes() -> Vec<Include> {
        vec![Include::Annotations]
    }
}

impl QueryAble<'_> for Area {
    fn path() -> &'static str {
        "area"
    }

    fn get_allowed_includes() -> Vec<Include> {
        vec![Include::Annotations]
    }
}
