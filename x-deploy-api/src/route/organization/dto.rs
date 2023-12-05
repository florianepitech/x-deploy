use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
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