use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LidarrArtist {
    pub id: i64,
    pub artist_name: String,
    pub musicbrainz_id: Option<String>,
    pub status: String,
    pub monitored: bool,
    pub album_count: i32,
    pub size_on_disk: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LidarrAlbum {
    pub id: i64,
    pub title: String,
    pub musicbrainz_id: Option<String>,
    pub release_date: Option<String>,
    pub monitored: bool,
}

pub async fn get_artists(base_url: &str, api_key: &str) -> anyhow::Result<Vec<LidarrArtist>> {
    let url = format!("{}/api/v1/artist", base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("X-Api-Key", api_key)
        .send()
        .await?;
    let artists: Vec<LidarrArtist> = resp.json().await?;
    Ok(artists)
}

pub async fn get_albums(base_url: &str, api_key: &str, artist_id: i64) -> anyhow::Result<Vec<LidarrAlbum>> {
    let url = format!("{}/api/v1/album?artistId={}", base_url.trim_end_matches('/'), artist_id);
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("X-Api-Key", api_key)
        .send()
        .await?;
    let albums: Vec<LidarrAlbum> = resp.json().await?;
    Ok(albums)
}

pub async fn add_artist(
    base_url: &str,
    api_key: &str,
    musicbrainz_id: &str,
    quality_profile_id: i64,
    root_folder_path: &str,
) -> anyhow::Result<serde_json::Value> {
    let url = format!("{}/api/v1/artist", base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "foreignArtistId": musicbrainz_id,
        "qualityProfileId": quality_profile_id,
        "rootFolderPath": root_folder_path,
        "monitored": true,
        "addOptions": {
            "searchForNewAlbums": true
        }
    });
    let resp = client
        .post(&url)
        .header("X-Api-Key", api_key)
        .json(&body)
        .send()
        .await?;
    let result: serde_json::Value = resp.json().await?;
    Ok(result)
}
