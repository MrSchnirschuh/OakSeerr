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
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub media_type: Option<String>,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/search", get(search_media))
        .route("/{id}", get(get_media))
        .route("/trending", get(get_trending))
}

async fn search_media(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Json<Vec<MediaItem>> {
    // Try to search via configured integrations
    let integrations = state.db.list_integrations().await.unwrap_or_default();
    let mut results = Vec::new();

    for integration in &integrations {
        if !integration.enabled { continue; }
        let media_type = query.media_type.as_deref().unwrap_or("");
        if !media_type.is_empty() && integration.integration_type != media_type {
            continue;
        }
        // Search via the integration's API
        if let Ok(items) = search_via_integration(integration, &query.q).await {
            results.extend(items);
        }
    }

    // If no integrations or search failed, return mock data
    if results.is_empty() {
        results = vec![
            MediaItem { id: "1".to_string(), title: "Dune: Part Two".to_string(), year: Some(2024), media_type: "movie".to_string(), poster_url: None, backdrop_url: None, overview: Some("Paul Atreides continues his journey.".to_string()), status: "available".to_string() },
            MediaItem { id: "2".to_string(), title: "The Batman".to_string(), year: Some(2022), media_type: "movie".to_string(), poster_url: None, backdrop_url: None, overview: Some("When a sadistic serial killer begins murdering key political figures in Gotham, Batman is forced to investigate the city's hidden corruption.".to_string()), status: "requested".to_string() },
            MediaItem { id: "3".to_string(), title: "Interstellar".to_string(), year: Some(2014), media_type: "movie".to_string(), poster_url: None, backdrop_url: None, overview: Some("A team of explorers travel through a wormhole in space.".to_string()), status: "available".to_string() },
            MediaItem { id: "4".to_string(), title: "Severance".to_string(), year: Some(2022), media_type: "tv".to_string(), poster_url: None, backdrop_url: None, overview: Some("Mark leads a team of office workers whose memories have been surgically divided.".to_string()), status: "available".to_string() },
            MediaItem { id: "5".to_string(), title: "The Dark Side of the Moon".to_string(), year: Some(1973), media_type: "music".to_string(), poster_url: None, backdrop_url: None, overview: Some("Pink Floyd's iconic album.".to_string()), status: "available".to_string() },
            MediaItem { id: "6".to_string(), title: "Dune".to_string(), year: Some(1965), media_type: "book".to_string(), poster_url: None, backdrop_url: None, overview: Some("Set on the desert planet Arrakis, Dune is the story of Paul Atreides.".to_string()), status: "available".to_string() },
            MediaItem { id: "7".to_string(), title: "Watchmen".to_string(), year: Some(1986), media_type: "comic".to_string(), poster_url: None, backdrop_url: None, overview: Some("In an alternate history, masked vigilantes are treated as outlaws.".to_string()), status: "available".to_string() },
        ];
        if !query.q.is_empty() {
            let q = query.q.to_lowercase();
            results.retain(|m| m.title.to_lowercase().contains(&q));
        }
    }

    Json(results)
}

async fn get_media(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<MediaItem> {
    // Try to get from DB first
    if let Ok(Some(media)) = state.db.get_media(&id).await {
        return Json(MediaItem {
            id: media.id,
            title: media.title,
            year: media.release_date.as_ref().and_then(|d| d[..4].parse().ok()),
            media_type: media.media_type,
            poster_url: media.poster_url,
            backdrop_url: media.backdrop_url,
            overview: media.overview,
            status: media.status,
        });
    }

    // Fallback mock
    Json(MediaItem {
        id,
        title: "Unknown Media".to_string(),
        year: None,
        media_type: "movie".to_string(),
        poster_url: None,
        backdrop_url: None,
        overview: None,
        status: "unknown".to_string(),
    })
}

async fn get_trending(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<MediaItem>> {
    // Try to get trending from integrations
    let integrations = state.db.list_integrations().await.unwrap_or_default();
    let mut results = Vec::new();

    for integration in &integrations {
        if !integration.enabled { continue; }
        if let Ok(items) = get_trending_via_integration(integration).await {
            results.extend(items);
        }
    }

    if results.is_empty() {
        results = vec![
            MediaItem { id: "1".to_string(), title: "Dune: Part Two".to_string(), year: Some(2024), media_type: "movie".to_string(), poster_url: None, backdrop_url: None, overview: None, status: "available".to_string() },
            MediaItem { id: "2".to_string(), title: "The Batman".to_string(), year: Some(2022), media_type: "movie".to_string(), poster_url: None, backdrop_url: None, overview: None, status: "requested".to_string() },
            MediaItem { id: "3".to_string(), title: "Interstellar".to_string(), year: Some(2014), media_type: "movie".to_string(), poster_url: None, backdrop_url: None, overview: None, status: "available".to_string() },
            MediaItem { id: "4".to_string(), title: "Blade Runner 2049".to_string(), year: Some(2017), media_type: "movie".to_string(), poster_url: None, backdrop_url: None, overview: None, status: "available".to_string() },
            MediaItem { id: "5".to_string(), title: "Everything Everywhere All at Once".to_string(), year: Some(2022), media_type: "movie".to_string(), poster_url: None, backdrop_url: None, overview: None, status: "processing".to_string() },
            MediaItem { id: "6".to_string(), title: "The Matrix".to_string(), year: Some(1999), media_type: "movie".to_string(), poster_url: None, backdrop_url: None, overview: None, status: "available".to_string() },
        ];
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
        _ => "unknown",
    };

    Ok(items.iter().enumerate().map(|(i, item)| {
        let title = item["title"].as_str().or(item["name"].as_str()).unwrap_or("Unknown").to_string();
        let year = item["year"].as_i64().or(item["releaseYear"].as_i64()).map(|y| y as i32);
        let overview = item["overview"].as_str().map(|s| s.to_string());
        let poster = item["images"].as_array()
            .and_then(|imgs| imgs.iter().find(|img| img["coverType"].as_str() == Some("poster")))
            .and_then(|img| img["remoteUrl"].as_str().map(|s| s.to_string()));

        MediaItem {
            id: format!("{}-{}", media_type, item["id"].as_i64().unwrap_or(i as i64)),
            title,
            year,
            media_type: media_type.to_string(),
            poster_url: poster,
            backdrop_url: None,
            overview,
            status: "unknown".to_string(),
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
        _ => "unknown",
    };

    Ok(items.iter().take(10).map(|item| {
        let title = item["title"].as_str().or(item["name"].as_str()).unwrap_or("Unknown").to_string();
        let year = item["year"].as_i64().or(item["releaseYear"].as_i64()).map(|y| y as i32);
        let overview = item["overview"].as_str().map(|s| s.to_string());
        let poster = item["images"].as_array()
            .and_then(|imgs| imgs.iter().find(|img| img["coverType"].as_str() == Some("poster")))
            .and_then(|img| img["remoteUrl"].as_str().map(|s| s.to_string()));

        MediaItem {
            id: format!("{}-{}", media_type, item["id"].as_i64().unwrap_or(0)),
            title,
            year,
            media_type: media_type.to_string(),
            poster_url: poster,
            backdrop_url: None,
            overview,
            status: "available".to_string(),
        }
    }).collect())
}
