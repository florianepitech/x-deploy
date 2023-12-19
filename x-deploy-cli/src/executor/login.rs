use crate::auth::AuthFile;
use crate::cmd::LoginArgs;
use std::process::exit;
use x_deploy_client::XDeployClient;

pub fn login(args: LoginArgs) {
  if AuthFile::is_authenticated() {
    println!("ERROR: Already authenticated, please logout first");
    exit(1);
  }
  let valid =
    futures::executor::block_on(verify_credential(&args.email, &args.password));
  if !valid {
    println!("ERROR: Invalid credentials");
    exit(1);
  }
  let credentials_str = format!("{}:{}", args.email, args.password);
  let credentials_base64 = base64::encode(&credentials_str.as_bytes());
  let auth_file = AuthFile::new(credentials_base64);
  auth_file.save_to_file();
  println!("INFO: Successfully authenticated")
}

async fn verify_credential(
  email: &String,
  password: &String,
) -> bool {
  let client = XDeployClient::new_without_auth();
  x_deploy_client::auth::login(&client, email, password)
    .await
    .is_ok()
}
