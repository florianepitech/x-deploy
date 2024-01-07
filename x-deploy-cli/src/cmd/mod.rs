use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CmdArgs {
  #[clap(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  /// Manage applications
  Application,
  /// Log in
  #[clap(subcommand)]
  Login(LoginArgs),
  /// Organization
  Organization,
  /// Log out
  Logout,
  /// Generate the autocompletion script for the specified shell
  Completion {
    // You can add parameters specific to each command here
  },
  /// Opens the application in Console in your browser
  Console,
  /// Manage containers
  Container,
  /// Manage CLI context
  Context,
  /// Manage cronjobs
  Cronjob,
  /// Manage databases
  Database,
  /// Manage Environment Variables and Secrets
  Env,
  /// Manage environments
  Environment,
  /// Manage lifecycle jobs
  Lifecycle,
  /// Print your application logs
  Log,
  /// Manage services
  Service,
  /// Connect to an application container
  Shell,
  /// Print the status of your application
  Status,
  /// Generate an API token
  Token,
  /// Upgrade CLI to latest version
  Upgrade,
  /// Print installed version of the CLI
  Version,
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
