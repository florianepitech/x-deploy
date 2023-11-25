use reqwest::{Error, Response, Url};

mod route;
mod data;

pub(crate) const OVH_BASE_URL: &str = "https://api.ovh.com/1.0";

pub struct OvhClient {
    application_key: String,
    application_secret: String,
    consumer_key: String,
    reqwest_client: reqwest::Client,
}

impl OvhClient {
    pub fn new(
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
        let url: Url = Url::parse(&url).unwrap();
        let base_request = reqwest::Request::new(reqwest::Method::GET, url);
        let request = reqwest::RequestBuilder::from_parts(self.reqwest_client.clone(), base_request)
            .header("X-Ovh-Application", self.application_key.as_str())
            .header("X-Ovh-Consumer", self.consumer_key.as_str())
            .build()
            .unwrap();
        self.reqwest_client.execute(request).await
    }
}