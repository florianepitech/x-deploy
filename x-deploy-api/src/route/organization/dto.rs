use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(FromForm, JsonSchema, Debug)]
pub struct GetByIdQuery {
    pub id: String,
}

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
