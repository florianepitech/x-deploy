pub trait ToFilePrefix {
  fn to_prefix(&self) -> String;
}

pub enum CommonS3BucketType {
  UserProfilePicture,
  OrganizationLogo,
  ProjectLogo,
}

impl ToFilePrefix for CommonS3BucketType {
  fn to_prefix(&self) -> String {
    match self {
      CommonS3BucketType::UserProfilePicture => {
        "user-profile-picture".to_string()
      }
      CommonS3BucketType::OrganizationLogo => "organization-logo".to_string(),
      CommonS3BucketType::ProjectLogo => "project-logo".to_string(),
    }
  }
}
