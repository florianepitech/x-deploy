use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub (self) struct Auth {
    pub (self) app_key: String,
    pub (self) app_secret: String,
    pub (self) consumer_key: String,
}

