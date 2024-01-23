use crate::oauth::OAuthUser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTokenInfo {
  pub email: String,
  pub verified_email: bool,
}

impl Into<OAuthUser> for GoogleTokenInfo {
  fn into(self) -> OAuthUser {
    OAuthUser::new(self.email)
  }
}
