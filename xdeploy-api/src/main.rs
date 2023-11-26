
use rocket::serde::{Deserialize, json::Json};
use rocket_okapi::{openapi, openapi_get_routes};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::rapidoc::{make_rapidoc, RapiDocConfig, GeneralConfig, HideShowConfig};
use rocket_okapi::settings::UrlObject;
use ovh_api::OvhClient;

extern crate ovh_api;

#[macro_use] extern crate rocket;

#[derive(Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct Cluster {
    name: String,
    description: String,
    version: String,
    status: String,
}

struct Project {
    name: String,
    description: String,
    version: String,
    status: String,
}

#[openapi(tag = "Clusters")]
#[get("/clusters")]
fn get_clusters() -> &'static str {
    "Hello, world!"
}


#[openapi(tag = "Clusters")]
#[post("/clusters", format = "application/json", data="<cluster>")]
fn create_cluster(cluster: Json<Cluster>) -> &'static str {
    "Hello, world!"
}


#[openapi(tag = "Projects")]
#[get("/projects")]
fn get_projects() ->  &'static str {
    let client = OvhClient::new(
        "".to_string(),
        "".to_string(),
        "".to_string()
    );
    "Hello, world!"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/",  openapi_get_routes![get_clusters, create_cluster, get_projects])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
        "/rapidoc/",
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
            ..Default::default()
        }),
    )
}