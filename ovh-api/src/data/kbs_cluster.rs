use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KbsCluster {
  #[serde(rename = "id")]
  id: String,

  #[serde(rename = "region")]
  region: String,

  #[serde(rename = "name")]
  name: String,

  #[serde(rename = "url")]
  url: String,

  #[serde(rename = "nodesUrl")]
  nodes_url: String,

  #[serde(rename = "version")]
  version: String,

  #[serde(rename = "nextUpgradeVersions")]
  next_upgrade_versions: Vec<String>,

  #[serde(rename = "kubeProxyMode")]
  kube_proxy_mode: String,

  #[serde(rename = "customization")]
  customization: Customization,

  #[serde(rename = "status")]
  status: String,

  #[serde(rename = "updatePolicy")]
  update_policy: String,

  #[serde(rename = "isUpToDate")]
  is_up_to_date: bool,

  #[serde(rename = "controlPlaneIsUpToDate")]
  control_plane_is_up_to_date: bool,

  #[serde(rename = "privateNetworkId")]
  private_network_id: Option<String>,

  #[serde(rename = "nodesSubnetId")]
  nodes_subnet_id: Option<String>,

  #[serde(rename = "createdAt")]
  created_at: String,

  #[serde(rename = "updatedAt")]
  updated_at: String,

  #[serde(rename = "auditLogsSubscribed")]
  audit_logs_subscribed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Customization {
  #[serde(rename = "apiServer")]
  api_server: ApiServer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiServer {
  #[serde(rename = "admissionPlugins")]
  admission_plugins: AdmissionPlugins,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdmissionPlugins {
  #[serde(rename = "enabled")]
  enabled: Vec<String>,

  #[serde(rename = "disabled")]
  disabled: Vec<String>,
}
