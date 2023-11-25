use crate::{OVH_BASE_URL, OvhClient};

pub (crate) async fn get_project_list(
    client: &OvhClient
) -> Vec<String> {
    let url: String = format!("{}/cloud/project", OVH_BASE_URL);
    let result = client.send_get_request(url.as_str()).await.unwrap();
    // parse with serde json the array of id and return array of string
    let result = result.text().await.unwrap();
    let strings: Vec<String> = serde_json::from_str(&result).unwrap();
    strings
}

pub (crate) async fn get_project_info(
    client: &OvhClient,
    project_id: &str
) {
    let url: String = format!("{}/cloud/project/{}", OVH_BASE_URL, project_id);
    let result = client.send_get_request(url.as_str()).await.unwrap();
    
}

pub(crate) async fn get_list_cluster_kbs(
    client: &OvhClient,
    service_name: &str
) {

}

pub(crate) async fn create_new_cluster(client: &OvhClient) {

}

pub(crate) async fn delete_cluster(
    client: &OvhClient,
    cluster_id: &str
) {

}

#[cfg(test)]
mod tests {
    use crate::OvhClient;
    use crate::route::cloud::get_project_list;

    #[tokio::test]
    async fn test_get_project_list() {
        // Load env with dotenv
        dotenv::dotenv().ok();
        let application_key = std::env::var("OVH_APPLICATION_KEY").unwrap();
        let application_secret = std::env::var("OVH_APPLICATION_SECRET").unwrap();
        let consumer_key = std::env::var("OVH_CONSUMER_KEY").unwrap();

        let client = OvhClient::new(
            application_key,
            application_secret,
            consumer_key,
        );
        let result = get_project_list(&client).await;
        println!("{:?}", result);
    }
}