use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::AppState;

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

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub media_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TrendingQuery {
    pub media_type: Option<String>,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/search", get(search_media))
        .route("/{id}", get(get_media))
        .route("/trending", get(get_trending))
}

fn map_media_type_to_integration(media_type: &str) -> &str {
    match media_type {
        "movie" => "radarr",
        "tv" => "sonarr",
        "music" => "lidarr",
        "book" => "readarr",
        "comic" => "mylar3",
        _ => media_type,
    }
}

async fn search_media(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Json<Vec<MediaItem>> {
    let integrations = state.db.list_integrations().await.unwrap_or_default();
    let mut results = Vec::new();

    for integration in &integrations {
        if !integration.enabled { continue; }
        let media_type = query.media_type.as_deref().unwrap_or("");
        let integration_type = if media_type.is_empty() { "" } else { map_media_type_to_integration(media_type) };
        if !integration_type.is_empty() && integration.integration_type != integration_type {
            continue;
        }
        if let Ok(items) = search_via_integration(integration, &query.q).await {
            results.extend(items);
        }
    }

    Json(results)
}

async fn get_media(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<Option<MediaItem>> {
    if let Ok(Some(media)) = state.db.get_media(&id).await {
        return Json(Some(MediaItem {
            id: media.id,
            title: media.title,
            year: media.release_date.as_ref().and_then(|d| d[..4].parse().ok()),
            media_type: media.media_type,
            poster_url: media.poster_url,
            backdrop_url: media.backdrop_url,
            overview: media.overview,
            status: media.status,
            rating: None,
            genres: None,
            season_count: None,
            episode_count: None,
            artist_name: None,
            author_name: None,
        }));
    }

    Json(None)
}

async fn get_trending(
    State(state): State<Arc<AppState>>,
    Query(query): Query<TrendingQuery>,
) -> Json<Vec<MediaItem>> {
    let integrations = state.db.list_integrations().await.unwrap_or_default();
    let mut results = Vec::new();

    for integration in &integrations {
        if !integration.enabled { continue; }
        // Filter by media_type if specified
        if let Some(ref mt) = query.media_type {
            let integration_type = match mt.as_str() {
                "movie" => "radarr",
                "tv" => "sonarr",
                "music" => "lidarr",
                "book" => "readarr",
                "comic" => "mylar3",
                _ => "",
            };
            if integration.integration_type != integration_type {
                continue;
            }
        }
        if let Ok(items) = get_trending_via_integration(integration).await {
            results.extend(items);
        }
    }

    Json(results)
}

async fn search_via_integration(
    integration: &crate::models::Integration,
    query: &str,
) -> Result<Vec<MediaItem>, String> {
    let client = reqwest::Client::new();
    let base = integration.base_url.trim_end_matches('/');

    let url = match integration.integration_type.as_str() {
        "radarr" => format!("{}/api/v3/movie/lookup?term={}", base, query),
        "sonarr" => format!("{}/api/v3/series/lookup?term={}", base, query),
        "lidarr" => format!("{}/api/v1/artist/lookup?term={}", base, query),
        "readarr" => format!("{}/api/v1/author/lookup?query={}", base, query),
        "mylar3" => format!("{}/api/v1/search?query={}", base, query),
        _ => return Err("Unsupported for search".to_string()),
    };

    let res = client.get(&url)
        .header("X-Api-Key", &integration.api_key)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("HTTP {}", res.status()));
    }

    let items: Vec<serde_json::Value> = res.json().await.map_err(|e| format!("Parse failed: {}", e))?;
    let media_type = match integration.integration_type.as_str() {
        "radarr" => "movie",
        "sonarr" => "tv",
        "lidarr" => "music",
        "readarr" => "book",
        "mylar3" => "comic",
        _ => "unknown",
    };

    Ok(items.iter().enumerate().map(|(i, item)| {
        let title = item["title"].as_str()
            .or(item["name"].as_str())
            .or(item["seriesName"].as_str())
            .unwrap_or("Unknown").to_string();
        let year = item["year"].as_i64()
            .or(item["releaseYear"].as_i64())
            .or(item["releaseDate"].as_str().and_then(|d| d[..4].parse().ok()))
            .map(|y| y as i32);
        let overview = item["overview"].as_str()
            .or(item["description"].as_str())
            .map(|s| s.to_string());
        let poster = item["images"].as_array()
            .and_then(|imgs| imgs.iter().find(|img| img["coverType"].as_str() == Some("poster")))
            .and_then(|img| img["remoteUrl"].as_str().map(|s| s.to_string()))
            .or(item["posterUrl"].as_str().map(|s| s.to_string()));
        let rating = item["ratings"].as_object()
            .and_then(|r| r["value"].as_f64())
            .or(item["rating"].as_f64());
        let genres = item["genres"].as_array()
            .map(|g| g.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect());
        let artist_name = item["artistName"].as_str().map(|s| s.to_string());
        let author_name = item["authorName"].as_str().map(|s| s.to_string());

        MediaItem {
            id: format!("{}-{}", media_type, item["id"].as_i64().unwrap_or(i as i64)),
            title,
            year,
            media_type: media_type.to_string(),
            poster_url: poster,
            backdrop_url: None,
            overview,
            status: "unknown".to_string(),
            rating,
            genres,
            season_count: None,
            episode_count: None,
            artist_name,
            author_name,
        }
    }).collect())
}

async fn get_trending_via_integration(
    integration: &crate::models::Integration,
) -> Result<Vec<MediaItem>, String> {
    let client = reqwest::Client::new();
    let base = integration.base_url.trim_end_matches('/');

    let url = match integration.integration_type.as_str() {
        "radarr" => format!("{}/api/v3/movie", base),
        "sonarr" => format!("{}/api/v3/series", base),
        "lidarr" => format!("{}/api/v1/artist", base),
        "readarr" => format!("{}/api/v1/author", base),
        "mylar3" => format!("{}/api/v1/series", base),
        _ => return Err("Unsupported for trending".to_string()),
    };

    let res = client.get(&url)
        .header("X-Api-Key", &integration.api_key)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("HTTP {}", res.status()));
    }

    let items: Vec<serde_json::Value> = res.json().await.map_err(|e| format!("Parse failed: {}", e))?;
    let media_type = match integration.integration_type.as_str() {
        "radarr" => "movie",
        "sonarr" => "tv",
        "lidarr" => "music",
        "readarr" => "book",
        "mylar3" => "comic",
        _ => "unknown",
    };

    Ok(items.iter().take(20).map(|item| {
        let title = item["title"].as_str()
            .or(item["name"].as_str())
            .or(item["seriesName"].as_str())
            .unwrap_or("Unknown").to_string();
        let year = item["year"].as_i64()
            .or(item["releaseYear"].as_i64())
            .or(item["releaseDate"].as_str().and_then(|d| d[..4].parse().ok()))
            .map(|y| y as i32);
        let overview = item["overview"].as_str()
            .or(item["description"].as_str())
            .map(|s| s.to_string());
        let poster = item["images"].as_array()
            .and_then(|imgs| imgs.iter().find(|img| img["coverType"].as_str() == Some("poster")))
            .and_then(|img| img["remoteUrl"].as_str().map(|s| s.to_string()))
            .or(item["posterUrl"].as_str().map(|s| s.to_string()));
        let rating = item["ratings"].as_object()
            .and_then(|r| r["value"].as_f64())
            .or(item["rating"].as_f64());
        let genres = item["genres"].as_array()
            .map(|g| g.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect());
        let artist_name = item["artistName"].as_str().map(|s| s.to_string());
        let author_name = item["authorName"].as_str().map(|s| s.to_string());

        MediaItem {
            id: format!("{}-{}", media_type, item["id"].as_i64().unwrap_or(0)),
            title,
            year,
            media_type: media_type.to_string(),
            poster_url: poster,
            backdrop_url: None,
            overview,
            status: "available".to_string(),
            rating,
            genres,
            season_count: None,
            episode_count: None,
            artist_name,
            author_name,
        }
    }).collect())
}
