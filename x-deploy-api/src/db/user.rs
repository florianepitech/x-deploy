use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,

    #[serde(rename = "firstname")]
    pub firstname: String,

    #[serde(rename = "lastname")]
    pub lastname: String,

    #[serde(rename = "email")]
    pub email: Email,
    
    #[serde(rename = "phone")]
    pub phone: Phone,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Email {
    #[serde(rename = "email")]
    email: String,
    
    #[serde(rename = "verified")]
    verified: bool,
    
    #[serde(rename = "code")]
    code: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Phone {
    #[serde(rename = "phone")]
    phone: String,
    
    #[serde(rename = "verified")]
    verified: bool,
    
    #[serde(rename = "code")]
    code: Option<String>,
}
