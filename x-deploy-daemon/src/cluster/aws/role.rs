use crate::cluster::aws::AwsCluster;
use aws_sdk_iam::Client;

impl AwsCluster {
  pub async fn create_eks_role(&self) {
    let role_name = "eksServiceRole";
    let iam_client = &self.iam_client;
    let create_role = iam_client
      .create_role()
      .role_name(role_name)
      .assume_role_policy_document(
        r#"{
          "Version": "2012-10-17",
          "Statement": [
            {
              "Effect": "Allow",
              "Principal": {
                "Service": "eks.amazonaws.com"
              },
              "Action": "sts:AssumeRole"
            }
          ]
        }"#,
      )
      .send()
      .await;
  }
}
