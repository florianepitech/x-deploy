use crate::config::DotEnvConfig;
use lazy_static::lazy_static;
use rocket::futures::StreamExt;
use rocket::serde::Deserialize;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::rapidoc::{
    make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig, Theme, UiConfig,
};
use rocket_okapi::settings::UrlObject;

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

    // Rapidoc

    let rapid_doc_config = make_rapidoc(&RapiDocConfig {
        general: GeneralConfig {
            spec_urls: vec![UrlObject::new("General", "../openapi.json")],
            ..Default::default()
        },
        hide_show: HideShowConfig {
            allow_spec_url_load: false,
            allow_spec_file_load: false,
            ..Default::default()
        },
        ui: UiConfig {
            theme: Theme::Dark,
            ..Default::default()
        },
        ..Default::default()
    });

    // Routes

    let routes = openapi_get_routes![
        // Auth
        route::auth::register,
        route::auth::login,
        route::auth::info,
        // Account
        route::account::get_info,
        route::account::verify_email,
        route::account::change_password,
        route::account::change_phone,
        // Organization
        route::organization::new,
        route::organization::get_by_id,
        // Project
        route::project::new,
        route::project::get_by_id,
        // Organization Ovh Credentials
        route::organization::ovh::new,
        route::organization::ovh::get,
        route::organization::ovh::delete,
        // Organization Aws Credentials
        route::organization::aws::new,
        route::organization::aws::get,
        route::organization::aws::delete,
        // Organization Azure Credentials
        route::organization::azure::new,
        route::organization::azure::get,
        route::organization::azure::delete,
        // Organization Google Cloud Credentials
        route::organization::gc::new,
        route::organization::gc::get,
        route::organization::gc::delete,
    ];

    rocket::build()
        .manage(mongodb_database)
        .manage(redis_client)
        .register("/", catcher_list)
        .mount("/docs", rapid_doc_config)
        .mount("/", routes)
}
