use crate::AppState;
use crate::models::Media;
use crate::services::media::{MediaDetail, MediaService};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaItem {
    pub id: String,
    pub title: String,
    pub year: Option<i32>,
    pub media_type: String,
    pub poster_url: Option<String>,
    pub backdrop_url: Option<String>,
    pub overview: Option<String>,
    pub status: String,
    pub rating: Option<f64>,
    pub genres: Option<Vec<String>>,
    pub season_count: Option<i32>,
    pub episode_count: Option<i32>,
    pub artist_name: Option<String>,
    pub author_name: Option<String>,
}

impl From<Media> for MediaItem {
    fn from(m: Media) -> Self {
        let year = m.release_date.as_ref().and_then(|d| d[..4].parse().ok());
        // Parse genres from JSON string stored in DB
        let genres: Option<Vec<String>> =
            m.genres.as_ref().and_then(|g| serde_json::from_str(g).ok());
        MediaItem {
            id: m.id,
            title: m.title,
            year,
            media_type: m.media_type,
            poster_url: m.poster_url,
            backdrop_url: m.backdrop_url,
            overview: m.overview,
            status: m.status,
            rating: m.rating,
            genres,
            season_count: m.season_count,
            episode_count: m.episode_count,
            artist_name: m.artist_name,
            author_name: m.author_name,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub media_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TrendingQuery {
    pub media_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LibraryQuery {
    pub media_type: Option<String>,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/search", get(search_media))
        .route("/trending", get(get_trending))
        .route("/library", get(get_library))
        .route("/{id}", get(get_media))
        .route("/{id}/detail", get(get_media_detail))
}

async fn search_media(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Json<Vec<MediaItem>> {
    let results = MediaService::search(&state.db, &query.q, query.media_type.as_deref())
        .await
        .unwrap_or_default();
    Json(results.into_iter().map(MediaItem::from).collect())
}

async fn get_trending(
    State(state): State<Arc<AppState>>,
    Query(query): Query<TrendingQuery>,
) -> Json<Vec<MediaItem>> {
    let results = MediaService::trending(&state.db, query.media_type.as_deref())
        .await
        .unwrap_or_default();
    Json(results.into_iter().map(MediaItem::from).collect())
}

async fn get_library(
    State(state): State<Arc<AppState>>,
    Query(query): Query<LibraryQuery>,
) -> Json<Vec<MediaItem>> {
    let media_type = query.media_type.as_deref().unwrap_or("movie");
    let results = MediaService::library(&state.db, media_type)
        .await
        .unwrap_or_default();
    Json(results.into_iter().map(MediaItem::from).collect())
}

async fn get_media(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<Option<MediaItem>> {
    let media = state.db.get_media(&id).await.unwrap_or(None);
    Json(media.map(MediaItem::from))
}

async fn get_media_detail(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<MediaDetail>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let detail = MediaService::detail(&state.db, &id)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
        })?
        .ok_or_else(|| {
            (
                axum::http::StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Media not found"})),
            )
        })?;

    Ok(Json(detail))
}
