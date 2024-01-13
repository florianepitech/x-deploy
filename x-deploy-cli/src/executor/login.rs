use crate::auth::Auth;
use crate::cmd::{LoginApiKeyArgs, LoginArgs, LoginCredentialsArgs};
use crate::error::CliResult;
use log::{debug, error, info};
use std::process::exit;
use x_deploy_client::auth::dto::LoginRequest;
use x_deploy_client::error::ClientResult;
use x_deploy_client::XDeployClient;

pub async fn login(args: LoginArgs) -> CliResult<String> {
  if let Ok(_) = Auth::load() {
    error!("You are already logged in, please logout first");
    exit(1);
  }
  return match args {
    LoginArgs::ApiKey(args) => login_api_key(args).await,
    LoginArgs::Credentials(args) => login_credentials(args).await,
  };
}

async fn login_api_key(args: LoginApiKeyArgs) -> CliResult<String> {
  info!("Login with api key...");
  // TODO: Add support for api key login
  Ok("Login successful".to_string())
}

async fn login_credentials(args: LoginCredentialsArgs) -> CliResult<String> {
  info!("Login with credentials...");
  let client = XDeployClient::new_without_auth();
  let login_request = LoginRequest {
    email: args.email.clone(),
    password: args.password.clone(),
  };
  let result = client.login(login_request).await?;
  let token = format!("Bearer {}", result.token);
  Auth::new(token).save()?;
  Ok("Login successful".to_string())
}
