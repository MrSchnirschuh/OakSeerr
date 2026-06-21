use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Mylar3Comic {
    pub id: i64,
    pub title: String,
    pub publisher: Option<String>,
    pub year: Option<i32>,
    pub status: String,
    pub monitored: bool,
    pub size_on_disk: i64,
}

pub async fn get_comics(base_url: &str, api_key: &str) -> anyhow::Result<Vec<Mylar3Comic>> {
    let url = format!("{}/api/v2/comics", base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("X-Api-Key", api_key)
        .send()
        .await?;
    let comics: Vec<Mylar3Comic> = resp.json().await?;
    Ok(comics)
}

pub async fn add_comic(
    base_url: &str,
    api_key: &str,
    comic_id: &str,
) -> anyhow::Result<serde_json::Value> {
    let url = format!("{}/api/v2/comics", base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "id": comic_id,
        "monitored": true
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
