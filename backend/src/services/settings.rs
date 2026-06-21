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

    pub async fn set(db: &Database, key: &str, value: &str) -> anyhow::Result<()> {
        db.set_setting(key, value).await?;
        Ok(())
    }
}
