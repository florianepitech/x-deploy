use crate::route::cloud_provider::aws::controller;
use crate::route::cloud_provider::aws::dto::{
  CloudProviderAwsInstance, CloudProviderAwsRegion,
};
use crate::route::{custom_response, ApiResult};
use aws_sdk_ec2::types::InstanceType;
use awsregion::Region;
use rocket::http::Status;

pub async fn all_region() -> ApiResult<Vec<CloudProviderAwsRegion>> {
  let available_region = vec![
    Region::AfSouth1,
    Region::ApEast1,
    Region::ApNortheast1,
    Region::ApNortheast2,
    Region::ApNortheast3,
    Region::ApSouth1,
    Region::ApSoutheast1,
    Region::ApSoutheast2,
    Region::CaCentral1,
    Region::CnNorth1,
    Region::CnNorthwest1,
    Region::EuCentral1,
    Region::EuNorth1,
    Region::EuWest1,
    Region::EuWest2,
    Region::EuWest3,
    Region::MeSouth1,
    Region::SaEast1,
    Region::UsEast1,
    Region::UsEast2,
    Region::UsWest1,
    Region::UsWest2,
  ];
  let mut result: Vec<CloudProviderAwsRegion> = Vec::new();
  for region in available_region {
    let new_region = CloudProviderAwsRegion {
      name: region.to_string(),
      endpoint: region.endpoint().to_string(),
    };
    result.push(new_region);
  }
  custom_response(Status::Ok, result)
}

pub async fn instance_types() -> ApiResult<Vec<CloudProviderAwsInstance>> {
  let instance = vec![
    InstanceType::A1Large,
    InstanceType::A1Medium,
    InstanceType::A1Xlarge,
    InstanceType::A12xlarge,
    InstanceType::A14xlarge,
    InstanceType::A1Metal,
    InstanceType::C1Medium,
    InstanceType::C1Xlarge,
  ];
  let mut result: Vec<CloudProviderAwsInstance> = Vec::new();
  for instance in instance {
    let new_instance = CloudProviderAwsInstance {
      name: instance.as_str().to_string(),
    };
    result.push(new_instance);
  }
  custom_response(Status::Ok, result)
}
