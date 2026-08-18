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

        let listen_addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| format!("0.0.0.0:{port}"));

        Ok(Config {
            database_url: env::var("OAKSEERR_DB_PATH")
                .map(|p| {
                    if p.starts_with("sqlite://") {
                        p
                    } else {
                        format!("sqlite://{p}?mode=rwc")
                    }
                })
                .or_else(|_| env::var("DATABASE_URL"))
                .unwrap_or_else(|_| "sqlite:///app/config/oakseerr.db?mode=rwc".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .map_err(|_| anyhow::anyhow!("JWT_SECRET environment variable must be set"))?,
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn config_defaults_and_custom_port() {
        unsafe {
            env::set_var("JWT_SECRET", "test-secret");
            env::remove_var("PORT");
            env::remove_var("OAKSEERR_DB_PATH");
            env::remove_var("DATABASE_URL");
        }
        let cfg = Config::from_env().unwrap();
        assert_eq!(cfg.port, 5055);
        assert_eq!(cfg.jwt_secret, "test-secret");
        assert!(cfg.database_url.starts_with("sqlite:///app/config/oakseerr.db"));

        unsafe { env::set_var("PORT", "8080"); }
        let cfg = Config::from_env().unwrap();
        assert_eq!(cfg.port, 8080);
        assert_eq!(cfg.listen_addr, "0.0.0.0:8080");
    }
}
