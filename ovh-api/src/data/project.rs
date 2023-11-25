use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    project_id: String,
    project_name: String,
    description: String,
    plan_code: String,
    unleash: bool,
    expiration: Option<String>,
    creation_date: String,
    order_id: Option<String>,
    access: String,
    status: String,
    manual_quota: bool,
    iam: Iam,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Iam {
    id: String,
    urn: String,
}
