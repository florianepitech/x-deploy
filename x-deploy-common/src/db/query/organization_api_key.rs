use crate::db::organization_apikey::OrganizationApiKey;
use crate::db::organization_role::OrganizationRole;
use crate::db::query::cursor_doc_to_vec;
use crate::db::{CommonCollection, ToCollectionName};
use crate::CommonResult;
use bson::doc;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationApiKeyQuery {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,

  #[serde(rename = "key")]
  pub key: String,

  #[serde(rename = "organizationId")]
  pub organization_id: ObjectId,

  #[serde(rename = "role")]
  pub role: Option<OrganizationRole>,

  #[serde(rename = "expiresAt")]
  pub expires_at: Option<bson::DateTime>,
}

impl CommonCollection<OrganizationApiKey> {
  pub async fn get_all_of_org(
    &self,
    org_id: &ObjectId,
  ) -> CommonResult<Vec<OrganizationApiKeyQuery>> {
    let mut pipeline = self.default_pipeline();
    let match_stage = doc! {
      "$match": {
        "organizationId": org_id
      }
    };
    pipeline.insert(0, match_stage);
    let cursor = self.collection.aggregate(pipeline, None).await?;
    let result = cursor_doc_to_vec::<OrganizationApiKeyQuery>(cursor).await?;
    Ok(result)
  }

  pub async fn get_by_id_of_org(
    &self,
    org_id: &ObjectId,
    id: &ObjectId,
  ) -> CommonResult<Option<OrganizationApiKeyQuery>> {
    let mut pipeline = self.default_pipeline();
    let match_stage = doc! {
      "$match": {
        "_id": id,
        "organizationId": org_id,
      }
    };
    pipeline.insert(0, match_stage);
    let cursor = self.collection.aggregate(pipeline, None).await?;
    let mut result =
      cursor_doc_to_vec::<OrganizationApiKeyQuery>(cursor).await?;
    Ok(result.pop())
  }

  pub async fn get_by_value(
    &self,
    value: &str,
  ) -> CommonResult<Option<OrganizationApiKeyQuery>> {
    let mut pipeline = self.default_pipeline();
    let match_stage = doc! {
      "$match": {
        "key": value
      }
    };
    pipeline.insert(0, match_stage);
    let cursor = self.collection.aggregate(pipeline, None).await?;
    let mut result =
      cursor_doc_to_vec::<OrganizationApiKeyQuery>(cursor).await?;
    Ok(result.pop())
  }

  fn default_pipeline(&self) -> Vec<bson::Document> {
    let org_role_name = OrganizationRole::collection_name();
    vec![
      doc! {
        "$lookup": {
          "from": org_role_name,
          "localField": "roleId",
          "foreignField": "_id",
          "as": "role"
        }
      },
      doc! {
        "$unwind": {
          "path": "$role",
          "preserveNullAndEmptyArrays": true
        }
      },
    ]
  }
}
