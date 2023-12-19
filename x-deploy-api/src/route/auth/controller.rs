use crate::cipher::password::verify_password;
use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::guard::token::gen_new_token;
use crate::route::auth::dto::{
  LoginBody, LoginResponse, RegisterBody, TwoFactorCode,
};
use crate::route::{custom_message, custom_response, CustomResponse, Message};
use crate::DOTENV_CONFIG;
use bson::doc;
use bson::oid::ObjectId;
use k8s_openapi::chrono;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;

pub(crate) async fn login(
  db: &State<Database>,
  body: Json<LoginBody>,
) -> CustomResponse<LoginResponse> {
  let login_body = body.into_inner();
  let mongodb_client = db.inner();
  let collection: Collection<User> =
    mongodb_client.collection(USER_COLLECTION_NAME);
  // Verify if email exists for an user
  let user = collection
    .find_one(
      doc! {
          "email.email": login_body.email
      },
      None,
    )
    .await
    .unwrap();
  if user.is_none() {
    return Err(Custom(
      Status::Unauthorized,
      Json(Message {
        message: "Email or password is incorrect".to_string(),
      }),
    ));
  }
  let user = user.unwrap();
  // Verify if password is correct
  let valid_password =
    verify_password(&login_body.password, user.password.password.as_str());
  if !valid_password {
    return Err(Custom(
      Status::Unauthorized,
      Json(Message {
        message: "Email or password is incorrect".to_string(),
      }),
    ));
  }
  let duration = chrono::Duration::hours(24);
  let jwt_secret = DOTENV_CONFIG.jwt_secret.clone();
  let new_token = gen_new_token(user.id.clone(), &duration, &jwt_secret, None)
    .expect("Error generating token");
  let response = LoginResponse { token: new_token };
  custom_response(Status::Ok, response)
}

pub(crate) async fn two_factor_auth(
  db: &State<Database>,
  body: TwoFactorCode,
) -> CustomResponse<Message> {
  custom_message(
    Status::NotImplemented,
    "Two factor authentication is not implemented yet.",
  )
}

pub(crate) async fn register(
  db: &State<Database>,
  body: Json<RegisterBody>,
) -> CustomResponse<Message> {
  let body = body.into_inner();
  let mongodb_client = db.inner();
  let collection: Collection<User> =
    mongodb_client.collection(USER_COLLECTION_NAME);
  // Verify if email exists for an user
  let user = collection
    .find_one(
      doc! {
          "email.email": body.email.clone()
      },
      None,
    )
    .await
    .unwrap();
  if user.is_some() {
    return Err(Custom(
      Status::Conflict,
      Json(Message {
        message: "Email already exists".to_string(),
      }),
    ));
  }
  let password_hash =
    crate::cipher::password::hash_password(body.password.as_str());
  let new_user: User = User::new(
    body.firstname.clone(),
    body.lastname.clone(),
    password_hash,
    body.email.clone(),
    body.password.clone(),
  );
  collection.insert_one(new_user, None).await.unwrap();
  return custom_message(Status::Created, "You are now registered");
}

pub(crate) async fn two_factor(
  db: &State<Database>,
  body: Json<TwoFactorCode>,
) -> CustomResponse<Message> {
  custom_message(
    Status::NotImplemented,
    "Two factor authentication is not implemented yet.",
  )
}
