use bson::doc;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use crate::db::user::{USER_COLLECTION_NAME, User};
use crate::route::Message;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct LoginBody {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[post("/auth/login", format = "application/json", data = "<body>")]
pub(crate) async fn login(
    db: &State<Database>,
    body: Json<LoginBody>,
) -> Result<Json<Message>, Status> {
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
    if user.password.password != login_body.password {
        return Err(Status::Unauthorized);
    }
    return Ok(Json(Message {
        message: "Login successful".to_string(),
    }));
}