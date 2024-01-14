use crate::guard::auth::Auth;
use crate::route::organization::project::cluster::dto::{
  ClusterInfoResponse, CreateClusterRequest,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResult, SuccessMessage,
};
use bson::oid::ObjectId;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;
use x_deploy_common::data::cloud_provider::CloudProviderType;
use x_deploy_common::db::organization_credential_ovh::OrganizationCredentialOvh;
use x_deploy_common::db::organization_project_cluster::{
  ClusterStatus, OrganizationProjectCluster,
};
use x_deploy_common::db::CommonCollection;

pub(crate) async fn new(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  project_id: &str,
  body: Json<CreateClusterRequest>,
) -> ApiResult<SuccessMessage> {
  // TODO: Add permission check
  let body = body.into_inner();
  let org_id = ObjectId::from_str(org_id)?;
  let cred_id = ObjectId::from_str(&body.credential_id)?;
  let project_id = ObjectId::from_str(project_id)?;
  let cp_type = CloudProviderType::from_str(&body.cloud_provider)?;
  if cp_type == CloudProviderType::Aws {
    return custom_error(Status::BadRequest, "AWS is not supported yet");
  }
  let ovh_cred_coll = CommonCollection::<OrganizationCredentialOvh>::new(db);
  let ovh_cred = match ovh_cred_coll.get_by_id(&cred_id).await? {
    Some(ovh_cred) => ovh_cred,
    None => {
      return custom_error(
        Status::NotFound,
        "The credential you provided does not exist",
      );
    }
  };
  let new_cluster = OrganizationProjectCluster::new(
    org_id,
    project_id,
    body.name,
    body.description,
    cp_type.to_string(),
    ovh_cred.id,
    ClusterStatus::Creating,
  );
  let cluster_coll = CommonCollection::<OrganizationProjectCluster>::new(db);
  cluster_coll.insert_one(&new_cluster).await?;
  custom_message(Status::Created, "Your cluster is being created")
}

pub(crate) async fn get_all(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  project_id: &str,
) -> ApiResult<Vec<ClusterInfoResponse>> {
  // TODO: Add permission check
  let org_id = ObjectId::from_str(org_id)?;
  let project_id = ObjectId::from_str(project_id)?;
  let cluster_coll = CommonCollection::<OrganizationProjectCluster>::new(db);
  let clusters = cluster_coll
    .get_of_org_and_project(&org_id, &project_id)
    .await?;
  let mut response: Vec<ClusterInfoResponse> = Vec::new();
  for cluster in clusters {
    let cluster_info = ClusterInfoResponse {
      id: cluster.id.to_hex(),
      name: cluster.name,
      description: cluster.description,
      cloud_provider: cluster.cloud_provider,
      status: cluster.status.to_string(),
    };
    response.push(cluster_info);
  }
  custom_response(Status::Ok, response)
}

pub(crate) async fn get(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  project_id: &str,
  cluster_id: &str,
) -> ApiResult<ClusterInfoResponse> {
  // TODO: Add permission check
  let org_id = ObjectId::from_str(org_id)?;
  let project_id = ObjectId::from_str(project_id)?;
  let cluster_id = ObjectId::from_str(cluster_id)?;

  let cluster_coll = CommonCollection::<OrganizationProjectCluster>::new(db);
  let cluster = cluster_coll
    .get_with_id_of_project(&org_id, &project_id, &cluster_id)
    .await?;
  return match cluster {
    Some(cluster) => {
      let cluster_info = ClusterInfoResponse {
        id: cluster.id.to_hex(),
        name: cluster.name,
        description: cluster.description,
        cloud_provider: cluster.cloud_provider,
        status: cluster.status.to_string(),
      };
      custom_response(Status::Ok, cluster_info)
    }
    None => {
      custom_error(Status::NotFound, "The cluster you requested does not exist")
    }
  };
}
