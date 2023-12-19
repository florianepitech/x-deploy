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

#[cfg(test)]
mod tests {
  use crate::route::cloud::{
    get_cluster_kbs_info, get_kubconfig, get_list_cluster_kbs,
    get_project_info, get_project_list,
  };
  use crate::OvhClient;

  #[tokio::test]
  async fn test_get_project_list() {
    // Load env with dotenv
    dotenv::dotenv().ok();
    let application_key = std::env::var("OVH_APPLICATION_KEY").unwrap();
    let application_secret = std::env::var("OVH_APPLICATION_SECRET").unwrap();
    let consumer_key = std::env::var("OVH_CONSUMER_KEY").unwrap();

    let client =
      OvhClient::new(application_key, application_secret, consumer_key);
    let result = get_project_list(&client).await;
    if (result.is_err()) {
      println!("Error: {:?}", result.err().unwrap());
      return;
    }
    println!("{:?}", result);
  }

  #[tokio::test]
  async fn test_get_project_info() {
    // Load env with dotenv
    dotenv::dotenv().ok();
    let application_key = std::env::var("OVH_APPLICATION_KEY").unwrap();
    let application_secret = std::env::var("OVH_APPLICATION_SECRET").unwrap();
    let consumer_key = std::env::var("OVH_CONSUMER_KEY").unwrap();

    let client =
      OvhClient::new(application_key, application_secret, consumer_key);
    let result = get_project_list(&client).await;
    println!("{:?}", result);

    let ids = result.unwrap();
    assert!(!ids.is_empty());

    let result = get_project_info(&client, ids.first().unwrap()).await;
    println!("{:?}", result);
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_get_list_cluster_kbs() {
    // Load env with dotenv
    dotenv::dotenv().ok();
    let application_key = std::env::var("OVH_APPLICATION_KEY").unwrap();
    let application_secret = std::env::var("OVH_APPLICATION_SECRET").unwrap();
    let consumer_key = std::env::var("OVH_CONSUMER_KEY").unwrap();

    let client =
      OvhClient::new(application_key, application_secret, consumer_key);
    let result = get_project_list(&client).await;
    println!("{:?}", result);

    let ids = result.unwrap();
    assert!(!ids.is_empty());

    let result = get_list_cluster_kbs(&client, ids.first().unwrap()).await;
    println!("{:?}", result);
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_get_cluster_kbs_info() {
    // Load env with dotenv
    dotenv::dotenv().ok();
    let application_key = std::env::var("OVH_APPLICATION_KEY").unwrap();
    let application_secret = std::env::var("OVH_APPLICATION_SECRET").unwrap();
    let consumer_key = std::env::var("OVH_CONSUMER_KEY").unwrap();

    let client =
      OvhClient::new(application_key, application_secret, consumer_key);
    let result = get_project_list(&client).await;
    println!("{:?}", result);

    let ids = result.unwrap();
    assert!(!ids.is_empty());

    let result = get_list_cluster_kbs(&client, ids.first().unwrap()).await;
    println!("{:?}", result);
    assert!(result.is_ok());

    let cluster_names = result.unwrap();
    assert!(!cluster_names.is_empty());

    let result = get_cluster_kbs_info(
      &client,
      ids.first().unwrap(),
      cluster_names.first().unwrap(),
    )
    .await;
    println!("{:?}", result);
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_get_kubconfig() {
    // Load env with dotenv
    dotenv::dotenv().ok();
    let application_key = std::env::var("OVH_APPLICATION_KEY").unwrap();
    let application_secret = std::env::var("OVH_APPLICATION_SECRET").unwrap();
    let consumer_key = std::env::var("OVH_CONSUMER_KEY").unwrap();

    let client =
      OvhClient::new(application_key, application_secret, consumer_key);
    let result = get_project_list(&client).await;
    println!("{:?}", result);

    let ids = result.unwrap();
    assert!(!ids.is_empty());

    let result = get_list_cluster_kbs(&client, ids.first().unwrap()).await;
    println!("{:?}", result);
    assert!(result.is_ok());

    let cluster_names = result.unwrap();
    assert!(!cluster_names.is_empty());

    let result = get_cluster_kbs_info(
      &client,
      ids.first().unwrap(),
      cluster_names.first().unwrap(),
    )
    .await;
    println!("{:?}", result);
    assert!(result.is_ok());

    let result = get_kubconfig(
      &client,
      ids.first().unwrap(),
      cluster_names.first().unwrap(),
    )
    .await;
    println!("{:?}", result);
    assert!(result.is_ok());
  }
}
