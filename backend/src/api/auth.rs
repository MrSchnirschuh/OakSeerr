use crate::AppState;
use crate::models::User;
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {}

#[derive(Debug, Deserialize)]
pub struct JellyfinLoginRequest {
    pub url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
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
            permissions: u.permissions,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login))
        .route("/jellyfin", post(jellyfin_login))
        .route("/me", get(get_me))
        .route("/logout", post(logout))
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(_req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    if !state.config.demo_mode {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(
                serde_json::json!({"error": "Demo mode is disabled. Use /auth/jellyfin to log in."}),
            ),
        ));
    }

    let (user, token) = state
        .auth_service
        .create_demo_user(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

async fn jellyfin_login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<JellyfinLoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    let (user, token) = state
        .auth_service
        .jellyfin_auth(&state.db, &req.url, &req.username, &req.password)
        .await
        .map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

async fn get_me(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Missing Authorization header"})),
            )
        })?;

    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Invalid Authorization format"})),
        )
    })?;

    let claims = state.auth_service.verify_token(token).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": format!("Invalid token: {}", e)})),
        )
    })?;

    let user = state
        .db
        .get_user(&claims.sub)
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

    Ok(Json(user.into()))
}

async fn logout() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "logged_out"}))
}
