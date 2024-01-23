use crate::error::ApiError;
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub mod github;

pub struct OAuth;

impl OAuth {
  pub async fn get_user(
    service: OAuthService,
    access_token: String,
  ) -> Result<OAuthUser, ApiError> {
    return match service {
      OAuthService::Github => github::get_user(access_token).await,
    };
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum OAuthService {
  #[serde(rename = "GITHUB")]
  Github,
}

impl FromStr for OAuthService {
  type Err = ApiError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "GITHUB" => Ok(Self::Github),
      _ => Err(ApiError::new(
        Status::BadRequest,
        "Invalid OAuth service".to_string(),
      )),
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OAuthUser {
  pub email: String,
}

impl OAuthUser {
  pub fn new(email: String) -> Self {
    Self { email }
  }
}
