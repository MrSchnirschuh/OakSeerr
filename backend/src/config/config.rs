use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub listen_addr: String,
    pub jellyfin_url: Option<String>,
    pub jellyfin_api_key: Option<String>,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://data/oakseerr.db?mode=rwc".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| uuid::Uuid::new_v4().to_string()),
            listen_addr: env::var("LISTEN_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:5055".to_string()),
            jellyfin_url: env::var("JELLYFIN_URL").ok(),
            jellyfin_api_key: env::var("JELLYFIN_API_KEY").ok(),
            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "oakseerr=info,tower_http=info".to_string()),
        })
    }
}
