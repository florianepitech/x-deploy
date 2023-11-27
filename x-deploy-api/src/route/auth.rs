use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct LoginBody {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[post("/auth/login", format = "application/json", data = "<body>")]
pub(crate) fn login(body: Json<LoginBody>) {

}