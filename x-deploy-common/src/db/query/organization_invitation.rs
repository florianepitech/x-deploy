use crate::db::organization::Organization;
use crate::db::organization_invitation::{
  InvitationStatus, OrganizationInvitation,
};
use crate::db::organization_role::OrganizationRole;
use crate::db::query::cursor_doc_to_vec;
use crate::db::user::User;
use crate::db::{CommonCollection, ToCollectionName};
use crate::CommonResult;
use bson::oid::ObjectId;
use bson::{doc, Document};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrganizationInvitationQuery {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "organization")]
  pub organization: Organization,

  #[serde(rename = "sender")]
  pub sender: User,

  #[serde(rename = "receiver")]
  pub receiver: User,

  #[serde(rename = "status")]
  pub status: InvitationStatus,

  #[serde(rename = "responseAt")]
  pub response_at: Option<DateTime<Utc>>,

  #[serde(rename = "role")]
  pub role: OrganizationRole,
}

impl Into<OrganizationInvitation> for OrganizationInvitationQuery {
  fn into(self) -> OrganizationInvitation {
    OrganizationInvitation {
      id: self.id,
      organization_id: self.organization.id,
      sender_id: self.sender.id,
      receiver_id: self.receiver.id,
      status: self.status,
      response_at: self.response_at,
      role: self.role.id,
    }
  }
}

impl CommonCollection<OrganizationInvitation> {
  pub async fn get_all_of_org(
    &self,
    org_id: &ObjectId,
  ) -> CommonResult<Vec<OrganizationInvitationQuery>> {
    let filter = doc! {
      "organizationId": org_id,
    };
    let mut pipeline = Self::default_pipeline();
    pipeline.insert(
      0,
      doc! {
        "$match": filter,
      },
    );
    let cursor = self.collection.aggregate(pipeline, None).await?;
    let result = cursor_doc_to_vec(cursor).await?;
    return Ok(result);
  }

  pub async fn get_by_id_of_org(
    &self,
    org_id: &ObjectId,
    invitation_id: &ObjectId,
  ) -> CommonResult<Option<OrganizationInvitationQuery>> {
    let filter = doc! {
      "organizationId": org_id,
      "_id": invitation_id,
    };
    let mut pipeline = Self::default_pipeline();
    pipeline.insert(
      0,
      doc! {
        "$match": filter,
      },
    );
    let cursor = self.collection.aggregate(pipeline, None).await?;
    let mut result = cursor_doc_to_vec(cursor).await?;
    return Ok(result.pop());
  }

  pub async fn get_of_user(
    &self,
    user_id: &ObjectId,
  ) -> CommonResult<Vec<OrganizationInvitationQuery>> {
    let filter = doc! {
      "receiverId": user_id,
    };
    let mut pipeline = Self::default_pipeline();
    pipeline.insert(
      0,
      doc! {
        "$match": filter,
      },
    );
    let cursor = self.collection.aggregate(pipeline, None).await?;
    let result = cursor_doc_to_vec(cursor).await?;
    return Ok(result);
  }

  pub async fn get_of_user_with_invitation_id(
    &self,
    user_id: &ObjectId,
    invitation_id: &ObjectId,
  ) -> CommonResult<Option<OrganizationInvitationQuery>> {
    let filter = doc! {
      "receiverId": user_id,
      "_id": invitation_id,
    };
    let mut pipeline = Self::default_pipeline();
    pipeline.insert(
      0,
      doc! {
        "$match": filter,
      },
    );
    let cursor = self.collection.aggregate(pipeline, None).await?;
    let mut result = cursor_doc_to_vec(cursor).await?;
    return Ok(result.pop());
  }

  fn default_pipeline() -> Vec<Document> {
    let organization_collection_name = Organization::collection_name();
    let user_collection_name = User::collection_name();
    let organization_role_collection_name = OrganizationRole::collection_name();
    vec![
      doc! {
        "$lookup": {
          "from": organization_collection_name,
          "localField": "organizationId",
          "foreignField": "_id",
          "as": "organization",
        },
      },
      doc! {
        "$lookup": {
          "from": user_collection_name.clone(),
          "localField": "senderId",
          "foreignField": "_id",
          "as": "sender",
        },
      },
      doc! {
        "$lookup": {
          "from": user_collection_name,
          "localField": "receiverId",
          "foreignField": "_id",
          "as": "receiver",
        },
      },
      doc! {
        "$lookup": {
          "from": organization_role_collection_name,
          "localField": "role",
          "foreignField": "_id",
          "as": "role",
        },
      },
      doc! {
        "$unwind": "$organization",
      },
      doc! {
        "$unwind": "$sender",
      },
      doc! {
        "$unwind": "$receiver",
      },
      doc! {
        "$unwind": "$role",
      },
    ]
  }
}
