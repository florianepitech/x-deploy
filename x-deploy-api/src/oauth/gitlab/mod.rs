use crate::error::ApiError;
use crate::oauth::gitlab::data::GitLabUser;
use crate::oauth::OAuthUser;
use crate::CONFIG;
use reqwest::{Method, Url};
use rocket::http::Status;

mod data;

const GITLAB_API_URL: &str = "https://gitlab.com/api/v4";

pub async fn get_user(access_token: String) -> Result<OAuthUser, ApiError> {
  let user = send_get_user(access_token).await?;
  if let None = user.confirmed_at {
    return Err(ApiError::new(
      Status::Forbidden,
      "The email of your GitLab account is not verified".to_string(),
    ));
  }
  let result: OAuthUser = user.into();
  Ok(result)
}

async fn send_get_user(access_token: String) -> Result<GitLabUser, ApiError> {
  let final_url = format!("{}/user", GITLAB_API_URL);
  let final_token = format!("Bearer {}", access_token);
  let url = Url::parse(&final_url).expect("Could not parse URL");
  let client = reqwest::Client::new();
  let request = client
    .request(Method::GET, url)
    .header("Authorization", final_token)
    .header("User-Agent", CONFIG.app_name.clone())
    .build()?;
  let response = client.execute(request).await?;
  let code = response.status().clone();
  let body_str = response.text().await?;
  debug!(
    "Response with code {} and body: {}",
    code.as_str(),
    body_str
  );
  let body: GitLabUser = serde_json::from_str(&body_str)?;
  Ok(body)
}
