use crate::models::Integration;
use serde_json::Value;

pub struct IntegrationService;

impl IntegrationService {
    /// Test if an integration is reachable
    pub async fn test(integration: &Integration) -> anyhow::Result<String> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        let url = format!(
            "{}/api/v3/{}?apikey={}&limit=1",
            integration.base_url.trim_end_matches('/'),
            match integration.integration_type.as_str() {
                "radarr" => "movie",
                "sonarr" => "series",
                "lidarr" => "artist",
                "readarr" => "book",
                "mylar3" => "comic",
                _ => "system/status",
            },
            integration.api_key,
        );

        let resp = client.get(&url).send().await?;

        if resp.status().is_success() {
            let data: Value = resp.json().await?;
            let version = data
                .get("version")
                .or_else(|| data.get("appVersion"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            Ok(format!("Connected (v{})", version))
        } else {
            Err(anyhow::anyhow!("Connection failed: HTTP {}", resp.status()))
        }
    }
}
