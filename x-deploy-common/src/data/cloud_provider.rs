use crate::CommonError;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(
  Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone, Copy,
)]
pub enum CloudProviderType {
  #[serde(rename = "AWS")]
  Aws,

  #[serde(rename = "OVH")]
  Ovh,
}

impl CloudProviderType {
  pub fn get_all() -> Vec<CloudProviderType> {
    vec![CloudProviderType::Aws, CloudProviderType::Ovh]
  }
}

impl Display for CloudProviderType {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      CloudProviderType::Aws => write!(f, "AWS"),
      CloudProviderType::Ovh => write!(f, "OVH"),
    }
  }
}

impl FromStr for CloudProviderType {
  type Err = CommonError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "AWS" => Ok(CloudProviderType::Aws),
      "OVH" => Ok(CloudProviderType::Ovh),
      _ => Err(CommonError::FromStrError(
        "Your cloud provider type is not valid".to_string(),
      )),
    }
  }
}
