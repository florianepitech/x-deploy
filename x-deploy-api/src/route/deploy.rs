use crate::kbs::deploy::deploy;
use crate::kbs::deploy::DeployInfo;
use rocket::serde::json::Json;

#[post("/clusters/deploy", format = "application/json", data = "<deployment>")]
pub async fn deploy_post(deployment: Json<DeployInfo>) -> &'static str {
    return deploy(deployment.into_inner()).await;
}
