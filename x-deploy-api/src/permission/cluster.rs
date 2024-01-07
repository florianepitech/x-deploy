use x_deploy_common::db::organization_role::{
  ClusterPermission, OrganizationRole,
};

#[deprecated]
pub fn has_cluster_permission(
  role: OrganizationRole,
  ask: ClusterPermission,
) -> bool {
  todo!()
}
