use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(FromForm, ToSchema, Debug)]
pub struct GetByIdQuery {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
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
