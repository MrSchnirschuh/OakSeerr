use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::AppState;
use crate::models::MediaRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestResponse {
    pub id: String,
    pub user_id: String,
    pub media_type: String,
    pub media_id: String,
    pub title: String,
    pub status: String,
    pub external_service_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRequestRequest {
    pub media_type: String,
    pub media_id: String,
    pub title: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_requests))
        .route("/", post(create_request))
        .route("/{id}", get(get_request))
        .route("/{id}/approve", post(approve_request))
        .route("/{id}/decline", post(decline_request))
}

async fn list_requests(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<RequestResponse>> {
    let requests = state.db.list_requests().await.unwrap_or_default();
    Json(requests.into_iter().map(|r| RequestResponse {
        id: r.id,
        user_id: r.user_id,
        media_type: r.media_type,
        media_id: r.media_id,
        title: r.title,
        status: r.status,
        external_service_id: r.external_service_id,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }).collect())
}

async fn create_request(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateRequestRequest>,
) -> Json<RequestResponse> {
    let now = chrono::Utc::now().to_rfc3339();
    let request = MediaRequest {
        id: Uuid::new_v4().to_string(),
        user_id: "demo-user".to_string(),
        media_type: req.media_type,
        media_id: req.media_id,
        title: req.title,
        status: "pending".to_string(),
        external_service_id: None,
        created_at: now.clone(),
        updated_at: now,
    };
    state.db.create_request(&request).await.unwrap();
    Json(RequestResponse {
        id: request.id,
        user_id: request.user_id,
        media_type: request.media_type,
        media_id: request.media_id,
        title: request.title,
        status: request.status,
        external_service_id: request.external_service_id,
        created_at: request.created_at,
        updated_at: request.updated_at,
    })
}

async fn get_request(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<RequestResponse> {
    let request = state.db.get_request(&id).await.unwrap().unwrap();
    Json(RequestResponse {
        id: request.id,
        user_id: request.user_id,
        media_type: request.media_type,
        media_id: request.media_id,
        title: request.title,
        status: request.status,
        external_service_id: request.external_service_id,
        created_at: request.created_at,
        updated_at: request.updated_at,
    })
}

async fn approve_request(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<RequestResponse> {
    let mut request = state.db.get_request(&id).await.unwrap().unwrap();
    request.status = "approved".to_string();
    request.updated_at = chrono::Utc::now().to_rfc3339();
    state.db.update_request(&request).await.unwrap();

    // Try to send to the appropriate integration
    let integrations = state.db.list_integrations().await.unwrap_or_default();
    let integration = integrations.iter().find(|i| {
        i.enabled && match request.media_type.as_str() {
            "movie" => i.integration_type == "radarr",
            "tv" => i.integration_type == "sonarr",
            "music" => i.integration_type == "lidarr",
            "book" => i.integration_type == "readarr",
            "comic" => i.integration_type == "mylar3",
            _ => false,
        }
    });

    if let Some(integration) = integration {
        let client = reqwest::Client::new();
        let base = integration.base_url.trim_end_matches('/');
        let media_id: i64 = request.media_id.parse().unwrap_or(0);

        let url = match request.media_type.as_str() {
            "movie" => format!("{}/api/v3/movie", base),
            "tv" => format!("{}/api/v3/series", base),
            "music" => format!("{}/api/v1/artist", base),
            "book" => format!("{}/api/v1/author", base),
            _ => return Json(RequestResponse {
                id: request.id,
                user_id: request.user_id,
                media_type: request.media_type.clone(),
                media_id: request.media_id.clone(),
                title: request.title.clone(),
                status: request.status.clone(),
                external_service_id: request.external_service_id.clone(),
                created_at: request.created_at.clone(),
                updated_at: request.updated_at.clone(),
            }),
        };

        let body = serde_json::json!({
            "id": media_id,
            "monitored": true,
            "addOptions": { "searchForMissing": true }
        });

        let res = client.post(&url)
            .header("X-Api-Key", &integration.api_key)
            .json(&body)
            .send()
            .await;

        if let Ok(r) = res {
            if r.status().is_success() {
                request.status = "fulfilled".to_string();
                request.updated_at = chrono::Utc::now().to_rfc3339();
                state.db.update_request(&request).await.unwrap();
            }
        }
    }

    Json(RequestResponse {
        id: request.id,
        user_id: request.user_id,
        media_type: request.media_type.clone(),
        media_id: request.media_id.clone(),
        title: request.title.clone(),
        status: request.status.clone(),
        external_service_id: request.external_service_id.clone(),
        created_at: request.created_at.clone(),
        updated_at: request.updated_at.clone(),
    })
}

async fn decline_request(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<RequestResponse> {
    let mut request = state.db.get_request(&id).await.unwrap().unwrap();
    request.status = "declined".to_string();
    request.updated_at = chrono::Utc::now().to_rfc3339();
    state.db.update_request(&request).await.unwrap();
    Json(RequestResponse {
        id: request.id,
        user_id: request.user_id,
        media_type: request.media_type.clone(),
        media_id: request.media_id.clone(),
        title: request.title.clone(),
        status: request.status.clone(),
        external_service_id: request.external_service_id.clone(),
        created_at: request.created_at.clone(),
        updated_at: request.updated_at.clone(),
    })
}
