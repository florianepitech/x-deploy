use crate::auth::AuthFile;
use crate::cmd::{LoginApiKeyArgs, LoginArgs, LoginCredentialsArgs};
use crate::error::CliResult;
use log::{debug, error, info};
use std::process::exit;
use x_deploy_client::auth::dto::LoginRequest;
use x_deploy_client::error::ClientResult;
use x_deploy_client::XDeployClient;

pub async fn login(args: LoginArgs) -> CliResult {
  if AuthFile::is_authenticated() {
    error!("Already authenticated, please logout first");
    exit(1);
  }
  return match args {
    LoginArgs::ApiKey(args) => login_api_key(args).await,
    LoginArgs::Credentials(args) => login_credentials(args).await,
  };
}

async fn login_api_key(args: LoginApiKeyArgs) -> CliResult {
  info!("Login with api key...");
  Ok("Login successful".to_string())
}

async fn login_credentials(args: LoginCredentialsArgs) -> CliResult {
  info!("Login with credentials...");
  let client = XDeployClient::new_without_auth();
  let login_request = LoginRequest {
    email: args.email.clone(),
    password: args.password.clone(),
  };
  let result = client.login(login_request).await?;
  debug!("{:?}", result);
  let token = format!("Bearer {}", result.token);
  AuthFile::new(token).save_to_file();
  Ok("Login successful".to_string())
}
