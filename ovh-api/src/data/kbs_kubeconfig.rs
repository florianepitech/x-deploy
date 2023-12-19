use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct KubeConfig {
  #[serde(rename = "content")]
  pub content: String,
}
