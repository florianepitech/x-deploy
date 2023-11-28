use ovh_api::OvhClient;

pub async fn test_ovh_connection(client: &OvhClient) -> bool {
    let result = ovh_api::route::cloud::get_project_list(client).await;
    if result.is_err() {
        return false;
    }
    true

}
