use crate::guard::token::Token;
use crate::route::account::dto::GetAccountInfoResponse;
use crate::route::{CustomResult, Message};
use crate::{custom_message};
use bson::doc;
use bson::oid::ObjectId;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use crate::db::user::{User, USER_COLLECTION_NAME};

pub(crate) mod api_key;
pub(crate) mod dto;

#[utoipa::path(
    get,
    path = "/account",
    tag = "Account",
    responses(
        (status = 200, description = "Get account info", body = GetAccountInfoResponse),
    ),
)]
#[get("/account", format = "application/json")]
pub(crate) async fn get_info(
    token: Token,
    db: &State<Database>,
) -> CustomResult<GetAccountInfoResponse> {
    let user_id: ObjectId = ObjectId::parse_str(token.id.as_str()).unwrap();
    let mongodb_client = db.inner();
    let collection: Collection<User> = mongodb_client.collection(USER_COLLECTION_NAME);
    let user = collection
        .find_one(
            doc! {
                "_id": user_id
            },
            None,
        )
        .await
        .unwrap();
    if user.is_none() {
        return custom_message!(Status::NotFound, "You're account doesn't exist !");
    }
    let user = user.unwrap();
    let result = GetAccountInfoResponse {
        firstname: user.firstname,
        lastname: user.lastname,
        email: user.email.email,
        email_verified: user.email.verified,
        phone: user.phone.phone,
    };
    return Ok(Json(result));
}

#[deprecated]
#[utoipa::path(
    post,
    path = "/account/verify-email",
    tag = "Account",
    responses(
        (status = 200, description = "Verify email", body = Message),
    ),
    request_body = VerifyEmailBody,
)]
#[post("/account/verify-email", format = "application/json", data = "<body>")]
pub(crate) async fn verify_email(
    db: &State<Database>,
    body: Json<dto::VerifyEmailBody>,
) -> Result<Json<Message>, Custom<Json<Message>>> {
    Err(Custom(
        Status::NotImplemented,
        Json(Message {
            message: "Not implemented".to_string(),
        }),
    ))
}

#[deprecated]
#[utoipa::path(
    post,
    path = "/account/change-password",
    tag = "Account",
    responses(
        (status = 200, description = "Change password", body = Message),
    ),
    request_body = ChangePasswordBody,
)]
#[post(
    "/account/change-password",
    format = "application/json",
    data = "<body>"
)]
pub(crate) async fn change_password(
    db: &State<Database>,
    body: Json<dto::ChangePasswordBody>,
) -> Result<Json<Message>, Custom<Json<Message>>> {
    Err(Custom(
        Status::NotImplemented,
        Json(Message {
            message: "Not implemented".to_string(),
        }),
    ))
}

#[deprecated]
#[utoipa::path(
    post,
    path = "/account/change-phone",
    tag = "Account",
    responses(
        (status = 200, description = "Change phone", body = Message),
    ),
    request_body = ChangePhoneBody,
)]
#[post("/account/change-phone", format = "application/json", data = "<body>")]
pub(crate) async fn change_phone(
    db: &State<Database>,
    body: Json<dto::ChangePhoneBody>,
) -> Result<Json<Message>, Custom<Json<Message>>> {
    Err(Custom(
        Status::NotImplemented,
        Json(Message {
            message: "Not implemented".to_string(),
        }),
    ))
}
