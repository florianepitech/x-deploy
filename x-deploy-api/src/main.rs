use crate::config::DotEnvConfig;
use lazy_static::lazy_static;
use rocket::futures::StreamExt;
use rocket::serde::Deserialize;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[macro_use]
extern crate rocket;

mod cipher;
mod config;
mod db;
mod error;
mod guard;
mod kbs;
mod ovh;
mod responder;
mod route;
mod event;

extern crate ovh_api;

lazy_static! {
  pub(crate) static ref DOTENV_CONFIG: DotEnvConfig =
    DotEnvConfig::from_dotenv();
}

#[derive(OpenApi)]
#[openapi(
    info(description = "My Api description"),
    paths(
        // Auth
        route::auth::login,
        route::auth::register,
        route::auth::two_factor,
        route::auth::two_factor_recovery,
        // Account
        route::account::get_info,
        route::account::verify_email,
        route::account::change_password,
        route::account::change_phone,
        route::account::info_2fa,
        route::account::setup_2fa,
        route::account::enable_2fa,
        route::account::disable_2fa,
        // Organization
        route::organization::all,
        route::organization::new,
        route::organization::get_by_id,
        route::organization::update,
        route::organization::delete,
        route::organization::transfer,
        // Organization Api Keys
        route::organization::api_key::new,
        route::organization::api_key::get,
        route::organization::api_key::get_by_id,
        route::organization::api_key::delete,
    ),
    components(schemas(
        // Global
        route::SuccessMessage,
        route::ErrorMessage,
        // Auth
        route::auth::dto::LoginBody,
        route::auth::dto::LoginResponse,
        route::auth::dto::RegisterBody,
        // Account
        route::account::dto::GetAccountInfoResponse,
        route::account::dto::VerifyEmailBody,
        route::account::dto::ChangePasswordBody,
        route::account::dto::ChangePhoneBody,
        route::account::dto::TwoFactorSetupRequest,
        route::account::dto::TwoFactorSetupResponse,
        route::account::dto::TwoFactorInfoRequest,
        route::account::dto::TwoFactorInfoResponse,
        route::account::dto::TwoFactorCodeRequest,
        // Organization
        route::organization::dto::CreateOrganizationBody,
        route::organization::dto::TransferOrganizationBody,
        route::organization::dto::OrganizationInfoResponse,
        // Organization Api Keys
        route::organization::api_key::dto::CreateApiKeyBody,
    ))
)]
struct ApiDoc;

#[rocket::launch]
async fn rocket() -> _ {
  let mongodb_client =
    mongodb::Client::with_uri_str(DOTENV_CONFIG.mongodb_url.as_str()).await;
  let mongodb_database = mongodb_client
    .unwrap()
    .database(DOTENV_CONFIG.mongodb_database.as_str());
  let redis_client =
    redis::Client::open(DOTENV_CONFIG.redis_url.as_str()).unwrap();

  // Catchers

  let catcher_list = catchers![
    responder::not_found,
    responder::unauthorized,
    responder::forbidden,
    responder::internal_server_error,
    responder::unprocessable_entity
  ];

  // Routes

  let routes = routes![
    // Auth
    route::auth::register,
    route::auth::login,
    route::auth::two_factor,
    // Account
    route::account::get_info,
    route::account::verify_email,
    route::account::change_password,
    route::account::change_phone,
    route::account::info_2fa,
    route::account::setup_2fa,
    route::account::enable_2fa,
    route::account::disable_2fa,
    // Organization
    route::organization::all,
    route::organization::new,
    route::organization::get_by_id,
    route::organization::update,
    route::organization::delete,
    route::organization::transfer,
    // Organization Api Keys
    route::organization::api_key::new,
    route::organization::api_key::get,
    route::organization::api_key::get_by_id,
    route::organization::api_key::delete,
    // Organization Members
    route::organization::member::get,
    route::organization::member::delete,
    // Organization Project
    route::organization::project::new,
    route::organization::project::get_by_id,
    route::organization::project::update,
    route::organization::project::delete,
    // Organization Aws Credentials
    route::organization::credentials::aws::new,
    route::organization::credentials::aws::get,
    route::organization::credentials::aws::delete,
    // Organization Azure Credentials
    route::organization::credentials::azure::new,
    route::organization::credentials::azure::get,
    route::organization::credentials::azure::delete,
    // Organization Google Cloud Credentials
    route::organization::credentials::google_cloud::new,
    route::organization::credentials::google_cloud::get,
    route::organization::credentials::google_cloud::delete,
    // Organization Ovh Credentials
    route::organization::credentials::ovh::new,
    route::organization::credentials::ovh::get,
    route::organization::credentials::ovh::delete,
  ];

  let doc = ApiDoc::openapi();

  let swagger_ui = SwaggerUi::new("/swagger-ui/<_..>")
    .url("/api-docs/openapi.json", ApiDoc::openapi());

  let redoc_ui = Redoc::with_url("/redoc", ApiDoc::openapi());

  rocket::build()
    .manage(mongodb_database)
    .manage(redis_client)
    .register("/", catcher_list)
    .mount("/", swagger_ui)
    .mount("/", redoc_ui)
    .mount("/", routes)
}
