use crate::model::area::Area;
use crate::model::artist::Artist;
use crate::model::event::Event;
use crate::model::instrument::*;
use crate::model::label::Label;
use crate::model::place::Place;
use crate::model::recording::Recording;
use crate::model::release::Release;
use crate::model::release_group::ReleaseGroup;
use crate::model::series::Series;
use crate::model::url::Url;
use crate::model::work::Work;

use crate::QueryAble;

pub mod alias;
pub mod area;
pub mod artist;
pub mod event;
pub mod instrument;
pub mod label;
pub mod lifespan;
pub mod place;
pub mod recording;
pub mod relations;
pub mod release;
pub mod release_group;
pub mod series;
pub mod url;
pub mod work;

impl QueryAble<'_, artist::Include> for Artist {
    fn path() -> &'static str {
        "artist"
    }
}

impl QueryAble<'_, recording::Include> for Recording {
    fn path() -> &'static str {
        "recording"
    }
}

impl QueryAble<'_, release_group::Include> for ReleaseGroup {
    fn path() -> &'static str {
        "release-group"
    }
}

impl QueryAble<'_, release::Include> for Release {
    fn path() -> &'static str {
        "release"
    }
}

impl QueryAble<'_, work::Include> for Work {
    fn path() -> &'static str {
        "work"
    }
}

impl QueryAble<'_, label::Include> for Label {
    fn path() -> &'static str {
        "label"
    }
}

impl QueryAble<'_, area::Include> for Area {
    fn path() -> &'static str {
        "area"
    }
}

impl QueryAble<'_, event::Include> for Event {
    fn path() -> &'static str {
        "event"
    }
}

impl QueryAble<'_, instrument::Include> for Instrument {
    fn path() -> &'static str {
        "instrument"
    }
}

impl QueryAble<'_, place::Include> for Place {
    fn path() -> &'static str {
        "place"
    }
}

impl QueryAble<'_, series::Include> for Series {
    fn path() -> &'static str {
        "series"
    }
}

impl QueryAble<'_, url::Include> for Url {
    fn path() -> &'static str {
        "url"
    }
}
