use crate::error::ApiError;
use crate::oauth::google::data::GoogleTokenInfo;
use crate::oauth::OAuthUser;
use crate::CONFIG;
use reqwest::{Method, Url};
use rocket::http::Status;

mod data;

const GOOGLE_API_URL: &str = "https://www.googleapis.com/oauth2/v2";

pub async fn get_user(access_token: String) -> Result<OAuthUser, ApiError> {
  let user = send_token_info(access_token).await?;
  if !user.verified_email {
    return Err(ApiError::new(
      Status::Forbidden,
      "The email of your Google account is not verified".to_string(),
    ));
  }
  let result: OAuthUser = user.into();
  Ok(result)
}

async fn send_token_info(
  access_token: String
) -> Result<GoogleTokenInfo, ApiError> {
  let final_url = format!("{}/tokeninfo", GOOGLE_API_URL);
  let url = Url::parse(&final_url).expect("Could not parse URL");

  let client = reqwest::Client::new();
  let request = client
    .request(Method::GET, url)
    .query(&[("access_token", access_token)])
    .header("User-Agent", CONFIG.app_name.as_str())
    .build()?;
  let response = client.execute(request).await?;
  let code = response.status().clone();
  let body_str = response.text().await?;
  debug!(
    "Response with code {} and body: {}",
    code.as_str(),
    body_str
  );
  let result: GoogleTokenInfo = serde_json::from_str(&body_str)?;
  Ok(result)
}
