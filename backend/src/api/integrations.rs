use axum::{
    extract::{Path, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::AppState;
use crate::models::Integration;

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
) -> Json<Vec<IntegrationResponse>> {
    let integrations = state.db.list_integrations().await.unwrap_or_default();
    Json(integrations.into_iter().map(|i| IntegrationResponse {
        id: i.id,
        name: i.name,
        integration_type: i.integration_type,
        base_url: i.base_url,
        api_key: i.api_key,
        enabled: i.enabled,
        created_at: i.created_at,
        updated_at: i.updated_at,
    }).collect())
}

async fn create_integration(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateIntegrationRequest>,
) -> Json<IntegrationResponse> {
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
    state.db.create_integration(&integration).await.unwrap();
    Json(IntegrationResponse {
        id: integration.id,
        name: integration.name,
        integration_type: integration.integration_type,
        base_url: integration.base_url,
        api_key: integration.api_key,
        enabled: integration.enabled,
        created_at: integration.created_at,
        updated_at: integration.updated_at,
    })
}

async fn get_integration(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<IntegrationResponse> {
    let integration = state.db.get_integration(&id).await.unwrap().unwrap();
    Json(IntegrationResponse {
        id: integration.id,
        name: integration.name,
        integration_type: integration.integration_type,
        base_url: integration.base_url,
        api_key: integration.api_key,
        enabled: integration.enabled,
        created_at: integration.created_at,
        updated_at: integration.updated_at,
    })
}

async fn update_integration(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateIntegrationRequest>,
) -> Json<IntegrationResponse> {
    let mut integration = state.db.get_integration(&id).await.unwrap().unwrap();
    if let Some(name) = req.name { integration.name = name; }
    if let Some(base_url) = req.base_url { integration.base_url = base_url; }
    if let Some(api_key) = req.api_key { integration.api_key = api_key; }
    if let Some(enabled) = req.enabled { integration.enabled = enabled; }
    integration.updated_at = chrono::Utc::now().to_rfc3339();
    state.db.update_integration(&integration).await.unwrap();
    Json(IntegrationResponse {
        id: integration.id,
        name: integration.name,
        integration_type: integration.integration_type,
        base_url: integration.base_url,
        api_key: integration.api_key,
        enabled: integration.enabled,
        created_at: integration.created_at,
        updated_at: integration.updated_at,
    })
}

async fn delete_integration(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    state.db.delete_integration(&id).await.unwrap();
    Json(serde_json::json!({"status": "deleted"}))
}

async fn test_integration(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    let integration = state.db.get_integration(&id).await.unwrap().unwrap();
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap();

    let result = match integration.integration_type.as_str() {
        "radarr" | "sonarr" | "lidarr" | "readarr" | "mylar3" => {
            let url = format!("{}/api/v3/system/status", integration.base_url.trim_end_matches('/'));
            let res = client.get(&url)
                .header("X-Api-Key", &integration.api_key)
                .send()
                .await;
            match res {
                Ok(r) if r.status().is_success() => Ok("Connected successfully"),
                Ok(r) => Err(format!("HTTP {}", r.status())),
                Err(e) => Err(format!("Connection failed: {}", e)),
            }
        }
        _ => Err("Unknown integration type".to_string()),
    };

    match result {
        Ok(msg) => Json(serde_json::json!({"status": "ok", "message": msg})),
        Err(msg) => Json(serde_json::json!({"status": "error", "message": msg})),
    }
}
