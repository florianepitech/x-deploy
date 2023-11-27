use reqwest::Error;
use crate::data::account::AccountCurrentCredential;
use crate::{OVH_BASE_URL, OvhClient};

pub async fn get_current_credentials(client: &OvhClient) -> Result<AccountCurrentCredential, Error> {
    let url = format!("{}/auth/currentCredential", OVH_BASE_URL);
    let response = client.send_get_request(url.as_str()).await?;
    let result = response.text().await?;
    let credential: AccountCurrentCredential = serde_json::from_str(&result).unwrap();
    Ok(credential)
}