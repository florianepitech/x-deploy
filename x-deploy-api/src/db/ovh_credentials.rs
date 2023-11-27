use bson::oid::ObjectId;

pub(crate) const USER_COLLECTION_NAME: &str = "ovh_credentials";


pub struct OvhCredentials {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub application_key: String,
    pub application_secret: String,
    pub consumer_key: String,
    pub user_id: ObjectId,
}

impl OvhCredentials {
    pub fn new(application_key: String, application_secret: String, consumer_key: String, user_id: ObjectId) -> Self {
        Self {
            id: ObjectId::new(),
            application_key,
            application_secret,
            consumer_key,
            user_id,
        }
    }
}