use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub(crate) const OVH_CRED_COLLECTION_NAME: &str = "ovh_credentials";


pub enum OvhCredentialsStatus {
    Pending,
    Active,
    Inactive,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OvhCredentials {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub application_key: String,
    pub application_secret: String,
    pub consumer_key: String,
    pub status: OvhCredentialsStatus,
    pub user_id: ObjectId,
}


impl OvhCredentials {
    pub fn new(
        application_key: String,
        application_secret: String,
        consumer_key: String,
        user_id: ObjectId,
    ) -> Self {
        Self {
            id: ObjectId::new(),
            application_key,
            application_secret,
            consumer_key,
            user_id,
            status,
        }
    }
}