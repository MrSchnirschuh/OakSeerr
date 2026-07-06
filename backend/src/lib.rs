pub mod api;
pub mod config;
pub mod db;
pub mod models;
pub mod services;

use std::sync::Arc;

pub struct AppState {
    pub db: db::Database,
    pub config: config::Config,
    pub auth_service: services::auth::AuthService,
}

/// Simple URL encoding for search queries
pub fn urlencoding(s: &str) -> String {
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

/// Background task: every 60 seconds, check Sabnzbd and *arrs for active downloads
/// and update request statuses.
pub async fn download_status_poller(state: Arc<AppState>) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
    loop {
        interval.tick().await;
        tracing::debug!("Polling download status...");

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

            let integration = match integrations.iter().find(|i| i.integration_type == integration_type && i.enabled) {
                Some(i) => i,
                None => continue,
            };

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
                            let has_file = item.get("hasFile").and_then(|v| v.as_bool()).unwrap_or(false);
                            let status = item.get("status").and_then(|v| v.as_str()).unwrap_or("");

                            let download_status = if has_file || status == "downloaded" {
                                "imported"
                            } else if status == "queued" {
                                "queued"
                            } else if status == "downloading" {
                                "downloading"
                            } else {
                                continue;
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
