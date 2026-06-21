use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub jellyfin_user_id: Option<String>,
    pub permissions: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MediaRequest {
    pub id: String,
    pub user_id: String,
    pub media_type: String, // movie, tv, music, book, comic
    pub media_id: String,
    pub title: String,
    pub status: String, // pending, approved, declined, fulfilled
    pub download_status: String, // none, queued, downloading, imported, failed
    pub external_service_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Media {
    pub id: String,
    pub tmdb_id: Option<i64>,
    pub tvdb_id: Option<i64>,
    pub musicbrainz_id: Option<String>,
    pub isbn: Option<String>,
    pub media_type: String,
    pub title: String,
    pub overview: Option<String>,
    pub poster_url: Option<String>,
    pub backdrop_url: Option<String>,
    pub release_date: Option<String>,
    pub status: String, // unknown, available, requested, processing
    pub rating: Option<f64>,
    pub genres: Option<String>, // JSON array stored as string
    pub season_count: Option<i32>,
    pub episode_count: Option<i32>,
    pub artist_name: Option<String>,
    pub author_name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Integration {
    pub id: String,
    pub name: String,
    pub integration_type: String, // radarr, sonarr, lidarr, readarr, mylar3, sabnzbd, prowlarr
    pub base_url: String,
    pub api_key: String,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Settings {
    pub key: String,
    pub value: String,
}
