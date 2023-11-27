use bson::doc;
use k8s_openapi::chrono;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use crate::cipher::password::verify_password;
use crate::cipher::token::gen_new_token;
use crate::db::user::{USER_COLLECTION_NAME, User};
use crate::route::Message;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct LoginBody {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct LoginResponse {
    pub(crate) token: String,
}

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
    let new_token = gen_new_token(
        user.id.clone(),
        &duration,
        &std::env::var("JWT_SECRET").expect("JWT_SECRET not found"),
    ).expect("Error generating token");
    return Ok(Json(LoginResponse {
        token: new_token,
    }));
}