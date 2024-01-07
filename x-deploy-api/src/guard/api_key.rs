use bson::oid::ObjectId;

pub struct ApiKey {
  pub(crate) id: String,
  pub(crate) exp: i64,
}

impl ApiKey {
  pub fn new(
    id: ObjectId,
    exp: i64,
  ) -> Self {
    Self {
      id: id.to_string(),
      exp,
    }
  }

  pub fn encode() {
    todo!()
  }

  pub fn decode() -> Self {
    todo!()
  }
}
