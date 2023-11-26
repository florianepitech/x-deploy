use reqwest::{Error, Response, Url};
use sha1::{Sha1, Digest};
use sha1::digest::Output;

pub mod route;
pub mod data;

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
        let timestamp: i64 = chrono::Utc::now().timestamp();
        let signature = self.create_signature(
            "GET",
            url.to_string().as_str(),
            "",
            &timestamp.to_string(),
        );
        println!("Signature: {}", signature);
        let base_request = reqwest::Request::new(reqwest::Method::GET, url);
        let request = reqwest::RequestBuilder::from_parts(self.reqwest_client.clone(), base_request)
            .header("X-Ovh-Application", self.application_key.as_str())
            .header("X-Ovh-Consumer", self.consumer_key.as_str())
            .header("X-Ovh-Signature", signature.as_str())
            .header("X-Ovh-Timestamp", timestamp.to_string().as_str())
            .build()
            .unwrap();
        self.reqwest_client.execute(request).await
    }

    fn create_signature(
        &self,
        method: &str,
        query: &str,
        body: &str,
        timestamp: &String
    ) -> String {
        let data = format!(
            "{}+{}+{}+{}+{}+{}",
            self.application_secret,
            self.consumer_key,
            method,
            query,
            body,
            timestamp.as_str()
        );
        let mut hasher = Sha1::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        format!("$1${}", hex::encode(result))
    }
}