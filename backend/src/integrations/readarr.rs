use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadarrBook {
    pub id: i64,
    pub title: String,
    pub author_name: String,
    pub isbn: Option<String>,
    pub goodreads_id: Option<i64>,
    pub status: String,
    pub monitored: bool,
    pub has_file: bool,
    pub size_on_disk: i64,
}

pub async fn get_books(base_url: &str, api_key: &str) -> anyhow::Result<Vec<ReadarrBook>> {
    let url = format!("{}/api/v1/book", base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("X-Api-Key", api_key)
        .send()
        .await?;
    let books: Vec<ReadarrBook> = resp.json().await?;
    Ok(books)
}

pub async fn add_book(
    base_url: &str,
    api_key: &str,
    foreign_id: &str,
    quality_profile_id: i64,
    root_folder_path: &str,
) -> anyhow::Result<serde_json::Value> {
    let url = format!("{}/api/v1/book", base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "foreignId": foreign_id,
        "qualityProfileId": quality_profile_id,
        "rootFolderPath": root_folder_path,
        "monitored": true,
        "addOptions": {
            "searchForNewBook": true
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
