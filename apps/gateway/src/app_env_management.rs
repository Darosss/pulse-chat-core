use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub message_service_url: String,
    pub accounts_service_url: String,
}

pub fn load_config() -> Config {
    dotenvy::dotenv().ok();

    envy::from_env::<Config>().expect("Failed to load configuration from environment variables")
}
