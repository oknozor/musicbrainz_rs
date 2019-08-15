lazy_static! {
    pub static ref HTTP_CLIENT: reqwest::Client = { reqwest::Client::new() };
}

pub const BASE_URL: &str = "http://musicbrainz.org/ws/2";
pub const FMT_JSON: &str = "?fmt=json";
pub const PARAM_INC: &str = "&inc=";
