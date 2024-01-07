use crate::db::organization::{Organization, ORGANIZATION_COLLECTION_NAME};
use crate::db::organization_invitation::{
  InvitationStatus, OrganizationInvitation,
  ORGANIZATION_INVITATION_COLLECTION_NAME,
};
use crate::db::organization_role::{
  OrganizationRole, ORGANIZATION_ROLE_COLLECTION_NAME,
};
use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::CommonResult;
use bson::oid::ObjectId;
use bson::{doc, Bson, Document};
use chrono::{DateTime, Utc};
use futures::StreamExt;
use mongodb::results::{DeleteResult, UpdateResult};
use mongodb::{Collection, Database};
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

impl OrganizationInvitation {
  pub async fn insert(
    &self,
    db: &Database,
  ) -> CommonResult<ObjectId> {
    let collection: Collection<OrganizationInvitation> =
      db.collection(ORGANIZATION_INVITATION_COLLECTION_NAME);
    let result = collection.insert_one(self.clone(), None).await?;
    return Ok(result.inserted_id.as_object_id().unwrap().clone());
  }

  pub async fn delete(
    &self,
    db: &Database,
  ) -> CommonResult<DeleteResult> {
    let collection: Collection<OrganizationInvitation> =
      db.collection(ORGANIZATION_INVITATION_COLLECTION_NAME);
    let filter = doc! {
      "_id": &self.id,
    };
    let result = collection.delete_one(filter, None).await?;
    return Ok(result);
  }

  pub async fn get_all_of_org(
    db: &Database,
    org_id: &ObjectId,
  ) -> CommonResult<Vec<OrganizationInvitationQuery>> {
    let collection: Collection<OrganizationInvitationQuery> =
      db.collection(ORGANIZATION_INVITATION_COLLECTION_NAME);
    let filter = doc! {
      "organizationId": org_id,
    };
    let mut pipeline = default_pipeline();
    pipeline.insert(
      0,
      doc! {
        "$match": filter,
      },
    );
    let mut cursor = collection.aggregate(pipeline, None).await?;
    let mut result: Vec<OrganizationInvitationQuery> = Vec::new();
    while let Some(doc) = cursor.next().await {
      let doc: OrganizationInvitationQuery = bson::from_document(doc?)?;
      result.push(doc);
    }
    return Ok(result);
  }

  pub async fn get_of_user_of_org(
    db: &Database,
    user_id: &ObjectId,
    org_id: &ObjectId,
  ) -> CommonResult<Option<Self>> {
    let collection: Collection<Self> =
      db.collection(ORGANIZATION_INVITATION_COLLECTION_NAME);
    let filter = doc! {
      "receiverId": user_id,
      "organizationId": org_id,
    };
    let result = collection.find_one(filter, None).await?;
    return Ok(result);
  }

  pub async fn find_by_id(
    db: &Database,
    invitation_id: &ObjectId,
  ) -> CommonResult<Option<Self>> {
    let collection: Collection<OrganizationInvitation> =
      db.collection(ORGANIZATION_INVITATION_COLLECTION_NAME);
    let filter = doc! {
      "_id": invitation_id,
    };
    let result = collection.find_one(filter, None).await?;
    return Ok(result);
  }
}

#[deprecated]
pub async fn query_organization_invitation_get_all(
  db: &Database,
  organization_id: &ObjectId,
) -> CommonResult<Vec<OrganizationInvitationQuery>> {
  let collection: Collection<OrganizationInvitationQuery> =
    db.collection(ORGANIZATION_INVITATION_COLLECTION_NAME);
  let filter = doc! {
    "organizationId": organization_id,
  };
  let mut pipeline = default_pipeline();
  pipeline.insert(
    0,
    doc! {
      "$match": filter,
    },
  );
  let mut cursor = collection.aggregate(pipeline, None).await?;
  let mut result: Vec<OrganizationInvitationQuery> = Vec::new();
  while let Some(doc) = cursor.next().await {
    let doc: OrganizationInvitationQuery = bson::from_document(doc?)?;
    result.push(doc);
  }
  return Ok(result);
}

#[deprecated]
pub async fn query_organization_invitation_of_user(
  db: &Database,
  user_id: &ObjectId,
) -> CommonResult<Vec<OrganizationInvitationQuery>> {
  let collection: Collection<OrganizationInvitationQuery> =
    db.collection(ORGANIZATION_INVITATION_COLLECTION_NAME);
  let mut pipeline = default_pipeline();
  pipeline.insert(
    0,
    doc! {
      "$match": doc! {
        "receiverId": user_id,
      },
    },
  );
  let mut cursor = collection.aggregate(pipeline, None).await?;
  let mut result: Vec<OrganizationInvitationQuery> = Vec::new();
  while let Some(doc) = cursor.next().await {
    let doc: OrganizationInvitationQuery = bson::from_document(doc?)?;
    result.push(doc);
  }
  return Ok(result);
}

#[deprecated]
pub async fn query_organization_invitation_by_id(
  db: &Database,
  invitation_id: &ObjectId,
  receiver_id: &ObjectId,
) -> CommonResult<Option<OrganizationInvitationQuery>> {
  let collection: Collection<OrganizationInvitationQuery> =
    db.collection(ORGANIZATION_INVITATION_COLLECTION_NAME);
  let filter = doc! {
    "_id": invitation_id,
    "receiverId": receiver_id,
  };
  let mut pipeline = default_pipeline();
  pipeline.insert(
    0,
    doc! {
      "$match": filter,
    },
  );
  let mut cursor = collection.aggregate(pipeline, None).await?;
  let mut result: Vec<OrganizationInvitationQuery> = Vec::new();
  while let Some(doc) = cursor.next().await {
    let doc: OrganizationInvitationQuery = bson::from_document(doc?)?;
    result.push(doc);
  }
  return Ok(result.pop());
}

#[deprecated]
pub async fn query_organization_invitation_update(
  db: &Database,
  invitation_id: &ObjectId,
  status: &InvitationStatus,
) -> CommonResult<UpdateResult> {
  let collection: Collection<OrganizationInvitationQuery> =
    db.collection(ORGANIZATION_INVITATION_COLLECTION_NAME);
  let response_at: Bson = Bson::DateTime(bson::DateTime::now());
  let status_str = status.to_string();
  let filter = doc! {
    "_id": invitation_id,
  };
  let update = doc! {
    "$set": {
      "status": status_str,
      "responseAt": response_at,
    },
  };
  let result = collection.update_one(filter, update, None).await?;
  return Ok(result);
}

// Private functions

fn default_pipeline() -> Vec<Document> {
  vec![
    doc! {
      "$lookup": {
        "from": ORGANIZATION_COLLECTION_NAME,
        "localField": "organizationId",
        "foreignField": "_id",
        "as": "organization",
      },
    },
    doc! {
      "$lookup": {
        "from": USER_COLLECTION_NAME,
        "localField": "senderId",
        "foreignField": "_id",
        "as": "sender",
      },
    },
    doc! {
      "$lookup": {
        "from": USER_COLLECTION_NAME,
        "localField": "receiverId",
        "foreignField": "_id",
        "as": "receiver",
      },
    },
    doc! {
      "$lookup": {
        "from": ORGANIZATION_ROLE_COLLECTION_NAME,
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
