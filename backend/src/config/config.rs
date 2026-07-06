use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub listen_addr: String,
    pub port: u16,
    pub jellyfin_url: Option<String>,
    pub jellyfin_api_key: Option<String>,
    pub log_level: String,
    pub demo_mode: bool,
    pub cors_origin: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let port: u16 = env::var("PORT")
            .unwrap_or_else(|_| "5055".to_string())
            .parse()
            .unwrap_or(5055);

        let listen_addr = env::var("LISTEN_ADDR")
            .unwrap_or_else(|_| format!("0.0.0.0:{}", port));

        Ok(Config {
            database_url: env::var("OAKSEERR_DB_PATH")
                .map(|p| {
                    if p.starts_with("sqlite://") { p }
                    else { format!("sqlite://{}?mode=rwc", p) }
                })
                .or_else(|_| env::var("DATABASE_URL"))
                .unwrap_or_else(|_| "sqlite:///app/config/oakseerr.db?mode=rwc".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| uuid::Uuid::new_v4().to_string()),
            listen_addr,
            port,
            jellyfin_url: env::var("JELLYFIN_URL").ok(),
            jellyfin_api_key: env::var("JELLYFIN_API_KEY").ok(),
            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "oakseerr=info,tower_http=info".to_string()),
            demo_mode: env::var("DEMO_MODE")
                .ok()
                .and_then(|v| v.parse::<bool>().ok())
                .unwrap_or(false),
            cors_origin: env::var("CORS_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:5055".to_string()),
        })
    }
}
