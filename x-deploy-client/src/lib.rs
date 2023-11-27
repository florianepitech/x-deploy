pub struct XDeployClient {
    api_key: String,
}

impl XDeployClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
        }
    }
}