use crate::{API_URL, XDeployClient};

pub async fn login(
    client: &XDeployClient,
    email: &String,
    password: &String
) -> Result<String, reqwest::Error> {
    let url = format!("{}/auth/login", API_URL);
    let body = serde_json::json!({
        "email": email,
        "password": password,
    });
    let body = reqwest::Client::new()
        .post(url)
        .body(body.to_string())
        .build()?;
    let body = client.reqwest_client.execute(body).await?.text().await?;
    let body: serde_json::Value = serde_json::from_str(&body).unwrap();
    let token = body["token"].as_str().unwrap();
    Ok(token.to_string())
}