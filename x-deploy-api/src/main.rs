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
mod guard;
mod kbs;
mod ovh;
mod responder;
mod route;

extern crate ovh_api;

lazy_static! {
    pub(crate) static ref DOTENV_CONFIG: DotEnvConfig = DotEnvConfig::from_dotenv();
}

#[derive(OpenApi)]
#[openapi(
    info(description = "My Api description"),
    paths(
        // Auth
        route::auth::login,
        route::auth::register,
        // Account
        route::account::get_info,
        route::account::verify_email,
        route::account::change_password,
        route::account::change_phone,
        // Organization
        route::organization::new,
        route::organization::get_by_id,
        route::organization::update,
        route::organization::delete,
        route::organization::transfer,
    ),
    components(schemas(
        // Global
        route::Message,
        // Auth
        route::auth::dto::LoginBody,
        route::auth::dto::LoginResponse,
        route::auth::dto::RegisterBody,
        route::auth::dto::AccountInfo,
        // Account
        route::account::dto::GetAccountInfoResponse,
        route::account::dto::VerifyEmailBody,
        route::account::dto::ChangePasswordBody,
        route::account::dto::ChangePhoneBody,
        // Organization
        route::organization::dto::CreateOrganizationBody,
        route::organization::dto::TransferOrganizationBody,
    ))
)]
struct ApiDoc;

#[rocket::launch]
async fn rocket() -> _ {
    let mongodb_client = mongodb::Client::with_uri_str(DOTENV_CONFIG.mongodb_url.as_str()).await;
    let mongodb_database = mongodb_client
        .unwrap()
        .database(DOTENV_CONFIG.mongodb_database.as_str());
    let redis_client = redis::Client::open(DOTENV_CONFIG.redis_url.as_str()).unwrap();

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
        route::auth::info,
        // Account
        route::account::get_info,
        route::account::verify_email,
        route::account::change_password,
        route::account::change_phone,
        // Account Api Keys
        route::account::api_key::new,
        route::account::api_key::get,
        route::account::api_key::get_by_id,
        route::account::api_key::delete,
        // Organization
        route::organization::new,
        route::organization::get_by_id,
        route::organization::update,
        route::organization::delete,
        route::organization::transfer,
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
