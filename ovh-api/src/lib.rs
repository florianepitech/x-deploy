mod route;

struct OvhClient {
    endpoint: String,
    application_key: String,
    application_secret: String,
    consumer_key: String,
}

impl OvhClient {

    fn new(endpoint: String, application_key: String, application_secret: String, consumer_key: String) -> Self {
        Self {
            endpoint,
            application_key,
            application_secret,
            consumer_key,
        }
    }

}