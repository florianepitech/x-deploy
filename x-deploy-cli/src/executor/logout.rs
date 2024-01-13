use crate::auth::AuthFile;
use crate::error::CliResult;
use std::process::exit;

pub fn logout() -> CliResult<String> {
  let auth = AuthFile::load()?;
  auth.delete()?;
  Ok("Successfully logged out".to_string())
}
