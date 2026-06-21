use axum::{
    extract::State,
    routing::{get, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsResponse {
    pub app_name: String,
    pub jellyfin_url: String,
    pub sso_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsRequest {
    pub app_name: Option<String>,
    pub jellyfin_url: Option<String>,
    pub jellyfin_api_key: Option<String>,
    pub sso_enabled: Option<bool>,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_settings))
        .route("/", put(update_settings))
        .route("/about", get(get_about))
}

async fn get_settings(
    State(state): State<Arc<AppState>>,
) -> Json<SettingsResponse> {
    let app_name = state.db.get_setting("app_name").await.unwrap_or(Some("OakSeerr".to_string())).unwrap_or_default();
    let jellyfin_url = state.db.get_setting("jellyfin_url").await.unwrap_or_default().unwrap_or_default();
    let sso_enabled = state.db.get_setting("sso_enabled").await.unwrap_or(Some("false".to_string())).unwrap_or_default();

    Json(SettingsResponse {
        app_name,
        jellyfin_url,
        sso_enabled: sso_enabled == "true",
    })
}

async fn update_settings(
    State(state): State<Arc<AppState>>,
    Json(req): Json<UpdateSettingsRequest>,
) -> Json<SettingsResponse> {
    if let Some(app_name) = &req.app_name {
        state.db.set_setting("app_name", app_name).await.unwrap();
    }
    if let Some(jellyfin_url) = &req.jellyfin_url {
        state.db.set_setting("jellyfin_url", jellyfin_url).await.unwrap();
    }
    if let Some(api_key) = &req.jellyfin_api_key {
        state.db.set_setting("jellyfin_api_key", api_key).await.unwrap();
    }
    if let Some(sso) = req.sso_enabled {
        state.db.set_setting("sso_enabled", if sso { "true" } else { "false" }).await.unwrap();
    }

    get_settings(State(state)).await
}

async fn get_about() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "OakSeerr",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "All-in-one media request manager for Jellyfin",
        "license": "MIT",
        "repository": "https://github.com/MrSchnirschuh/OakSeerr",
        "features": ["Movies", "TV Shows", "Music", "Books", "Comics"],
        "integrations": ["Radarr", "Sonarr", "Lidarr", "Readarr", "Mylar3", "SABnzbd", "Prowlarr"]
    }))
}
