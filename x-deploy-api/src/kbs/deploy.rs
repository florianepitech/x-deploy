use crate::config::DotEnvConfig;
use crate::kbs;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use kube::api::PostParams;
use kube::{Api, Config};
use ovh_api::data::kbs_cluster::KbsCluster;
use ovh_api::data::Project;
use ovh_api::OvhClient;
use rocket::futures::{stream, StreamExt};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use std::string::String;
use std::sync::Arc;

#[derive(Clone, Deserialize)]
pub struct DeployInfo {
    pub(crate) project_id: String,
    pub(crate) cluster_id: String,
    pub(crate) deployment_name: String,
    pub(crate) app_name: String,
    pub(crate) image: String,
    pub(crate) tag: String,
    replicas: u32,
    namespace: String,
}

pub async fn deploy(deployment_info: DeployInfo) -> &'static str {
    let client = Arc::new(OvhClient::new(
        std::env::var("OVH_APPLICATION_KEY").expect("OVH_APPLICATION_KEY not found"),
        std::env::var("OVH_APPLICATION_SECRET").expect("OVH_APPLICATION_SECRET not found"),
        std::env::var("OVH_CONSUMER_KEY").expect("OVH_CONSUMER_KEY not found"),
    ));
    let kubeconfig =
        ovh_api::route::cloud::get_kubconfig(
            &client,
            &deployment_info.project_id,
            &deployment_info.cluster_id,
        )
        .await
        .expect("Error getting kubeconfig");
    let kube_client = kbs::connect_with_kubeconfig(kubeconfig.content.as_str()).await;
    let deployment = create_deployment_type(deployment_info.clone());
    let deployments: Api<Deployment> = Api::namespaced(kube_client, &deployment_info.namespace);
    return match deployments
        .create(&PostParams::default(), &deployment)
        .await
    {
        Ok(_) => "Deployment created",
        Err(_) => "Error creating deployment",
    };
}

pub fn create_deployment_type(args: DeployInfo) -> Deployment {
    Deployment {
        metadata: kube::api::ObjectMeta {
            name: Some(args.deployment_name.as_str().parse().unwrap()),
            ..Default::default()
        },
        spec: Some(k8s_openapi::api::apps::v1::DeploymentSpec {
            replicas: Some(1), // Set the number of replicas
            selector: LabelSelector {
                match_labels: Some(std::collections::BTreeMap::from([(
                    "app".parse().unwrap(),
                    args.app_name.as_str().parse().unwrap(),
                )])),
                ..Default::default()
            },
            template: k8s_openapi::api::core::v1::PodTemplateSpec {
                metadata: Some(kube::api::ObjectMeta {
                    labels: Some(std::collections::BTreeMap::from([(
                        "app".parse().unwrap(),
                        args.app_name.as_str().parse().unwrap(),
                    )])),
                    ..Default::default()
                }),
                spec: Some(k8s_openapi::api::core::v1::PodSpec {
                    containers: vec![k8s_openapi::api::core::v1::Container {
                        name: args.app_name.to_string() + "-container",
                        image: Some(args.image.to_string() + ":" + args.tag.as_str()),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        }),
        ..Default::default()
    }
}
