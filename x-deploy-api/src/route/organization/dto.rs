use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(FromForm, ToSchema, Clone, Debug)]
pub struct GetByIdQuery {
  pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[schema(example = json!({
    "name": "My Stunning Organization",
    "description": "A new amazing organization !",
    "website": "https://my-stunning-organization.net",
    "contact_email": "contact@my-stunning-organization.net",
}))]
pub(crate) struct CreateOrganizationRequest {
  #[serde(rename = "name")]
  pub(crate) name: String,

  #[serde(rename = "description")]
  pub(crate) description: String,

  #[serde(rename = "website")]
  pub(crate) website: String,

  #[serde(rename = "contactEmail")]
  pub(crate) contact_email: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[schema(example = json!({
    "name": "My Stunning Organization",
    "description": "A new amazing organization !",
    "website": "https://my-stunning-organization.net",
    "contact_email": "contact@my-stunning-organization.net",
}))]
pub(crate) struct UpdateOrganizationRequest {
  #[serde(rename = "name")]
  pub(crate) name: String,

  #[serde(rename = "description")]
  pub(crate) description: String,

  #[serde(rename = "website")]
  pub(crate) website: String,

  #[serde(rename = "contactEmail")]
  pub(crate) contact_email: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[schema(example = json!({
    "new_owner_email": "john@doe.net",
    "password": "myAmazingStringPassword123!"
}))]
pub(crate) struct TransferOrganizationRequest {
  #[serde(rename = "newOwnerEmail")]
  pub(crate) new_owner_email: String,

  #[serde(rename = "password")]
  pub(crate) password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[schema(example = json!({
    "password": "myAmazingStringPassword123!"
}))]
pub(crate) struct DeleteOrganizationRequest {
  #[serde(rename = "password")]
  pub(crate) password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[schema(example = json!({
    "id": "5f9b3b3b3b3b3b3b3b3b3b3b",
    "name": "My Stunning Organization",
    "description": "A new amazing organization !",
    "website": "https://my-stunning-organization.net",
    "contact_email": "contact@my-stunning-organization.net",
}))]
pub(crate) struct OrganizationInfoResponse {
  #[serde(rename = "id")]
  pub(crate) id: String,

  #[serde(rename = "name")]
  pub(crate) name: String,

  #[serde(rename = "description")]
  pub(crate) description: String,

  #[serde(rename = "website")]
  pub(crate) website: String,

  #[serde(rename = "contactEmail")]
  pub(crate) contact_email: String,
}
