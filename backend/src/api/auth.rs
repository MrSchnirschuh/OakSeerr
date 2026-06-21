use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login))
        .route("/jellyfin", post(jellyfin_login))
        .route("/me", get(get_me))
        .route("/logout", post(logout))
}

async fn login() -> &'static str {
    "login"
}

async fn jellyfin_login() -> &'static str {
    "jellyfin_login"
}

async fn get_me() -> &'static str {
    "get_me"
}

async fn logout() -> &'static str {
    "logout"
}
