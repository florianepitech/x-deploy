use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
  project_id: String,
  #[serde(rename = "projectName")]
  project_name: String,
  description: String,
  #[serde(rename = "planCode")]
  plan_code: String,
  unleash: bool,
  expiration: Option<String>,
  #[serde(rename = "creationDate")]
  creation_date: String,
  #[serde(rename = "orderID")]
  order_id: Option<String>,
  access: String,
  status: String,
  #[serde(rename = "manualQuota")]
  manual_quota: bool,
  iam: Iam,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Iam {
  id: String,
  urn: String,
}
