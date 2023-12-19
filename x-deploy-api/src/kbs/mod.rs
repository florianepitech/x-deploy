pub(crate) mod deploy;

use kube::{Client, Config};

pub(crate) async fn connect_with_kubeconfig(yaml: &str) -> Client {
  // Charger la configuration depuis le fichier kubeconfig (par défaut)
  let kubeconfig_yaml: kube::config::Kubeconfig =
    serde_yaml::from_str(yaml).unwrap();
  // Create a Kubernetes configuration
  let kubeconfig =
    Config::from_custom_kubeconfig(kubeconfig_yaml, &Default::default())
      .await
      .unwrap();
  // Create a Kubernetes client
  return Client::try_from(kubeconfig).unwrap();
}

#[cfg(test)]
mod tests {
  use crate::kbs::connect_with_kubeconfig;
  use ovh_api::route::cloud::{
    get_cluster_kbs_info, get_kubconfig, get_list_cluster_kbs, get_project_list,
  };
  use ovh_api::OvhClient;
  use rocket::tokio;

  #[tokio::test]
  async fn test_connect_with_kubeconfig() {
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

    connect_with_kubeconfig(result.unwrap().content.as_str()).await;
  }
}
