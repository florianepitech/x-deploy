mod kbs;
mod db;
mod config;
mod route;
mod cipher;
mod guard;
mod ovh;
mod responder;

use rocket::serde::Deserialize;
use rocket::futures::{StreamExt};
use lazy_static::lazy_static;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::rapidoc::{GeneralConfig, HideShowConfig, make_rapidoc, RapiDocConfig, Theme, UiConfig};
use rocket_okapi::settings::UrlObject;
use crate::config::DotEnvConfig;

extern crate ovh_api;

#[macro_use]
extern crate rocket;

lazy_static! {
    pub(crate) static ref DOTENV_CONFIG: DotEnvConfig = DotEnvConfig::from_dotenv();
}


#[rocket::launch]
async fn rocket() -> _ {
    let mongodb_client = mongodb::Client::with_uri_str(DOTENV_CONFIG.mongodb_url.as_str()).await;
    let mongodb_database = mongodb_client.unwrap()
        .database(DOTENV_CONFIG.mongodb_database.as_str());
    let redis_client = redis::Client::open(DOTENV_CONFIG.redis_url.as_str()).unwrap();

    let catcher_list = catchers![
            responder::not_found,
            responder::unauthorized,
            responder::forbidden,
            responder::internal_server_error,
            responder::unprocessable_entity
        ];

    rocket::build()
        .manage(mongodb_database)
        .manage(redis_client)
        .mount(
            "/docs/",
            make_rapidoc(&RapiDocConfig {
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
            }),
        )
        .register("/", catcher_list)
        .mount("/auth", openapi_get_routes![route::auth::register, route::auth::login, route::auth::info])
        .mount("/", routes![route::ovh::post_credentials])
}
