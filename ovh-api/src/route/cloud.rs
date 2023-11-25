use reqwest::Error;
use crate::{OVH_BASE_URL, OvhClient};
use crate::data::Project;

pub (crate) async fn get_project_list(
    client: &OvhClient
) -> Result<Vec<String>, Error> {
    let url: String = format!("{}/cloud/project", OVH_BASE_URL);
    let result = client.send_get_request(url.as_str()).await.unwrap();
    // parse with serde json the array of id and return array of string
    let result = result.text().await.unwrap();
    let strings: Vec<String> = serde_json::from_str(&result).unwrap();
    Ok(strings)
}

pub (crate) async fn get_project_info(
    client: &OvhClient,
    project_id: &str
) -> Result<Project, Error> {
    let url: String = format!("{}/cloud/project/{}", OVH_BASE_URL, project_id);
    let response = client.send_get_request(url.as_str()).await.unwrap();
    let result = response.text().await.unwrap();
    let project: Project = serde_json::from_str(&result).unwrap();
    Ok(project)
}

pub(crate) async fn get_list_cluster_kbs(
    client: &OvhClient,
    project_name: &str
) -> Result<Vec<String>, Error>{
    let url: String = format!("{}/cloud/project/{}/kube", OVH_BASE_URL, project_name);
    let response = client.send_get_request(url.as_str()).await.unwrap();
    let result = response.text().await.unwrap();
    let strings: Vec<String> = serde_json::from_str(&result).unwrap();
    Ok(strings)
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
    use crate::route::cloud::{get_project_info, get_project_list};

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

    #[tokio::test]
    async fn test_get_project_info() {
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
        assert!(!result.unwrap().is_empty());

        let result = get_project_info(&client, result.unwrap().first().unwrap()).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}