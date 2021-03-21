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

use crate::Fetch;
use crate::Path;
use crate::{Browse, Search};

pub mod include;

pub mod alias;
pub mod area;
pub mod artist;
pub mod artist_credit;
pub mod browse;
pub mod event;
pub mod genre;
pub mod instrument;
pub mod label;
pub mod lifespan;
pub mod place;
pub mod rating;
pub mod recording;
pub mod relations;
pub mod release;
pub mod release_group;
pub mod search;
pub mod series;
pub mod tag;
pub mod url;
pub mod work;

impl Fetch<'_> for Artist {}
impl Fetch<'_> for Recording {}
impl Fetch<'_> for ReleaseGroup {}
impl Fetch<'_> for Release {}
impl Fetch<'_> for Work {}
impl Fetch<'_> for Label {}
impl Fetch<'_> for Area {}
impl Fetch<'_> for Event {}
impl Fetch<'_> for Instrument {}
impl Fetch<'_> for Place {}
impl Fetch<'_> for Series {}
impl Fetch<'_> for Url {}

impl Browse<'_> for Artist {}
impl Browse<'_> for Recording {}
impl Browse<'_> for ReleaseGroup {}
impl Browse<'_> for Release {}
impl Browse<'_> for Label {}
impl Browse<'_> for Event {}
impl Browse<'_> for Place {}
impl Browse<'_> for Work {}
// impl Browse<'_, url::Include> for Url {}
// impl Browse<'_, series::Include> for Series {}
// impl Browse<'_, instrument::Include> for Instrument {}
// impl Browse<'_, area::Include> for Area {}

impl Search<'_> for Artist {}

impl Path<'_> for Artist {
    fn path() -> &'static str {
        "artist"
    }
}

impl Path<'_> for Recording {
    fn path() -> &'static str {
        "recording"
    }
}

impl Path<'_> for ReleaseGroup {
    fn path() -> &'static str {
        "release-group"
    }
}

impl Path<'_> for Release {
    fn path() -> &'static str {
        "release"
    }
}

impl Path<'_> for Work {
    fn path() -> &'static str {
        "work"
    }
}

impl Path<'_> for Label {
    fn path() -> &'static str {
        "label"
    }
}

impl Path<'_> for Area {
    fn path() -> &'static str {
        "area"
    }
}

impl Path<'_> for Event {
    fn path() -> &'static str {
        "event"
    }
}

impl Path<'_> for Instrument {
    fn path() -> &'static str {
        "instrument"
    }
}

impl Path<'_> for Place {
    fn path() -> &'static str {
        "place"
    }
}

impl Path<'_> for Series {
    fn path() -> &'static str {
        "series"
    }
}

impl Path<'_> for Url {
    fn path() -> &'static str {
        "url"
    }
}
