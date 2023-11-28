pub mod auth;

const API_URL: &str = "http://localhost:8000";

pub struct XDeployClient {
    pub(crate) api_key: Option<String>,
    pub(crate) reqwest_client: reqwest::Client,
}

impl XDeployClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key: Some(api_key),
            reqwest_client: reqwest::Client::new(),
        }
    }

    pub fn new_without_auth() -> Self {
        Self {
            api_key: None,
            reqwest_client: reqwest::Client::new(),
        }
    }

    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = Some(api_key);
    }
}
