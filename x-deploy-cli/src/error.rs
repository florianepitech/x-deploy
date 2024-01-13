use keyring::Error;
use x_deploy_client::error::ClientError;
use Error::{
  Ambiguous, BadEncoding, Invalid, NoEntry, NoStorageAccess, PlatformFailure,
  TooLong,
};

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
pub struct CliError {
  pub message: String,
}

impl CliError {
  pub fn new(message: String) -> Self {
    Self { message }
  }
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

impl From<Error> for CliError {
  fn from(error: Error) -> Self {
    let result = match error {
      Ambiguous(_) => CliError {
        message: "Ambiguous keyring error".to_string(),
      },
      BadEncoding(_) => CliError {
        message: "The keyring contains invalid data".to_string(),
      },
      NoStorageAccess(_) => CliError {
        message: "The cli does not have access to the keyring".to_string(),
      },
      PlatformFailure(_) => {
        CliError {
          message: "Your platform is not supported, please use the environment variable to authenticate".to_string(),
        }
      }
      NoEntry => {
        CliError {
          message: "You are not authenticated, please login first".to_string(),
        }
      }
      TooLong(_, _) => {
        CliError {
          message: "The token is too long, please use the environment variable to authenticate".to_string(),
        }
      }
      Invalid(_, _) => {
        CliError {
          message: "The token is invalid, please use the environment variable to authenticate".to_string(),
        }
      }
      _ => {
        CliError {
          message: "Unknown error with your credentials, please use the environment variable to authenticate".to_string(),
        }
      }
    };
    result
  }
}
