use reqwest::{Error, Response};

mod route;
mod data;

pub(crate) const OVH_BASE_URL: &str = "https://api.ovh.com/1.0";

struct OvhClient {
    application_key: String,
    application_secret: String,
    consumer_key: String,
    reqwest_client: reqwest::Client,
}

impl OvhClient {
    fn new(
        application_key: String,
        application_secret: String,
        consumer_key: String,
    ) -> Self {
        Self {
            application_key,
            application_secret,
            consumer_key,
            reqwest_client: reqwest::Client::new(),
        }
    }

    pub(crate) async fn send_get_request(
        &self,
        url: &str,
    ) -> Result<Response, Error> {
        let url = reqwest::Url::parse(&url).unwrap();
        let request = reqwest::Request::new(reqwest::Method::GET, url);
        self.reqwest_client.execute(request).await
    }
}