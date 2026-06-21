use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RadarrMovie {
    pub id: i64,
    pub title: String,
    pub tmdb_id: i64,
    pub year: Option<i32>,
    pub status: String,
    pub monitored: bool,
    pub has_file: bool,
    pub size_on_disk: i64,
    pub download_status: Option<String>,
    pub estimated_completion_time: Option<String>,
}

pub async fn get_movies(base_url: &str, api_key: &str) -> anyhow::Result<Vec<RadarrMovie>> {
    let url = format!("{}/api/v3/movie", base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("X-Api-Key", api_key)
        .send()
        .await?;
    let movies: Vec<RadarrMovie> = resp.json().await?;
    Ok(movies)
}

pub async fn add_movie(
    base_url: &str,
    api_key: &str,
    tmdb_id: i64,
    quality_profile_id: i64,
    root_folder_path: &str,
) -> anyhow::Result<serde_json::Value> {
    let url = format!("{}/api/v3/movie", base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "tmdbId": tmdb_id,
        "qualityProfileId": quality_profile_id,
        "rootFolderPath": root_folder_path,
        "monitored": true,
        "addOptions": {
            "searchForMovie": true
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
