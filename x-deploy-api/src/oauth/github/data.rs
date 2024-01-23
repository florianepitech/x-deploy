use crate::oauth::OAuthUser;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitHubEmail {
  pub email: String,
  pub verified: bool,
  pub primary: bool,
  pub visibility: String,
}

impl GitHubEmail {
  pub fn get_primary_email(emails: Vec<GitHubEmail>) -> Option<GitHubEmail> {
    for email in emails {
      if email.primary {
        return Some(email);
      }
    }
    None
  }
}

pub struct GitHubUser {
  email: String,
}

impl Into<OAuthUser> for GitHubEmail {
  fn into(self) -> OAuthUser {
    OAuthUser::new(self.email)
  }
}
