use axum::{
    routing::{get},
    Router,
};
use std::sync::Arc;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/search", get(search_media))
        .route("/{id}", get(get_media))
        .route("/trending", get(get_trending))
}

async fn search_media() -> &'static str {
    "search_media"
}

async fn get_media() -> &'static str {
    "get_media"
}

async fn get_trending() -> &'static str {
    "get_trending"
}
