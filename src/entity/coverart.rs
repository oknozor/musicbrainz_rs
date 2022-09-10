use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Coverart {
    pub images: Vec<CoverartImage>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CoverartImage {
    pub approved: bool,
    pub back: bool,
    pub comment: String,
    pub edit: u64,
    pub front: bool,
    pub id: u64,
    pub image: String,
    pub thumbnails: Thumbnail,
    pub types: Vec<ImageType>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Thumbnail {
    /// This is now deprecated in MusicBrainz API. Use `res_250` instead.
    pub small: Option<String>,
    /// khis is now deprecated in MusicBrainz API. Use `res_500` instead.
    pub large: Option<String>,

    #[serde(rename = "1200")]
    pub res_1200: Option<String>,
    #[serde(rename = "500")]
    pub res_500: Option<String>,
    #[serde(rename = "250")]
    pub res_250: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ImageType {
    /// The album cover, this is the front of the packaging of an audio recording (or in the
    /// case of a digital release the image associated with it in a digital media store).
    Front,

    /// The back of the package of an audio recording, this will often contain the track
    /// listing, barcode and copyright information.
    Back,

    /// A small book or group of pages inserted into the compact disc or DVD jewel case or
    /// the equivalent packaging for vinyl records and cassettes.
    Booklet,

    /// The medium contains the audio recording. For a compact disc release it is the compact
    /// disc itself, for a vinyl release it is the vinyl disc itself, etc.
    Medium,

    /// The image behind or on the tray containing the medium.
    Tray,

    /// An obi is a strip of paper around the spine (or occasionally one of the other edges of the packaging).
    Obi,

    /// A spine is the edge of the package of an audio recording, it is often the only part
    /// visible when recordings are stacked or stored in a shelf.
    Spine,

    /// Digital releases sometimes have cover art associated with each individual track of a
    /// release (typically embedded in the .mp3 files), use this type for images associated
    /// with individual tracks.
    Track,

    /// A liner is a protective sleeve surrounding a medium (usually a vinyl record, but sometimes
    /// a CD), often printed with notes or images.
    Liner,

    /// A sticker is an adhesive piece of paper, that is attached to the plastic film or enclosed
    /// inside the packaging.
    Sticker,

    /// A poster included with a release. May be the same size as the packaging or larger
    /// (in this case it would fold out).
    Poster,

    /// A watermark is a piece of text or an image which is not part of the cover art but is
    /// added by the person who scanned the cover art. Images without any watermarks are preferred
    /// where possible - this type is useful in cases where either the only available image is
    /// watermarked, or where a better quality watermarked image is uploaded alongside a poorer
    /// quality non-watermarked image.
    Watermark,

    /// Select this type when uploading images that are usable for reference, but need more work to
    /// be usable for tagging (for example, uncropped scans like the one below).
    Raw,

    /// Anything which doesn't fit in the types defined above.
    Other,
}
