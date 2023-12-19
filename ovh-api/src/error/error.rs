use std::fmt::Formatter;
use std::string::ParseError;

pub enum OvhClientError {
  SerializeError(serde_json::Error),
  NetworkError(reqwest::Error),
  ParseError(String),
  WrongResponse(reqwest::Response, u16),
}

impl std::fmt::Display for OvhClientError {
  fn fmt(
    &self,
    f: &mut Formatter,
  ) -> std::fmt::Result {
    match self {
      OvhClientError::SerializeError(e) => write!(f, "Serialize error: {}", e),
      OvhClientError::NetworkError(e) => write!(f, "Network error: {}", e),
      OvhClientError::ParseError(e) => write!(f, "Parse error in URL: {}", e),
      OvhClientError::WrongResponse(r, c) => {
        write!(f, "Wrong response: (code: {}) {:?}", c, r)
      }
    }
  }
}
