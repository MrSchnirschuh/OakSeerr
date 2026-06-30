use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::api::middleware::require_auth;
use crate::AppState;
use crate::models::Integration;
use crate::services::integrations::IntegrationService;

const ALLOWED_TYPES: &[&str] = &["radarr", "sonarr", "lidarr", "readarr", "mylar3"];

#[derive(Debug, Serialize, Deserialize)]
pub struct IntegrationResponse {
    pub id: String,
    pub name: String,
    pub integration_type: String,
    pub base_url: String,
    pub api_key: String,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Integration> for IntegrationResponse {
    fn from(i: Integration) -> Self {
        IntegrationResponse {
            id: i.id,
            name: i.name,
            integration_type: i.integration_type,
            base_url: i.base_url,
            api_key: i.api_key,
            enabled: i.enabled,
            created_at: i.created_at,
            updated_at: i.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateIntegrationRequest {
    pub name: String,
    pub integration_type: String,
    pub base_url: String,
    pub api_key: String,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateIntegrationRequest {
    pub name: Option<String>,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub enabled: Option<bool>,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_integrations))
        .route("/", post(create_integration))
        .route("/{id}", get(get_integration))
        .route("/{id}", put(update_integration))
        .route("/{id}", delete(delete_integration))
        .route("/{id}/test", post(test_integration))
}

async fn list_integrations(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<IntegrationResponse>>, (StatusCode, Json<serde_json::Value>)> {
    require_auth(&headers, &state)?;
    let integrations = state.db.list_integrations().await.unwrap_or_default();
    Ok(Json(integrations.into_iter().map(IntegrationResponse::from).collect()))
}

async fn create_integration(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CreateIntegrationRequest>,
) -> Result<Json<IntegrationResponse>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    require_auth(&headers, &state)?;
    if !ALLOWED_TYPES.contains(&req.integration_type.as_str()) {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": format!("Invalid integration type '{}'. Allowed types: {:?}", req.integration_type, ALLOWED_TYPES)
            })),
        ));
    }

    let integration = Integration {
        id: Uuid::new_v4().to_string(),
        name: req.name,
        integration_type: req.integration_type,
        base_url: req.base_url,
        api_key: req.api_key,
        enabled: req.enabled,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };

    state.db.create_integration(&integration).await.map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(IntegrationResponse::from(integration)))
}

async fn get_integration(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<IntegrationResponse>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    require_auth(&headers, &state)?;
    let integration = state.db.get_integration(&id).await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
        })?
        .ok_or_else(|| {
            (
                axum::http::StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Integration not found"})),
            )
        })?;

    Ok(Json(IntegrationResponse::from(integration)))
}

async fn update_integration(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(req): Json<UpdateIntegrationRequest>,
) -> Result<Json<IntegrationResponse>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    require_auth(&headers, &state)?;
    let mut integration = state.db.get_integration(&id).await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
        })?
        .ok_or_else(|| {
            (
                axum::http::StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Integration not found"})),
            )
        })?;

    if let Some(name) = req.name { integration.name = name; }
    if let Some(base_url) = req.base_url { integration.base_url = base_url; }
    if let Some(api_key) = req.api_key { integration.api_key = api_key; }
    if let Some(enabled) = req.enabled { integration.enabled = enabled; }
    integration.updated_at = chrono::Utc::now().to_rfc3339();

    state.db.update_integration(&integration).await.map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(IntegrationResponse::from(integration)))
}

async fn delete_integration(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_auth(&headers, &state)?;
    state.db.delete_integration(&id).await.unwrap_or_default();
    Ok(Json(serde_json::json!({"status": "deleted"})))
}

async fn test_integration(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    require_auth(&headers, &state)?;
    let integration = state.db.get_integration(&id).await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
        })?
        .ok_or_else(|| {
            (
                axum::http::StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Integration not found"})),
            )
        })?;

    match IntegrationService::test(&integration).await {
        Ok(msg) => Ok(Json(serde_json::json!({"status": "ok", "message": msg}))),
        Err(e) => Ok(Json(serde_json::json!({"status": "error", "message": e.to_string()}))),
    }
}
