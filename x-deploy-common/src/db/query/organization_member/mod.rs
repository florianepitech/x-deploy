use crate::db::organization::{Organization, ORGANIZATION_COLLECTION_NAME};
use crate::db::organization_member::{
  OrganizationMember, ORGANIZATION_MEMBER_COLLECTION_NAME,
};
use crate::db::organization_role::{
  OrganizationRole, ORGANIZATION_ROLE_COLLECTION_NAME,
};
use crate::db::query::cursor_doc_to_vec;
use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::CommonResult;
use bson::{doc, oid, Document};
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::{Collection, Database};
use oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationMemberQuery {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "organization")]
  pub organization: Organization,

  #[serde(rename = "user")]
  pub user: User,

  #[serde(rename = "isOwner")]
  pub is_owner: bool,

  #[serde(rename = "role")]
  pub role: Option<OrganizationRole>,
}

impl OrganizationMemberQuery {
  pub fn to_organization_member(&self) -> OrganizationMember {
    return OrganizationMember {
      id: self.id.clone(),
      organization_id: self.organization.id.clone(),
      user_id: self.user.id.clone(),
      is_owner: self.is_owner,
      role: match &self.role {
        Some(role) => Some(role.id.clone()),
        None => None,
      },
    };
  }
}

impl OrganizationMember {
  pub async fn insert(
    &self,
    db: &Database,
  ) -> CommonResult<InsertOneResult> {
    let collection: Collection<OrganizationMember> =
      db.collection(ORGANIZATION_MEMBER_COLLECTION_NAME);
    let result = collection.insert_one(self, None).await?;
    return Ok(result);
  }

  pub async fn delete(
    &self,
    db: &Database,
  ) -> CommonResult<DeleteResult> {
    let collection: Collection<OrganizationMember> =
      db.collection(ORGANIZATION_MEMBER_COLLECTION_NAME);
    let filter = doc! {
      "_id": &self.id,
    };
    let result = collection.delete_one(filter, None).await?;
    return Ok(result);
  }

  pub async fn get_all_user_in_org(
    db: &Database,
    org_id: &ObjectId,
  ) -> CommonResult<Vec<OrganizationMemberQuery>> {
    let collection: Collection<OrganizationMember> =
      db.collection(ORGANIZATION_MEMBER_COLLECTION_NAME);

    let filter = doc! {
      "$match": {
        "organizationId": org_id
      }
    };

    let mut pipeline = default_pipeline();
    pipeline.insert(0, filter);

    let cursor = collection.aggregate(pipeline, None).await?;
    let result: Vec<OrganizationMemberQuery> =
      cursor_doc_to_vec(cursor).await?;
    return Ok(result);
  }

  pub async fn get_all_org_of_user(
    db: &Database,
    user_id: &ObjectId,
  ) -> CommonResult<Vec<OrganizationMemberQuery>> {
    let collection: Collection<OrganizationMember> =
      db.collection(ORGANIZATION_MEMBER_COLLECTION_NAME);

    let filter = doc! {
      "$match": {
        "userId": user_id
      }
    };

    let mut pipeline = default_pipeline();
    pipeline.insert(0, filter);

    let cursor = collection.aggregate(pipeline, None).await?;
    let result: Vec<OrganizationMemberQuery> =
      cursor_doc_to_vec(cursor).await?;
    return Ok(result);
  }

  pub async fn get_user_in_org(
    db: &Database,
    org_id: &ObjectId,
    member_id: &ObjectId,
  ) -> CommonResult<Option<OrganizationMemberQuery>> {
    let collection: Collection<OrganizationMember> =
      db.collection(ORGANIZATION_MEMBER_COLLECTION_NAME);

    let filter = doc! {
      "$match": {
        "organizationId": org_id,
        "userId": member_id
      }
    };

    let mut pipeline = default_pipeline();
    pipeline.insert(0, filter);

    let cursor = collection.aggregate(pipeline, None).await?;
    let result = cursor_doc_to_vec::<OrganizationMemberQuery>(cursor).await?;
    return match result.len() {
      0 => {
        return Ok(None);
      }
      _ => Ok(Some(result[0].clone())),
    };
  }
}

fn default_pipeline() -> Vec<Document> {
  return vec![
    doc! {
      "$lookup": {
        "from": USER_COLLECTION_NAME,
        "localField": "userId",
        "foreignField": "_id",
        "as": "user"
      }
    },
    doc! {
      "$lookup": {
        "from": ORGANIZATION_ROLE_COLLECTION_NAME,
        "localField": "role",
        "foreignField": "_id",
        "as": "role"
      }
    },
    doc! {
      "$lookup": {
        "from": ORGANIZATION_COLLECTION_NAME,
        "localField": "organizationId",
        "foreignField": "_id",
        "as": "organization"
      }
    },
    doc! {
      "$unwind": "$user"
    },
    doc! {
      "$unwind": {
        "path": "$role",
        "preserveNullAndEmptyArrays": true
      }
    },
    doc! {
      "$unwind": "$organization"
    },
  ];
}
