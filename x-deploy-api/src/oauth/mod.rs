use crate::error::ApiError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub mod github;
mod gitlab;
mod google;

pub struct OAuth;

impl OAuth {
  pub async fn get_user(
    service: OAuthService,
    access_token: String,
  ) -> Result<OAuthUser, ApiError> {
    return match service {
      OAuthService::Github => github::get_user(access_token).await,
      OAuthService::Google => google::get_user(access_token).await,
      OAuthService::GitLab => gitlab::get_user(access_token).await,
    };
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OAuthService {
  Github,
  Google,
  GitLab,
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
