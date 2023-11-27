use serde::{Deserialize, Serialize};

const AUTH_FILE_PATH: &str = "./auth.json";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Auth {
    pub token: String,
}

impl Auth {
    pub(crate) fn new(token: String) -> Self {
        Self {
            token,
        }
    }

    pub(crate) fn from_file() -> Self {
        let content = std::fs::read_to_string(AUTH_FILE_PATH).expect("Something went wrong reading the file");
        serde_json::from_str::<Auth>(&content).expect("Error while parsing json")
    }

    pub(crate) fn save_to_file(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        std::fs::write(AUTH_FILE_PATH, json).expect("Unable to write file");
    }
}