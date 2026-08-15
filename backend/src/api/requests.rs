use crate::AppState;
use crate::api::middleware::{require_admin, require_auth};
use crate::models::MediaRequest;
use crate::services::requests::RequestService;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestResponse {
    pub id: String,
    pub user_id: String,
    pub media_type: String,
    pub media_id: String,
    pub title: String,
    pub status: String,
    pub download_status: String,
    pub external_service_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<MediaRequest> for RequestResponse {
    fn from(r: MediaRequest) -> Self {
        RequestResponse {
            id: r.id,
            user_id: r.user_id,
            media_type: r.media_type,
            media_id: r.media_id,
            title: r.title,
            status: r.status,
            download_status: r.download_status,
            external_service_id: r.external_service_id,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
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
        .route("/status", get(get_requests_status))
        .route("/{id}", get(get_request))
        .route("/{id}/approve", post(approve_request))
        .route("/{id}/decline", post(decline_request))
}

async fn list_requests(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<RequestResponse>>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&headers, &state).await?;
    let requests = state.db.list_requests().await.unwrap_or_default();
    Ok(Json(
        requests.into_iter().map(RequestResponse::from).collect(),
    ))
}

async fn create_request(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CreateRequestRequest>,
) -> Result<Json<RequestResponse>, (StatusCode, Json<serde_json::Value>)> {
    let claims = require_auth(&headers, &state)?;
    let user_id = claims.sub;

    let request = RequestService::create(
        &state.db,
        &user_id,
        &req.media_type,
        &req.media_id,
        &req.title,
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(RequestResponse::from(request)))
}

async fn get_request(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<RequestResponse>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&headers, &state).await?;
    let request = state
        .db
        .get_request(&id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Request not found"})),
            )
        })?;

    Ok(Json(RequestResponse::from(request)))
}

async fn approve_request(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<RequestResponse>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&headers, &state).await?;
    let request = RequestService::approve(&state.db, &id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(RequestResponse::from(request)))
}

async fn decline_request(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<RequestResponse>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&headers, &state).await?;
    let request = RequestService::decline(&state.db, &id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(RequestResponse::from(request)))
}

/// GET /api/v1/requests/status - returns all requests with download status
async fn get_requests_status(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<RequestResponse>>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&headers, &state).await?;
    let requests = RequestService::get_all_with_status(&state.db)
        .await
        .unwrap_or_default();
    Ok(Json(
        requests.into_iter().map(RequestResponse::from).collect(),
    ))
}
