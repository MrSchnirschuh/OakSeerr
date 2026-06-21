use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_integrations))
        .route("/", post(create_integration))
        .route("/{id}", get(get_integration))
        .route("/{id}", put(update_integration))
        .route("/{id}", delete(delete_integration))
        .route("/{id}/test", post(test_integration))
}

async fn list_integrations() -> &'static str {
    "list_integrations"
}

async fn create_integration() -> &'static str {
    "create_integration"
}

async fn get_integration() -> &'static str {
    "get_integration"
}

async fn update_integration() -> &'static str {
    "update_integration"
}

async fn delete_integration() -> &'static str {
    "delete_integration"
}

async fn test_integration() -> &'static str {
    "test_integration"
}
