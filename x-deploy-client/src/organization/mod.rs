pub mod dto;

use crate::error::ClientResult;
use crate::organization::dto::OrganizationInfo;
use crate::{XDeployClient, API_URL};

impl XDeployClient {
  pub async fn get_all_organization(
    &self
  ) -> ClientResult<Vec<OrganizationInfo>> {
    let url = format!("{}/organization", API_URL);
    self.send_get(url).await
  }
}
