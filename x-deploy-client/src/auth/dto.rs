use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LoginResponse {
  pub token: String,
}
