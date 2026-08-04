use crate::AppState;
use crate::api::middleware::require_auth;
use crate::services::settings::SettingsService;
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, put},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsRequest {
    pub settings: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeys {
    pub tmdb_api_key: String,
    pub lastfm_api_key: String,
    pub comicvine_api_key: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_settings))
        .route("/", put(update_settings))
        .route("/keys", get(get_api_keys))
        .route("/keys", put(update_api_keys))
        .route("/about", get(get_about))
}

async fn get_settings(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<HashMap<String, String>>, (StatusCode, Json<serde_json::Value>)> {
    require_auth(&headers, &state)?;
    let settings = SettingsService::get_all(&state.db)
        .await
        .unwrap_or_default();
    Ok(Json(settings))
}

async fn update_settings(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<UpdateSettingsRequest>,
) -> Result<Json<HashMap<String, String>>, (StatusCode, Json<serde_json::Value>)> {
    require_auth(&headers, &state)?;
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

    let settings = SettingsService::get_all(&state.db)
        .await
        .unwrap_or_default();
    Ok(Json(settings))
}

async fn get_api_keys(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<ApiKeys>, (StatusCode, Json<serde_json::Value>)> {
    require_auth(&headers, &state)?;
    let tmdb = SettingsService::get(&state.db, "TMDB_API_KEY")
        .await
        .unwrap_or(None)
        .unwrap_or_default();
    let lastfm = SettingsService::get(&state.db, "LASTFM_API_KEY")
        .await
        .unwrap_or(None)
        .unwrap_or_default();
    let comicvine = SettingsService::get(&state.db, "COMICVINE_API_KEY")
        .await
        .unwrap_or(None)
        .unwrap_or_default();

    Ok(Json(ApiKeys {
        tmdb_api_key: tmdb,
        lastfm_api_key: lastfm,
        comicvine_api_key: comicvine,
    }))
}

async fn update_api_keys(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(keys): Json<ApiKeys>,
) -> Result<Json<ApiKeys>, (StatusCode, Json<serde_json::Value>)> {
    require_auth(&headers, &state)?;
    SettingsService::set(&state.db, "TMDB_API_KEY", &keys.tmdb_api_key)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
        })?;
    SettingsService::set(&state.db, "LASTFM_API_KEY", &keys.lastfm_api_key)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
        })?;
    SettingsService::set(&state.db, "COMICVINE_API_KEY", &keys.comicvine_api_key)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(keys))
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
