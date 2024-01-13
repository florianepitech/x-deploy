use crate::auth::Auth;
use crate::error::CliResult;
use log::info;
use x_deploy_client::organization::dto::OrganizationInfo;
use x_deploy_client::XDeployClient;

pub async fn organization() -> CliResult<String> {
  let auth = Auth::load()?;
  let client: XDeployClient = auth.into();
  let result = client.get_all_organization().await?;
  let size = result.len();
  for (i, organization) in result.iter().enumerate() {
    info!("({}) Organization: {}", i + 1, organization.name);
    info!("- id: {}", organization.id);
    info!("- description: {}", organization.description);
    info!("- website: {}", organization.website);
    info!("- contact email: {}", organization.contact_email);
  }
  Ok(format!("{} organization(s) found", size))
}
