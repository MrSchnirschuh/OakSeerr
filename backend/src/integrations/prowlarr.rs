use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProwlarrIndexer {
    pub id: i64,
    pub name: String,
    pub protocol: String,
    pub priority: i32,
    pub enabled: bool,
}

pub async fn get_indexers(base_url: &str, api_key: &str) -> anyhow::Result<Vec<ProwlarrIndexer>> {
    let url = format!("{}/api/v1/indexer", base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("X-Api-Key", api_key)
        .send()
        .await?;
    let indexers: Vec<ProwlarrIndexer> = resp.json().await?;
    Ok(indexers)
}

pub async fn search(
    base_url: &str,
    api_key: &str,
    query: &str,
    media_type: &str,
) -> anyhow::Result<serde_json::Value> {
    let url = format!(
        "{}/api/v1/search?query={}&type={}",
        base_url.trim_end_matches('/'),
        query,
        media_type
    );
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("X-Api-Key", api_key)
        .send()
        .await?;
    let result: serde_json::Value = resp.json().await?;
    Ok(result)
}
