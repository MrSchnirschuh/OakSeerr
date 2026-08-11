use crate::AppState;
use crate::api::middleware::require_admin;
use crate::models::User;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub jellyfin_user_id: Option<String>,
    pub permissions: i64,
    pub created_at: String,
    pub updated_at: String,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        UserResponse {
            id: u.id,
            username: u.username,
            display_name: u.display_name,
            email: u.email,
            avatar_url: u.avatar_url,
            jellyfin_user_id: u.jellyfin_user_id,
            permissions: u.permissions,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub display_name: String,
    pub email: Option<String>,
    pub permissions: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub permissions: Option<i64>,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_users))
        .route("/", post(create_user))
        .route("/{id}", get(get_user))
        .route("/{id}", put(update_user))
        .route("/{id}", delete(delete_user))
}

async fn list_users(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<UserResponse>>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&headers, &state).await?;
    let users = state.db.list_users().await.unwrap_or_default();
    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&headers, &state).await?;
    // Check if username already exists
    if let Some(_existing) = state
        .db
        .get_user_by_username(&req.username)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
        })?
    {
        return Err((
            StatusCode::CONFLICT,
            Json(serde_json::json!({"error": "Username already exists"})),
        ));
    }

    let user = User {
        id: Uuid::new_v4().to_string(),
        username: req.username,
        display_name: req.display_name,
        email: req.email,
        avatar_url: None,
        jellyfin_user_id: None,
        permissions: req.permissions.unwrap_or(0),
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };

    state.db.create_user(&user).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(UserResponse::from(user)))
}

async fn get_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&headers, &state).await?;
    let user = state
        .db
        .get_user(&id)
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
                Json(serde_json::json!({"error": "User not found"})),
            )
        })?;

    Ok(Json(UserResponse::from(user)))
}

async fn update_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&headers, &state).await?;
    let mut user = state
        .db
        .get_user(&id)
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
                Json(serde_json::json!({"error": "User not found"})),
            )
        })?;

    if let Some(username) = req.username {
        user.username = username;
    }
    if let Some(display_name) = req.display_name {
        user.display_name = display_name;
    }
    if let Some(email) = req.email {
        user.email = Some(email);
    }
    if let Some(permissions) = req.permissions {
        user.permissions = permissions;
    }
    user.updated_at = chrono::Utc::now().to_rfc3339();

    state.db.update_user(&user).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(UserResponse::from(user)))
}

async fn delete_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&headers, &state).await?;
    // Don't allow deleting the demo user
    if id == "demo-user" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Cannot delete demo user"})),
        ));
    }

    state.db.delete_user(&id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(serde_json::json!({"status": "deleted"})))
}
