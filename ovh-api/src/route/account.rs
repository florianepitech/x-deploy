use crate::data::account::AccountCurrentCredential;
use crate::error::error::OvhClientError;
use crate::{OvhClient, OVH_BASE_URL};
use reqwest::{Error, Response};

#[deprecated(since = "0.1.0", note = "Not worked")]
pub async fn get_current_credentials(
  client: &OvhClient
) -> Result<AccountCurrentCredential, OvhClientError> {
  let url = format!("{}/auth/currentCredential", OVH_BASE_URL);
  let response: Result<Response, Error> =
    client.send_get_request(url.as_str()).await;
  //if response.is_err() {
  //    return Err(OvhClientError::from_reqwest_error(response.err().unwrap()));
  //}
  //let response_content = response.unwrap();
  //let response_status = response_content.status();
  //if !response_status.is_success() {
  //    return Err(OvhClientError::from_wrong_response(
  //        "Wrong response".to_string(),
  //        response_status.as_u16(),
  //    ));
  //}
  //let result = response_content.text().await;
  //if result.is_err() {
  //    return Err(OvhClientError::from_reqwest_error(result.err().unwrap()));
  //}
  //let credential = serde_json::from_str(&result.unwrap());
  //if credential.is_err() {
  //    return Err(OvhClientError::from_serialize_error(credential.err().unwrap()));
  //}
  //Ok(credential.unwrap())
  Err(OvhClientError::WrongResponse(response.unwrap(), 200))
}
