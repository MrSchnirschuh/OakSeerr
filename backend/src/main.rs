use axum::{
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use std::path::PathBuf;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

mod api;
mod config;
mod db;
mod models;
mod services;

pub struct AppState {
    pub db: db::Database,
    pub config: config::Config,
    pub auth_service: services::auth::AuthService,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "oakseerr=info,tower_http=info".into()))
        .init();

    // Load config
    let config = config::Config::from_env()?;
    tracing::info!("OakSeerr v{} starting...", env!("CARGO_PKG_VERSION"));

    // Initialize database
    let db = db::Database::new(&config.database_url).await?;
    db.run_migrations().await?;

    let state = Arc::new(AppState {
        db,
        config: config.clone(),
        auth_service: services::auth::AuthService::new(&config.jwt_secret),
    });

    // Start background download status polling
    let poll_state = state.clone();
    tokio::spawn(async move {
        download_status_poller(poll_state).await;
    });

    let addr = state.config.listen_addr.clone();

    // Build router
    let frontend_path: PathBuf = std::env::var("OAKSEERR_FRONTEND_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            [env!("CARGO_MANIFEST_DIR"), "..", "frontend", "out"].iter().collect()
        });

    let app = Router::new()
        .route("/api/health", get(health_check))
        .nest("/api/v1/auth", api::auth::router())
        .nest("/api/v1/requests", api::requests::router())
        .nest("/api/v1/media", api::media::router())
        .nest("/api/v1/settings", api::settings::router())
        .nest("/api/v1/integrations", api::integrations::router())
        .nest("/api/v1/users", api::users::router())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
        .fallback_service(ServeDir::new(&frontend_path).append_index_html_on_directories(true));

    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Background task: every 60 seconds, check Sabnzbd and *arrs for active downloads
/// and update request statuses.
async fn download_status_poller(state: Arc<AppState>) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
    loop {
        interval.tick().await;
        tracing::debug!("Polling download status...");

        // Get all approved requests that are not yet imported/failed
        let requests = match state.db.list_requests().await {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!("Failed to list requests for polling: {}", e);
                continue;
            }
        };

        let active_requests: Vec<_> = requests.into_iter()
            .filter(|r| r.status == "approved" && r.download_status != "imported" && r.download_status != "failed")
            .collect();

        if active_requests.is_empty() {
            continue;
        }

        // Get integrations for checking
        let integrations = match state.db.list_integrations().await {
            Ok(i) => i,
            Err(e) => {
                tracing::warn!("Failed to list integrations for polling: {}", e);
                continue;
            }
        };

        for request in &active_requests {
            let integration_type = match request.media_type.as_str() {
                "movie" => "radarr",
                "tv" => "sonarr",
                "music" => "lidarr",
                "book" => "readarr",
                "comic" => "mylar3",
                _ => continue,
            };

            // Find matching integration
            let integration = match integrations.iter().find(|i| i.integration_type == integration_type && i.enabled) {
                Some(i) => i,
                None => continue,
            };

            // Check the *arr API for the item's status
            let endpoint = match integration_type {
                "radarr" => "movie",
                "sonarr" => "series",
                "lidarr" => "artist",
                "readarr" => "book",
                "mylar3" => "comics",
                _ => continue,
            };

            let client = reqwest::Client::new();
            let search_url = format!(
                "{}/api/v3/{}?apikey={}&term={}",
                integration.base_url.trim_end_matches('/'),
                endpoint,
                integration.api_key,
                urlencoding(&request.title),
            );

            match client.get(&search_url).send().await {
                Ok(resp) => {
                    if let Ok(data) = resp.json::<Vec<serde_json::Value>>().await {
                        if let Some(item) = data.first() {
                            // Check if the item has been downloaded/imported
                            let has_file = item.get("hasFile").and_then(|v| v.as_bool()).unwrap_or(false);
                            let _monitored = item.get("monitored").and_then(|v| v.as_bool()).unwrap_or(false);
                            let status = item.get("status").and_then(|v| v.as_str()).unwrap_or("");

                            let download_status = if has_file || status == "downloaded" {
                                "imported"
                            } else if status == "queued" {
                                "queued"
                            } else if status == "downloading" {
                                "downloading"
                            } else {
                                continue; // No change
                            };

                            if let Err(e) = services::requests::RequestService::update_download_status(
                                &state.db,
                                &request.id,
                                download_status,
                            ).await {
                                tracing::warn!("Failed to update download status for request {}: {}", request.id, e);
                            } else {
                                tracing::info!("Updated download status for '{}' to {}", request.title, download_status);
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::debug!("Failed to check *arr for '{}': {}", request.title, e);
                }
            }
        }
    }
}

fn urlencoding(s: &str) -> String {
    let mut result = String::new();
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            b' ' => result.push_str("%20"),
            _ => {
                result.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    result
}
