use crate::error::error::OvhClientError;
use crate::OvhClient;
use reqwest::{Error, RequestBuilder, Response, Url};
use sha1::Digest;
use sha1::Sha1;

impl OvhClient {
  pub(crate) async fn send_get_mapped<T>(
    &self,
    url: &str,
  ) -> Result<serde_json::Value, OvhClientError>
  where
    T: serde::de::DeserializeOwned,
  {
    let url_str = url.to_string();
    let url = Url::parse(&url);
    // Check url error
    let url: Url = match url {
      Ok(u) => u,
      Err(e) => return Err(OvhClientError::ParseError(url_str)),
    };
    let timestamp: i64 = chrono::Utc::now().timestamp();
    let signature = self.create_request_signature(
      "GET",
      url.to_string().as_str(),
      "",
      &timestamp.to_string(),
    );
    let base_request = reqwest::Request::new(reqwest::Method::GET, url);
    let request =
      RequestBuilder::from_parts(self.reqwest_client.clone(), base_request)
        .header("X-Ovh-Application", self.application_key.as_str())
        .header("X-Ovh-Consumer", self.consumer_key.as_str())
        .header("X-Ovh-Signature", signature.as_str())
        .header("X-Ovh-Timestamp", timestamp.to_string().as_str())
        .build();
    return match request {
      Ok(r) => self.send_mapped(url_str.as_str()).await,
      Err(e) => Err(OvhClientError::NetworkError(e)),
    };
  }

  pub(crate) async fn send_mapped<T>(
    &self,
    url: &str,
  ) -> Result<T, OvhClientError>
  where
    T: serde::de::DeserializeOwned,
  {
    let response: Result<Response, Error> = self.send_get_request(url).await;
    // Check network error
    let response: Response = match response {
      Ok(r) => r,
      Err(e) => return Err(OvhClientError::NetworkError(e)),
    };
    // Check response status code
    let response_status_code: u16 = response.status().as_u16();
    if response_status_code != 200 {
      return Err(OvhClientError::WrongResponse(
        response,
        response_status_code,
      ));
    }
    let response_body: reqwest::Result<String> = response.text().await;
    // Check network error
    let response_body: String = match response_body {
      Ok(r) => r,
      Err(e) => return Err(OvhClientError::NetworkError(e)),
    };
    let response_body = serde_json::from_str(&response_body);
    // Check serialize error
    let response_body: T = match response_body {
      Ok(r) => r,
      Err(e) => return Err(OvhClientError::SerializeError(e)),
    };
    Ok(response_body)
  }

  fn create_request_signature(
    &self,
    method: &str,
    query: &str,
    body: &str,
    timestamp: &String,
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
