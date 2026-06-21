use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SabnzbdQueue {
    pub status: String,
    pub slots: Vec<SabnzbdSlot>,
    pub kb_left: i64,
    pub kb_per_sec: f64,
    pub estimated_time: String,
    pub paused: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SabnzbdSlot {
    pub nzo_id: String,
    pub filename: String,
    pub status: String,
    pub mb_left: f64,
    pub mb: f64,
    pub percentage: f64,
    pub eta: String,
    pub priority: String,
}

pub async fn get_queue(base_url: &str, api_key: &str) -> anyhow::Result<SabnzbdQueue> {
    let url = format!(
        "{}/sabnzbd/api?mode=queue&output=json&apikey={}",
        base_url.trim_end_matches('/'),
        api_key
    );
    let client = reqwest::Client::new();
    let resp = client.get(&url).send().await?;
    let data: serde_json::Value = resp.json().await?;
    let queue: SabnzbdQueue = serde_json::from_value(data["queue"].clone())?;
    Ok(queue)
}

pub async fn get_history(base_url: &str, api_key: &str) -> anyhow::Result<serde_json::Value> {
    let url = format!(
        "{}/sabnzbd/api?mode=history&output=json&apikey={}",
        base_url.trim_end_matches('/'),
        api_key
    );
    let client = reqwest::Client::new();
    let resp = client.get(&url).send().await?;
    let data: serde_json::Value = resp.json().await?;
    Ok(data)
}
