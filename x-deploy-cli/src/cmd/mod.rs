use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CmdArgs {
  #[clap(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  /// Log in
  #[clap(subcommand)]
  Login(LoginArgs),
  /// Log out
  Logout,
  /// Organization
  Organization,
}

#[derive(Subcommand, Debug)]
pub enum LoginArgs {
  /// Log in using API key
  ApiKey(LoginApiKeyArgs),
  /// Log in using credentials
  Credentials(LoginCredentialsArgs),
}

#[derive(Parser, Debug)]
pub struct LoginApiKeyArgs {
  /// API key
  #[clap(short, long)]
  pub key: String,
}

#[derive(Parser, Debug)]
pub struct LoginCredentialsArgs {
  /// Email
  #[clap(short, long)]
  pub email: String,

  #[clap(short, long)]
  /// Password
  pub password: String,
}
