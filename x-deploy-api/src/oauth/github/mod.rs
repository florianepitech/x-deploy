use crate::error::ApiError;
use crate::oauth::github::data::GitHubEmail;
use crate::oauth::OAuthUser;
use reqwest::header::HeaderName;
use reqwest::{Method, Url};
use rocket::http::Status;

mod data;

const GITHUB_API_URL: &str = "https://api.github.com";

pub async fn get_user(access_token: String) -> Result<OAuthUser, ApiError> {
  let email = send_get_email(access_token).await?;
  let user = GitHubEmail::get_primary_email(email);
  match user {
    None => Err(ApiError::new(
      Status::InternalServerError,
      "Could not get primary email from GitHub".to_string(),
    )),
    Some(user) => {
      let result: OAuthUser = user.into();
      Ok(result)
    }
  }
}

async fn send_get_email(
  access_token: String
) -> Result<Vec<GitHubEmail>, ApiError> {
  let final_url = format!("{}/user/emails", GITHUB_API_URL);
  let url = Url::parse(&final_url).expect("Could not parse URL");
  let client = reqwest::Client::new();
  let request = client
    .request(Method::GET, url)
    .header(HeaderName::from_static("Authorization"), access_token)
    .header(
      HeaderName::from_static("Accept"),
      "application/vnd.github.v3+json",
    )
    .header(
      HeaderName::from_static("X-GitHub-Api-Version"),
      "2022-11-28",
    )
    .build()?;
  let response = client.execute(request).await?;
  let body_str = response.text().await?;
  let body: Vec<GitHubEmail> = serde_json::from_str(&body_str)?;
  Ok(body)
}
