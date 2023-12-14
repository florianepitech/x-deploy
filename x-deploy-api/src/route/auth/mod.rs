use crate::cipher::password::verify_password;
use crate::cipher::token::{gen_new_token, Token};
use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::route::auth::dto::{AccountInfo, LoginBody, LoginResponse, RegisterBody};
use crate::route::Message;
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

pub mod dto;

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "Auth",
    responses(
        (status = 200, description = "You're now logged in")
    ),
    request_body = LoginBody,
)]
#[post("/auth/login", format = "application/json", data = "<body>")]
pub(crate) async fn login(
    db: &State<Database>,
    body: Json<LoginBody>,
) -> Result<Json<LoginResponse>, Custom<Json<Message>>> {
    let login_body = body.into_inner();
    let mongodb_client = db.inner();
    let collection: Collection<User> = mongodb_client.collection(USER_COLLECTION_NAME);
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
    let valid_password = verify_password(&login_body.password, user.password.password.as_str());
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
    let new_token =
        gen_new_token(user.id.clone(), &duration, &jwt_secret).expect("Error generating token");
    return Ok(Json(LoginResponse { token: new_token }));
}

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Auth",
    responses(
        (status = 200, description = "You're now registred")
    ),
)]
#[post("/auth/register", format = "application/json", data = "<body>")]
pub(crate) async fn register(
    db: &State<Database>,
    body: Json<RegisterBody>,
) -> Result<Json<Message>, Custom<Json<Message>>> {
    let body = body.into_inner();
    let mongodb_client = db.inner();
    let collection: Collection<User> = mongodb_client.collection(USER_COLLECTION_NAME);
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
    let password_hash = crate::cipher::password::hash_password(body.password.as_str());
    let new_user: User = User::new(
        body.firstname.clone(),
        body.lastname.clone(),
        password_hash,
        body.email.clone(),
        body.password.clone(),
    );
    collection.insert_one(new_user, None).await.unwrap();
    return Ok(Json(Message {
        message: "You are now registered".to_string(),
    }));
}

#[get("/auth/info")]
pub(crate) async fn info(
    db: &State<Database>,
    token: Token,
) -> Result<Json<AccountInfo>, Custom<Json<Message>>> {
    let mongodb_client = db.inner();
    let collection: Collection<User> = mongodb_client.collection(USER_COLLECTION_NAME);
    let object_id = ObjectId::from_str(token.id.as_str());
    if object_id.is_err() {
        return Err(Custom(
            Status::BadRequest,
            Json(Message {
                message: "Malformed objectId in your token.".to_string(),
            }),
        ));
    }
    let user = collection
        .find_one(
            doc! {
                "_id": object_id.unwrap()
            },
            None,
        )
        .await
        .unwrap();
    if user.is_none() {
        return Err(Custom(
            Status::NotFound,
            Json(Message {
                message: "User not found with this token".to_string(),
            }),
        ));
    }
    let user = user.unwrap();
    return Ok(Json(AccountInfo {
        firstname: user.firstname.clone(),
        lastname: user.lastname.clone(),
        email: user.email.email.clone(),
        phone: user.phone.phone.clone(),
    }));
}
