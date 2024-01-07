use x_deploy_client::error::ClientError;

pub type CliResult = Result<String, CliError>;

#[derive(Debug)]
pub struct CliError {
  pub message: String,
}

impl From<ClientError> for CliError {
  fn from(error: ClientError) -> Self {
    match error {
      ClientError::NetworkError(_) => CliError {
        message: "A network error occurred, verify your internet connection or try again later"
          .to_string(),
      },
      ClientError::ParseError(_) => CliError {
        message: "Error while parsing json response from server".to_string(),
      },
      ClientError::ApiError(message) => CliError { message },
    }
  }
}
