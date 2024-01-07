mod install;
mod role;

use aws_config::{BehaviorVersion, Region};
use aws_sdk_eks::config::Credentials;
use aws_sdk_eks::{Client as EksClient, Config as EksConfig};
use aws_sdk_iam::{Client as IamClient, Config as IamConfig};

pub struct AwsCluster {
  pub secret_key: String,
  pub access_key_id: String,
  pub region: String,
  pub iam_client: IamClient,
  pub eks_client: EksClient,
}

impl AwsCluster {
  pub fn new(
    secret_key: String,
    access_key_id: String,
    region: String,
  ) -> Self {
    let iam_client = Self::new_iam_client(
      secret_key.clone(),
      access_key_id.clone(),
      region.clone(),
    );
    let eks_client = Self::new_eks_client(
      secret_key.clone(),
      access_key_id.clone(),
      region.clone(),
    );
    Self {
      secret_key,
      access_key_id,
      region,
      iam_client,
      eks_client,
    }
  }

  fn new_iam_client(
    secret_key: String,
    access_key_id: String,
    region: String,
  ) -> IamClient {
    let aws_region = Region::new(region.clone());
    let creds =
      Credentials::from_keys(access_key_id.clone(), secret_key.clone(), None);
    let conf = IamConfig::builder()
      .credentials_provider(creds)
      .behavior_version(BehaviorVersion::latest())
      .region(aws_region)
      .build();
    let client = IamClient::from_conf(conf);
    client
  }

  fn new_eks_client(
    secret_key: String,
    access_key_id: String,
    region: String,
  ) -> EksClient {
    let aws_region = Region::new(region.clone());
    let creds =
      Credentials::from_keys(access_key_id.clone(), secret_key.clone(), None);
    let conf = EksConfig::builder()
      .credentials_provider(creds)
      .behavior_version(BehaviorVersion::latest())
      .region(aws_region)
      .build();
    let client = EksClient::from_conf(conf);
    client
  }
}
