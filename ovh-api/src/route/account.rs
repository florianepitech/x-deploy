use reqwest::{Error, Response};
use crate::data::account::AccountCurrentCredential;
use crate::{OVH_BASE_URL, OvhClient};
use crate::error::OvhClientError;

pub async fn get_current_credentials(client: &OvhClient) -> Result<AccountCurrentCredential, OvhClientError> {
    let url = format!("{}/auth/currentCredential", OVH_BASE_URL);
    let response: Result<Response, Error> = client.send_get_request(url.as_str()).await;
    if response.is_err() {
        return Err(OvhClientError::from_reqwest_error(response.err().unwrap()));
    }
    if !response.unwrap().status().is_success() {
        return Err(OvhClientError::from_wrong_response("Wrong response".to_string(), response.unwrap().status().as_u16()));
    }
    let result = response.unwrap().text().await;
    if result.is_err() {
        return Err(OvhClientError::from_reqwest_error(result.err().unwrap()));
    }
    let credential = serde_json::from_str(&result.unwrap());
    if credential.is_err() {
        return Err(OvhClientError::from_serialize_error(credential.err().unwrap()));
    }
    Ok(credential.unwrap())
}