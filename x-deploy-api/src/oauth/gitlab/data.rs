use crate::oauth::OAuthUser;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GitLabUser {
  pub email: String,
  pub confirmed_at: Option<String>,
}

impl Into<OAuthUser> for GitLabUser {
  fn into(self) -> OAuthUser {
    OAuthUser::new(self.email)
  }
}
