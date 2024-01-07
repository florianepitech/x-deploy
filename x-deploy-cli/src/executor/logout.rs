use crate::auth::AuthFile;
use crate::error::CliResult;
use std::process::exit;

pub fn logout() -> CliResult {
  if !AuthFile::is_authenticated() {
    panic!("ERROR: Not authenticated, please login first");
  }
  AuthFile::delete_file();
  Ok("Successfully logged out".to_string())
}
