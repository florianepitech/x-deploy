mod kbs;

use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use std::string::String;
use std::sync::Arc;
use rocket::futures::{FutureExt, stream, StreamExt};
use ovh_api::data::kbs_cluster::KbsCluster;


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




#[get("/clusters/<project_id>")]
async fn get_clusters(project_id: &str) -> Json<Vec<KbsCluster>> {
    let client = Arc::new(OvhClient::new(
        std::env::var("OVH_APPLICATION_KEY").expect("OVH_APPLICATION_KEY not found"),
        std::env::var("OVH_APPLICATION_SECRET").expect("OVH_APPLICATION_SECRET not found"),
        std::env::var("OVH_CONSUMER_KEY").expect("OVH_CONSUMER_KEY not found"),
    ));
    let clusters_id: Vec<String> = ovh_api::route::cloud::get_list_cluster_kbs(&client, project_id).await.unwrap();
    let clusters: Vec<KbsCluster> = stream::iter(clusters_id)
        .then(|id| {
            let client_clone = client.clone(); // Clone the Arc here
            async move {
                ovh_api::route::cloud::get_cluster_kbs_info(&client_clone, project_id, &id).await
            }
        })
        .filter_map(|result| async move {
            match result {
                Ok(cluster) => Some(cluster),
                Err(_) => None, // or handle the error as you see fit
            }
        })
        .collect()
        .await;

    Json(clusters)
}



#[post("/clusters", format = "application/json", data="<cluster>")]
fn create_cluster(cluster: Json<Cluster>) -> &'static str {
    "Hello, world!"
}


#[get("/projects")]
async fn get_projects() -> Json<Vec<Project>> {

    let client = Arc::new(OvhClient::new(
        std::env::var("OVH_APPLICATION_KEY").expect("OVH_APPLICATION_KEY not found"),
        std::env::var("OVH_APPLICATION_SECRET").expect("OVH_APPLICATION_SECRET not found"),
        std::env::var("OVH_CONSUMER_KEY").expect("OVH_CONSUMER_KEY not found"),
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
    dotenv::dotenv().ok();
    rocket::build()
        .mount("/",  routes![get_clusters, create_cluster, get_projects])
}