pub mod cluster;
pub mod general;

use x_deploy_common::db::organization_role::StandardPermission;

fn get_level(permission: &StandardPermission) -> u8 {
  return match permission {
    StandardPermission::None => 0,
    StandardPermission::Read => 1,
    StandardPermission::ReadWrite => 2,
  };
}
