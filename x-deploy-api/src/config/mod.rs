pub(crate) struct DotEnvConfig {
    pub(crate) mongodb_url: String,
    pub(crate) mongodb_database: String,
    pub(crate) redis_url: String,
}

impl DotEnvConfig {
    pub(crate) fn from_dotenv() -> Self {
        dotenv::dotenv().ok();
        return Self {
            mongodb_url: dotenv::var("MONGODB_URL").expect("MONGODB_URL must be set"),
            mongodb_database: dotenv::var("MONGODB_DATABASE").expect("MONGODB_DATABASE must be set"),
            redis_url: dotenv::var("REDIS_URL").expect("REDIS_URL must be set"),
        };
    }
}