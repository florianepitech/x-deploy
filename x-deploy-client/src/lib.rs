mod auth;

pub struct XDeployClient {
    pub(crate) api_key: String,
    pub(crate) reqwest_client: reqwest::Client,
}

impl XDeployClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            reqwest_client: reqwest::Client::new(),
        }
    }
}