use axum::{
    routing::{get, put},
    Router,
};
use std::sync::Arc;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_settings))
        .route("/", put(update_settings))
        .route("/about", get(get_about))
}

async fn get_settings() -> &'static str {
    "get_settings"
}

async fn update_settings() -> &'static str {
    "update_settings"
}

async fn get_about() -> &'static str {
    "get_about"
}
