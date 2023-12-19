use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(FromForm, ToSchema, Debug)]
pub struct GetByIdQuery {
  pub id: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "name": "My Stunning Organization",
    "description": "A new amazing organization !",
    "website": "https://my-stunning-organization.net",
    "contact_email": "contact@my-stunning-organization.net",
}))]
pub(crate) struct CreateOrganizationBody {
  #[serde(rename = "name")]
  pub(crate) name: String,

  #[serde(rename = "description")]
  pub(crate) description: String,

  #[serde(rename = "website")]
  pub(crate) website: String,

  #[serde(rename = "contactEmail")]
  pub(crate) contact_email: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "new_owner_email": "john@doe.net",
    "password": "myAmazingStringPassword123!"
}))]
pub(crate) struct TransferOrganizationBody {
  #[serde(rename = "newOwnerEmail")]
  pub(crate) new_owner_email: String,

  #[serde(rename = "password")]
  pub(crate) password: String,
}
