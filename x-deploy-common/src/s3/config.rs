use rusoto_core::credential::StaticProvider;
use rusoto_core::{HttpClient, Region};
use rusoto_s3::S3Client;

pub struct CommonS3Config {
  pub(crate) endpoint: String,
  pub(crate) bucket: String,
  pub(crate) access_key: String,
  pub(crate) secret_key: String,
  pub(crate) region: String,
  pub(crate) s3_client: S3Client,
}

impl CommonS3Config {
  pub fn new(
    endpoint: String,
    bucket: String,
    access_key: String,
    secret_key: String,
    region: String,
  ) -> Self {
    let region_s3 = Region::Custom {
      name: region.clone(),
      endpoint: endpoint.clone(),
    };
    let s3_client = S3Client::new_with(
      HttpClient::new().expect("failed to create request dispatcher"),
      StaticProvider::new(
        access_key.to_owned(),
        secret_key.to_owned(),
        None,
        None,
      ),
      region_s3,
    );
    Self {
      endpoint,
      bucket,
      access_key,
      secret_key,
      region,
      s3_client,
    }
  }
}
