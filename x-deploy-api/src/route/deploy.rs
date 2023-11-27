use rocket::serde::json::Json;
use crate::kbs::deploy::DeployInfo;
use crate::kbs::deploy::deploy;

#[post("/clusters/deploy", format = "application/json", data = "<deployment>")]
pub fn deploy_post(deployment: Json<DeployInfo>) -> &'static str {
    return deploy(deployment.into_inner());
}