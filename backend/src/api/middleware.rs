use crate::AppState;
use crate::services::auth::Claims;
use axum::Json;
use axum::http::{StatusCode, header};
use std::sync::Arc;

const ADMIN_PERMISSION: i64 = 100;

/// Extract and verify the JWT from the Authorization header.
/// Returns the claims on success, or a 401 error response on failure.
pub fn require_auth(
    headers: &axum::http::HeaderMap,
    state: &Arc<AppState>,
) -> Result<Claims, (StatusCode, Json<serde_json::Value>)> {
    let auth_header = headers
        .get(header::AUTHORIZATION)
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
            Json(serde_json::json!({"error": "Invalid Authorization format. Use: Bearer <token>"})),
        )
    })?;

    state.auth_service.verify_token(token).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": format!("Invalid token: {}", e)})),
        )
    })
}

/// Verify the request is authenticated and the user has admin permissions.
pub async fn require_admin(
    headers: &axum::http::HeaderMap,
    state: &Arc<AppState>,
) -> Result<Claims, (StatusCode, Json<serde_json::Value>)> {
    let claims = require_auth(headers, state)?;
    let user = state.db.get_user(&claims.sub).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
    })?;

    match user {
        Some(u) if u.permissions >= ADMIN_PERMISSION => Ok(claims),
        Some(_) => Err((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "Admin permissions required"})),
        )),
        None => Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "User not found"})),
        )),
    }
}
