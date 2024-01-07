use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Display, Formatter};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize)]
pub enum CloudProviderType {
  #[serde(rename = "AWS")]
  Aws,

  #[serde(rename = "GOOGLE_CLOUD")]
  GoogleCloud,

  #[serde(rename = "AZURE")]
  Azure,

  #[serde(rename = "OVH")]
  Ovh,
}

impl Display for CloudProviderType {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      CloudProviderType::Aws => write!(f, "AWS"),
      CloudProviderType::GoogleCloud => write!(f, "GOOGLE_CLOUD"),
      CloudProviderType::Azure => write!(f, "AZURE"),
      CloudProviderType::Ovh => write!(f, "OVH"),
    }
  }
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
  "name": "AWS"
}))]
#[serde(rename_all = "camelCase")]
pub struct CloudProviderResponse {
  pub(crate) name: String,
}
