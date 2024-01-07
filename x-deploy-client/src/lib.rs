use crate::error::{ClientError, ClientResult};
use reqwest::header::{HeaderMap, HeaderName, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub mod auth;
pub mod error;
pub mod organization;

const API_URL: &str = "http://localhost:8000";

#[derive(Deserialize, Serialize, Clone, Debug)]
struct ErrorMessage {
  error: String,
}

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

  pub fn set_api_key(
    &mut self,
    api_key: String,
  ) {
    self.api_key = Some(api_key);
  }

  pub(crate) async fn send_get<R>(
    &self,
    url: String,
  ) -> ClientResult<R>
  where
    R: DeserializeOwned,
  {
    let mut request = self.reqwest_client.get(url);
    if let Some(api_key) = &self.api_key {
      request = request.header("Authorization", format!("{}", api_key));
    }
    let response = request.send().await?;
    let status = response.status();
    let body = response.text().await?;
    verify_response(status, &body)?;
    let result = serde_json::from_str::<R>(&body)?;
    Ok(result)
  }

  pub(crate) async fn send_post<B, R>(
    &self,
    url: String,
    body: B,
  ) -> ClientResult<R>
  where
    B: Serialize,
    R: DeserializeOwned,
  {
    let mut request = self
      .reqwest_client
      .post(url)
      .header(CONTENT_TYPE.as_str(), "application/json");
    if let Some(api_key) = &self.api_key {
      request = request.header("Authorization", format!("Bearer {}", api_key));
    }
    let body_str = serde_json::to_string(&body)?;
    let response = request.body(body_str).send().await?;
    let status = response.status();
    let body = response.text().await?;
    verify_response(status, &body)?;
    let result = serde_json::from_str::<R>(&body)?;
    Ok(result)
  }
}

fn verify_response(
  status: StatusCode,
  body: &String,
) -> ClientResult<()> {
  if status.is_success() {
    return Ok(());
  }
  let error_message = serde_json::from_str::<ErrorMessage>(&body)?;
  Err(ClientError::ApiError(error_message.error))
}
