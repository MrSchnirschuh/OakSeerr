use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use crate::AppState;
use crate::services::settings::SettingsService;

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsRequest {
    pub settings: HashMap<String, String>,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_settings))
        .route("/", put(update_settings))
        .route("/about", get(get_about))
}

async fn get_settings(
    State(state): State<Arc<AppState>>,
) -> Json<HashMap<String, String>> {
    let settings = SettingsService::get_all(&state.db).await.unwrap_or_default();
    Json(settings)
}

async fn update_settings(
    State(state): State<Arc<AppState>>,
    Json(req): Json<UpdateSettingsRequest>,
) -> Result<Json<HashMap<String, String>>, (StatusCode, Json<serde_json::Value>)> {
    for (key, value) in &req.settings {
        SettingsService::set(&state.db, key, value)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": format!("Failed to set '{}': {}", key, e)})),
                )
            })?;
    }

    let settings = SettingsService::get_all(&state.db).await.unwrap_or_default();
    Ok(Json(settings))
}

async fn get_about() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "OakSeerr",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "All-in-one media request manager for Jellyfin",
        "license": "MIT",
        "repository": "https://github.com/MrSchnirschuh/OakSeerr",
        "features": ["Movies", "TV Shows", "Music", "Books", "Comics"],
        "integrations": ["Radarr", "Sonarr", "Lidarr", "Readarr", "Mylar3"]
    }))
}
