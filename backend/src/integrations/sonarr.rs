use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SonarrSeries {
    pub id: i64,
    pub title: String,
    pub tvdb_id: i64,
    pub year: Option<i32>,
    pub status: String,
    pub monitored: bool,
    pub season_count: i32,
    pub episode_count: i32,
    pub size_on_disk: i64,
}

pub async fn get_series(base_url: &str, api_key: &str) -> anyhow::Result<Vec<SonarrSeries>> {
    let url = format!("{}/api/v3/series", base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("X-Api-Key", api_key)
        .send()
        .await?;
    let series: Vec<SonarrSeries> = resp.json().await?;
    Ok(series)
}

pub async fn add_series(
    base_url: &str,
    api_key: &str,
    tvdb_id: i64,
    quality_profile_id: i64,
    root_folder_path: &str,
    seasons: Vec<i32>,
) -> anyhow::Result<serde_json::Value> {
    let url = format!("{}/api/v3/series", base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let seasons_json: Vec<serde_json::Value> = seasons
        .iter()
        .map(|s| {
            serde_json::json!({
                "seasonNumber": s,
                "monitored": true
            })
        })
        .collect();
    let body = serde_json::json!({
        "tvdbId": tvdb_id,
        "qualityProfileId": quality_profile_id,
        "rootFolderPath": root_folder_path,
        "monitored": true,
        "seasons": seasons_json,
        "addOptions": {
            "searchForMissingEpisodes": true
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
