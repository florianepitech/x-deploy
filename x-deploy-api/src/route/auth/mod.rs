use bson::doc;
use k8s_openapi::chrono;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use crate::cipher::password::verify_password;
use crate::cipher::token::gen_new_token;
use crate::db::user::{USER_COLLECTION_NAME, User};
use crate::DOTENV_CONFIG;
use crate::route::auth::dto::{LoginBody, LoginResponse, RegisterBody};
use crate::route::Message;

pub mod dto;

#[post("/auth/login", format = "application/json", data = "<body>")]
pub(crate) async fn login(
    db: &State<Database>,
    body: Json<LoginBody>,
) -> Result<Json<LoginResponse>, Status> {
    let login_body = body.into_inner();
    let mongodb_client = db.inner();
    let collection: Collection<User> = mongodb_client.collection(USER_COLLECTION_NAME);
    // Verify if email exists for an user
    let user = collection.find_one(
        doc! {
            "email.email": login_body.email
        },
        None,
    ).await.unwrap();
    if user.is_none() {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();
    // Verify if password is correct
    let valid_password = verify_password(user.password.password.as_str(), &login_body.password);
    if !valid_password {
        return Err(Status::Unauthorized);
    }
    let duration = chrono::Duration::hours(24);
    let jwt_secret = DOTENV_CONFIG.jwt_secret.clone();
    let new_token = gen_new_token(
        user.id.clone(),
        &duration,
        &jwt_secret,
    ).expect("Error generating token");
    return Ok(Json(LoginResponse {
        token: new_token,
    }));
}

#[post("/auth/register", format = "application/json", data = "<body>")]
pub(crate) async fn register(
    db: &State<Database>,
    body: Json<RegisterBody>,
) -> Result<Json<Message>, Status> {
    let body = body.into_inner();
    let mongodb_client = db.inner();
    let collection: Collection<User> = mongodb_client.collection(USER_COLLECTION_NAME);
    // Verify if email exists for an user
    let user = collection.find_one(
        doc! {
            "email.email": body.email
        },
        None,
    ).await.unwrap();
    if user.is_some() {
        return Err(Status::Conflict);
    }
    let password_hash = crate::cipher::password::hash_password(body.password.as_str());
    let new_user: User = User::new(
        body.firstname.clone(),
        body.lastname.clone(),
        password_hash,
        body.phone.clone(),
        body.password.clone(),
    );
    collection.insert_one(new_user, None).await.unwrap();
    return Ok(Json(Message {
        message: "You are now registered".to_string(),
    }));
}