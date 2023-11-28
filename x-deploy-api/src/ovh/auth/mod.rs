use ovh_api::OvhClient;

pub async fn test_ovh_connection(client: &OvhClient) -> bool {
    let result = ovh_api::route::account::get_current_credentials(&client).await;
    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}