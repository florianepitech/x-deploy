pub type ClientResult<T> = Result<T, ClientError>;

pub enum ClientError {
  ApiError(String),
  NetworkError(reqwest::Error),
  ParseError(serde_json::Error),
}

impl From<reqwest::Error> for ClientError {
  fn from(error: reqwest::Error) -> Self {
    Self::NetworkError(error)
  }
}

impl From<serde_json::Error> for ClientError {
  fn from(error: serde_json::Error) -> Self {
    Self::ParseError(error)
  }
}