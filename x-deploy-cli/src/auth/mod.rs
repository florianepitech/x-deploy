use crate::error::CliResult;
use keyring::Entry;

const KEYRING_SERVICE: &str = "x-deploy-cli";
const KEYRING_USER: &str = "x-deploy-cli";

#[derive(Debug)]
pub(crate) struct AuthFile {
  pub token: String,
}

impl AuthFile {
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
    let entry = Self::get_keyring_entry()?;
    let token = entry.get_password()?;
    Ok(Self::new(token))
  }

  fn get_keyring_entry() -> CliResult<Entry> {
    Ok(Entry::new(KEYRING_SERVICE.clone(), KEYRING_USER.clone())?)
  }
}
