
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use std::string::String;
use std::sync::Arc;
use rocket::futures::{FutureExt, stream, StreamExt};

// use rocket_okapi::{openapi, openapi_get_routes};
// use rocket_okapi::okapi::schemars;
// use rocket_okapi::okapi::schemars::JsonSchema;
// use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
// use rocket_okapi::rapidoc::{make_rapidoc, RapiDocConfig, GeneralConfig, HideShowConfig, LayoutConfig, UiConfig, Theme};
// use rocket_okapi::settings::UrlObject;
use ovh_api::OvhClient;
use ovh_api::data::Project;

extern crate ovh_api;

#[macro_use] extern crate rocket;

#[derive(Deserialize,Serialize)]
#[serde(crate = "rocket::serde")]
struct Cluster {
    name: String,
    description: String,
    version: String,
    status: String,
}




#[get("/clusters")]
fn get_clusters() -> &'static str {

    "Hello, world!"
}



#[post("/clusters", format = "application/json", data="<cluster>")]
fn create_cluster(cluster: Json<Cluster>) -> &'static str {
    "Hello, world!"
}


#[get("/projects")]
async fn get_projects() -> Json<Vec<Project>> {
    let client = Arc::new(OvhClient::new(
        "".to_string(),
        "".to_string(),
        "".to_string()
    ));
    let projets_id : Vec<String> = ovh_api::route::cloud::get_project_list(&client).await.unwrap();
    let projets: Vec<Project> = stream::iter(projets_id)
        .then(|id| {
            let client_clone = client.clone(); // Clone the Arc here
            async move {
                ovh_api::route::cloud::get_project_info(&client_clone, &id).await
            }
        })
        .filter_map(|result| async move {
            match result {
                Ok(project) => Some(project),
                Err(_) => None, // or handle the error as you see fit
            }
        })
        .collect()
        .await;

    Json(projets)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/",  routes![get_clusters, create_cluster, get_projects])
}