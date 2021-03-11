use reqwest::header;
use reqwest::Client;
use reqwest::RequestBuilder;
use std::sync::Arc;
use std::sync::Mutex;

pub struct MusicBrainzClient(Arc<Mutex<Client>>);

lazy_static! {
    pub static ref HTTP_CLIENT: MusicBrainzClient = init_http_client();
}

impl MusicBrainzClient {
    pub fn get(&self, path: &str) -> RequestBuilder {
        let client_ref = Arc::clone(&HTTP_CLIENT.0);
        let client_lock = client_ref.lock().expect("Unable to get musicbrainz client");
        client_lock.get(path)
    }
}

fn init_http_client() -> MusicBrainzClient {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("musicbrainz_rs default"),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build().expect("Unable to set default user agent, the following values must be set in Cargo.toml : 'name', 'version', 'authors'");

    MusicBrainzClient {
        0: Arc::new(Mutex::new(client)),
    }
}

pub fn set_user_agent(user_agent: &'static str) {
    let client_ref = Arc::clone(&HTTP_CLIENT.0);
    let mut client_lock = client_ref.lock().expect("Unable to set musicbrainz client");

    let mut headers = header::HeaderMap::new();

    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(user_agent),
    );

    *client_lock = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Unable to set user agent");
}

pub const BASE_URL: &str = "http://musicbrainz.org/ws/2";
pub const FMT_JSON: &str = "?fmt=json";
pub const PARAM_INC: &str = "&inc=";
