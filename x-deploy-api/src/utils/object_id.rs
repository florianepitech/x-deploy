use crate::error::ApiError;
use bson::oid::ObjectId;

#[deprecated]
pub trait ToObjectId {
  #[deprecated]
  fn to_object_id(&self) -> Result<ObjectId, ApiError>;
}

impl ToObjectId for &str {
  fn to_object_id(&self) -> Result<ObjectId, ApiError> {
    match ObjectId::parse_str(self) {
      Ok(id) => Ok(id),
      Err(_) => Err(ApiError::new(
        rocket::http::Status::BadRequest,
        "You provided an id in a wrong format".to_string(),
      )),
    }
  }
}

impl ToObjectId for String {
  fn to_object_id(&self) -> Result<ObjectId, ApiError> {
    return self.as_str().to_object_id();
  }
}
