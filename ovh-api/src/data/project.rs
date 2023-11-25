use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    project_id: String,
    project_name: String,
    description: String,
    plan_code: String,
    unleash: bool,
    expiration: Option<String>,  // Assuming expiration can be null, hence Option
    creation_date: String,
    order_id: Option<String>,    // Assuming order_id can be null, hence Option
    access: String,
    status: String,
    manual_quota: bool,
    iam: Iam,
}

#[derive(Serialize, Deserialize, Debug)]
struct Iam {
    id: String,
    urn: String,
}
