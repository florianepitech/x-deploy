use crate::error::CliResult;
use keyring::Entry;
use x_deploy_client::XDeployClient;

const KEYRING_SERVICE: &str = "x-deploy-cli";
const KEYRING_USER: &str = "x-deploy-cli";

#[derive(Debug)]
pub(crate) struct Auth {
  pub token: String,
}

impl Auth {
  pub fn new(token: String) -> Self {
    Self { token }
  }

  pub fn save(&self) -> CliResult<()> {
    let entry = Self::get_keyring_entry()?;
    entry.set_password(&self.token)?;
    Ok(())
  }

  pub fn delete(&self) -> CliResult<()> {
    let entry = Self::get_keyring_entry()?;
    entry.delete_password()?;
    Ok(())
  }

  pub fn load() -> CliResult<Self> {
    let env = std::env::var("XDEPLOY_APIKEY");
    if let Ok(token) = env {
      return Ok(Self::new(token));
    }
    let entry = Self::get_keyring_entry()?;
    let token = entry.get_password()?;
    Ok(Self::new(token))
  }

  fn get_keyring_entry() -> CliResult<Entry> {
    Ok(Entry::new(KEYRING_SERVICE.clone(), KEYRING_USER.clone())?)
  }
}

impl Into<XDeployClient> for Auth {
  fn into(self) -> XDeployClient {
    XDeployClient::new(self.token)
  }
}
