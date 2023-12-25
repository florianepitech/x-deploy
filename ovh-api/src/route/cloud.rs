use crate::data::kbs_cluster::KbsCluster;
use crate::data::kbs_kubeconfig::KubeConfig;
use crate::data::Project;
use crate::{OvhClient, OVH_BASE_URL};
use reqwest::Error;

pub async fn get_project_list(
  client: &OvhClient
) -> Result<Vec<String>, Error> {
  let url: String = format!("{}/cloud/project", OVH_BASE_URL);
  let result = client.send_get_request(url.as_str()).await.unwrap();
  // parse with serde json the array of id and return array of string
  let result = result.text().await.unwrap();
  println!("Get project list result: {:?}", result);
  let strings: Vec<String> = serde_json::from_str(&result).unwrap();
  Ok(strings)
}

pub async fn get_project_info(
  client: &OvhClient,
  project_id: &str,
) -> Result<Project, Error> {
  let url: String = format!("{}/cloud/project/{}", OVH_BASE_URL, project_id);
  let response = client.send_get_request(url.as_str()).await.unwrap();
  let result = response.text().await.unwrap();
  println!("Get project info result: {:?}", result);
  let project: Project = serde_json::from_str(&result).unwrap();
  Ok(project)
}

pub async fn get_list_cluster_kbs(
  client: &OvhClient,
  project_id: &str,
) -> Result<Vec<String>, Error> {
  let url: String =
    format!("{}/cloud/project/{}/kube", OVH_BASE_URL, project_id);
  let response = client.send_get_request(url.as_str()).await.unwrap();
  let result = response.text().await.unwrap();
  println!("Get list cluster kbs result: {:?}", result);
  let strings: Vec<String> = serde_json::from_str(&result).unwrap();
  Ok(strings)
}

pub async fn get_cluster_kbs_info(
  client: &OvhClient,
  project_id: &str,
  cluster_id: &str,
) -> Result<KbsCluster, Error> {
  let url: String = format!(
    "{}/cloud/project/{}/kube/{}",
    OVH_BASE_URL, project_id, cluster_id
  );
  let response = client.send_get_request(url.as_str()).await.unwrap();
  let result = response.text().await.unwrap();
  println!("Get cluster kbs info result: {:?}", result);
  let cluster: KbsCluster = serde_json::from_str(&result).unwrap();
  Ok(cluster)
}

pub async fn get_kubconfig(
  client: &OvhClient,
  project_id: &str,
  cluster_id: &str,
) -> Result<KubeConfig, Error> {
  let url: String = format!(
    "{}/cloud/project/{}/kube/{}/kubeconfig",
    OVH_BASE_URL, project_id, cluster_id
  );
  let response = client.send_post_request(url.as_str(), None).await.unwrap();
  let result = response.text().await.unwrap();
  println!("Get kubconfig result: {:?}", result);
  let kubconfig: KubeConfig = serde_json::from_str(&result).unwrap();
  Ok(kubconfig)
}
