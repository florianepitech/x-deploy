pub(crate) struct DotEnvConfig {
  pub(crate) mongodb_url: String,
  pub(crate) mongodb_database: String,
  pub(crate) kafka_url: String,
  pub(crate) redis_url: String,
  pub(crate) jwt_secret: String,
  pub(crate) app_name: String,
  pub(crate) max_organization_by_owner: u64,
  pub(crate) max_apikey_by_organization: u64,
  pub(crate) jwt_key_duration_in_minutes: u64,
}

impl DotEnvConfig {
  pub(crate) fn from_dotenv() -> Self {
    dotenv::dotenv().ok();
    return Self {
      mongodb_url: dotenv::var("MONGODB_URL").expect("MONGODB_URL must be set"),
      mongodb_database: dotenv::var("MONGODB_DATABASE")
        .expect("MONGODB_DATABASE must be set"),
      kafka_url: dotenv::var("KAFKA_URL").expect("KAFKA_URL must be set"),
      redis_url: dotenv::var("REDIS_URL").expect("REDIS_URL must be set"),
      jwt_secret: dotenv::var("JWT_SECRET").expect("JWT_SECRET must be set"),
      app_name: dotenv::var("APP_NAME").expect("APP_NAME must be set"),
      max_organization_by_owner: dotenv::var("MAX_ORGANIZATION_BY_OWNER")
        .expect("MAX_ORGANIZATION_BY_OWNER must be set")
        .parse::<u64>()
        .expect("MAX_ORGANIZATION_BY_OWNER must be an unsigned integer"),
      max_apikey_by_organization: dotenv::var("MAX_APIKEY_BY_ORGANIZATION")
        .expect("MAX_APIKEY_BY_ORGANIZATION must be set")
        .parse::<u64>()
        .expect("MAX_APIKEY_BY_ORGANIZATION must be an unsigned integer"),
      jwt_key_duration_in_minutes: dotenv::var("JWT_KEY_DURATION_IN_MINUTES")
        .expect("JWT_KEY_DURATION_IN_MINUTES must be set")
        .parse::<u64>()
        .expect("JWT_KEY_DURATION_IN_MINUTES must be an unsigned integer"),
    };
  }
}
