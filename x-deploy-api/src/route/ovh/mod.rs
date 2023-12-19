mod dto;

use crate::db::ovh_credentials::{
  OvhCredentials, OvhCredentialsStatus, OVH_CRED_COLLECTION_NAME,
};
use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::guard::token::Token;
use crate::ovh::auth::test_ovh_connection;
use crate::route::Message;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::{Collection, Database};
use ovh_api::OvhClient;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;

///Add ovh credentials to a the user
#[post("/ovh/credentials", format = "application/json", data = "<body>")]
pub async fn post_credentials(
  db: &State<Database>,
  token: Token,
  body: Json<dto::Auth>,
) -> Result<Json<Message>, Custom<Json<Message>>> {
  let auth_body = body.into_inner();
  let client = OvhClient::new(
    auth_body.application_key,
    auth_body.application_secret,
    auth_body.consumer_key,
  );
  // if (!test_ovh_connection(&client).await) {
  //     return Err(Custom(
  //         Status::Forbidden,
  //         Json(Message {
  //             message: "Credentials are incorrect".to_string(),
  //         }),
  //     ));
  // }

  let mongodb_client = db.inner();
  let collection: Collection<OvhCredentials> =
    mongodb_client.collection(OVH_CRED_COLLECTION_NAME);
  let object_id = ObjectId::from_str(token.id.as_str());
  if object_id.is_err() {
    return Err(Custom(
      Status::BadRequest,
      Json(Message {
        message: "Malformed objectId in your token.".to_string(),
      }),
    ));
  }
  let user_id = object_id.unwrap();
  let ovh_credentials = OvhCredentials::new(
    client.application_key,
    client.application_secret,
    client.consumer_key,
    user_id,
    OvhCredentialsStatus::Active,
  );
  collection.insert_one(ovh_credentials, None).await.unwrap();

  Ok(Json(Message {
    message: "Credentials added".to_string(),
  }))
}

//delete ovh credentials with the id
#[delete("/ovh/credentials/<id>")]
pub async fn delete_credentials(
  db: &State<Database>,
  token: Token,
  id: String,
) -> Result<Json<Message>, Custom<Json<Message>>> {
  let mongodb_client = db.inner();
  let collection: Collection<OvhCredentials> =
    mongodb_client.collection(OVH_CRED_COLLECTION_NAME);

  // Convert token ID to ObjectId and handle any error
  let user_id = match ObjectId::from_str(&token.id) {
    Ok(oid) => oid,
    Err(_) => {
      return Err(Custom(
        Status::BadRequest,
        Json(Message {
          message: "Malformed objectId in your token.".to_string(),
        }),
      ))
    }
  };

  // Convert credential ID to ObjectId and handle any error
  let credential_id = match ObjectId::from_str(&id) {
    Ok(oid) => oid,
    Err(_) => {
      return Err(Custom(
        Status::BadRequest,
        Json(Message {
          message: "Invalid credentials ID.".to_string(),
        }),
      ))
    }
  };

  // Perform the deletion
  let delete_result = collection
    .delete_one(
      doc! {
          "_id": credential_id,
          "user_id": user_id
      },
      None,
    )
    .await;

  // Check the outcome of delete operation
  match delete_result {
    Ok(delete_response) => {
      if delete_response.deleted_count == 0 {
        Err(Custom(
          Status::NotFound,
          Json(Message {
            message: "Credentials not found or not belonging to the user."
              .to_string(),
          }),
        ))
      } else {
        Ok(Json(Message {
          message: "Credentials deleted.".to_string(),
        }))
      }
    }
    Err(_) => Err(Custom(
      Status::InternalServerError,
      Json(Message {
        message: "Internal server error occurred.".to_string(),
      }),
    )),
  }
}

//get ovh credentials with the id
#[get("/ovh/credentials/<credential_id>")]
pub async fn get_credentials(
  db: &State<Database>,
  token: Token,
  credential_id: String,
) -> Result<Json<OvhCredentials>, Custom<Json<Message>>> {
  let mongodb_client = db.inner();
  let collection: Collection<OvhCredentials> =
    mongodb_client.collection(OVH_CRED_COLLECTION_NAME);

  collection
    .find_one(
      doc! {
          "_id": ObjectId::from_str(&credential_id).unwrap(),
          "user_id": ObjectId::from_str(&token.id).unwrap()
      },
      None,
    )
    .await
    .map(|cred| match cred {
      Some(cred) => Ok(Json(cred)),
      None => Err(Custom(
        Status::NotFound,
        Json(Message {
          message: "Credentials not found or not belonging to the user."
            .to_string(),
        }),
      )),
    })
    .unwrap()
}
