use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_requests))
        .route("/", post(create_request))
        .route("/{id}", get(get_request))
        .route("/{id}/approve", post(approve_request))
        .route("/{id}/decline", post(decline_request))
}

async fn list_requests() -> &'static str {
    "list_requests"
}

async fn create_request() -> &'static str {
    "create_request"
}

async fn get_request() -> &'static str {
    "get_request"
}

async fn approve_request() -> &'static str {
    "approve_request"
}

async fn decline_request() -> &'static str {
    "decline_request"
}
