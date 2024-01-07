pub mod dto;

use crate::auth::dto::{LoginRequest, LoginResponse};
use crate::error::ClientResult;
use crate::{XDeployClient, API_URL};

impl XDeployClient {
  pub async fn login(
    &self,
    body: LoginRequest,
  ) -> ClientResult<LoginResponse> {
    let url = format!("{}/auth/login", API_URL);
    let response = self.send_post(url, body).await?;
    Ok(response)
  }
}
