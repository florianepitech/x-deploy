use crate::auth::Auth;
use crate::error::CliResult;
use std::process::exit;

pub fn logout() -> CliResult<String> {
  let auth = Auth::load()?;
  auth.delete()?;
  Ok("Successfully logged out".to_string())
}
