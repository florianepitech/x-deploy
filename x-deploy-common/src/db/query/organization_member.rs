use crate::db::organization::Organization;
use crate::db::organization_member::OrganizationMember;
use crate::db::organization_role::OrganizationRole;
use crate::db::query::cursor_doc_to_vec;
use crate::db::user::User;
use crate::db::{CommonCollection, ToCollectionName};
use crate::CommonResult;
use bson::{doc, oid, Document};
use mongodb::Collection;
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

impl CommonCollection<OrganizationMember> {
  pub async fn get_user_in_org(
    &self,
    org_id: &ObjectId,
    member_id: &ObjectId,
  ) -> CommonResult<Option<OrganizationMemberQuery>> {
    let collection: Collection<OrganizationMember> = self.collection.clone();

    let filter = doc! {
      "$match": {
        "organizationId": org_id,
        "userId": member_id
      }
    };

    let mut pipeline = Self::default_pipeline();
    pipeline.insert(0, filter);

    let cursor = collection.aggregate(pipeline, None).await?;
    let result = cursor_doc_to_vec::<OrganizationMemberQuery>(cursor).await?;
    let first = result.first();
    return match first {
      Some(first) => Ok(Some(first.clone())),
      None => Ok(None),
    };
  }

  pub async fn get_all_user_in_org(
    &self,
    org_id: &ObjectId,
  ) -> CommonResult<Vec<OrganizationMemberQuery>> {
    let collection: Collection<OrganizationMember> = self.collection.clone();

    let filter = doc! {
      "$match": {
        "organizationId": org_id
      }
    };

    let mut pipeline = Self::default_pipeline();
    pipeline.insert(0, filter);

    let cursor = collection.aggregate(pipeline, None).await?;
    let result: Vec<OrganizationMemberQuery> =
      cursor_doc_to_vec(cursor).await?;
    return Ok(result);
  }

  pub async fn get_all_org_of_user(
    &self,
    user_id: &ObjectId,
  ) -> CommonResult<Vec<OrganizationMemberQuery>> {
    let collection: Collection<OrganizationMember> = self.collection.clone();

    let filter = doc! {
      "$match": {
        "userId": user_id
      }
    };

    let mut pipeline = Self::default_pipeline();
    pipeline.insert(0, filter);

    let cursor = collection.aggregate(pipeline, None).await?;
    let result: Vec<OrganizationMemberQuery> =
      cursor_doc_to_vec(cursor).await?;
    return Ok(result);
  }

  fn default_pipeline() -> Vec<Document> {
    let user_collection_name = User::collection_name();
    let organization_collection_name = Organization::collection_name();
    let organization_role_collection_name = OrganizationRole::collection_name();
    return vec![
      doc! {
        "$lookup": {
          "from": user_collection_name,
          "localField": "userId",
          "foreignField": "_id",
          "as": "user"
        }
      },
      doc! {
        "$lookup": {
          "from": organization_role_collection_name,
          "localField": "role",
          "foreignField": "_id",
          "as": "role"
        }
      },
      doc! {
        "$lookup": {
          "from": organization_collection_name,
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
}

impl Into<OrganizationMember> for OrganizationMemberQuery {
  fn into(self) -> OrganizationMember {
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
