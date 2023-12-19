use serde::{Deserialize, Serialize};

const AUTH_FILE_PATH: &str = "./auth.json";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AuthFile {
  pub token: String,
}

impl AuthFile {
  pub(crate) fn new(token: String) -> Self {
    Self { token }
  }

  pub(crate) fn is_authenticated() -> bool {
    std::path::Path::new(AUTH_FILE_PATH).exists()
  }

  pub(crate) fn from_file() -> Self {
    let content = std::fs::read_to_string(AUTH_FILE_PATH)
      .expect("Something went wrong reading the file");
    serde_json::from_str::<AuthFile>(&content)
      .expect("Error while parsing json")
  }

  pub(crate) fn save_to_file(&self) {
    let json = serde_json::to_string_pretty(&self).unwrap();
    std::fs::write(AUTH_FILE_PATH, json).expect("Unable to write file");
  }

  pub(crate) fn delete_file() {
    std::fs::remove_file(AUTH_FILE_PATH).expect("Unable to delete file");
  }
}
