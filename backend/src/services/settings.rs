use crate::db::Database;

pub struct SettingsService;

impl SettingsService {
    pub async fn get_all(db: &Database) -> anyhow::Result<std::collections::HashMap<String, String>> {
        let settings = db.list_settings().await?;
        let map: std::collections::HashMap<String, String> = settings.into_iter()
            .map(|s| (s.key, s.value))
            .collect();
        Ok(map)
    }

    pub async fn get(db: &Database, key: &str) -> anyhow::Result<Option<String>> {
        db.get_setting(key).await
    }

    pub async fn set(db: &Database, key: &str, value: &str) -> anyhow::Result<()> {
        db.set_setting(key, value).await?;
        Ok(())
    }

    /// Get an API key from settings, with a fallback default
    pub async fn get_api_key(db: &Database, key_name: &str, default: &str) -> anyhow::Result<String> {
        let val = db.get_setting(key_name).await?;
        Ok(val.unwrap_or_else(|| default.to_string()))
    }
}
