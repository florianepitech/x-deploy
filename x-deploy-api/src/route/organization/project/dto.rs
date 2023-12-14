use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub(crate) struct CreateProjectBody {
    #[serde(rename = "name")]
    pub(crate) name: String,

    #[serde(rename = "description")]
    pub(crate) description: String,

    #[serde(rename = "organizationId")]
    pub(crate) organization_id: String,
}
