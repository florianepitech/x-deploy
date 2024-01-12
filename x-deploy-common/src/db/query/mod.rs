use crate::CommonError;
use bson::Document;
use futures::StreamExt;
use serde::de::DeserializeOwned;

mod organization;
pub mod organization_invitation;
pub mod organization_member;

pub(crate) async fn cursor_doc_to_vec<T>(
  mut cursor: mongodb::Cursor<Document>
) -> Result<Vec<T>, CommonError>
where
  T: DeserializeOwned,
{
  let mut result: Vec<T> = Vec::new();
  while let Some(doc) = cursor.next().await {
    let result_doc = doc?;
    let result_doc: T = bson::from_document(result_doc)?;
    result.push(result_doc);
  }
  return Ok(result);
}

pub(crate) async fn cursor_to_vec<T>(
  mut cursor: mongodb::Cursor<T>
) -> Result<Vec<T>, CommonError>
where
  T: DeserializeOwned,
{
  let mut result: Vec<T> = Vec::new();
  while let Some(doc) = cursor.next().await {
    let result_doc = doc?;
    result.push(result_doc);
  }
  return Ok(result);
}
