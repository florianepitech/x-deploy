use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Display, Formatter};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
  "name": "AWS"
}))]
#[serde(rename_all = "camelCase")]
pub struct CloudProviderResponse {
  pub(crate) name: String,
}
