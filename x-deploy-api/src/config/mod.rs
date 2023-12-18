pub(crate) struct DotEnvConfig {
    pub(crate) mongodb_url: String,
    pub(crate) mongodb_database: String,
    pub(crate) redis_url: String,
    pub(crate) jwt_secret: String,
    pub(crate) max_organization_by_owner: u64,
}

impl DotEnvConfig {
    pub(crate) fn from_dotenv() -> Self {
        dotenv::dotenv().ok();
        return Self {
            mongodb_url: dotenv::var("MONGODB_URL").expect("MONGODB_URL must be set"),
            mongodb_database: dotenv::var("MONGODB_DATABASE")
                .expect("MONGODB_DATABASE must be set"),
            redis_url: dotenv::var("REDIS_URL").expect("REDIS_URL must be set"),
            jwt_secret: dotenv::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            max_organization_by_owner: dotenv::var("MAX_ORGANIZATION_BY_OWNER")
                .expect("MAX_ORGANIZATION_BY_OWNER must be set")
                .parse::<u64>()
                .expect("MAX_ORGANIZATION_BY_OWNER must be an unsigned integer"),
        };
    }
}
