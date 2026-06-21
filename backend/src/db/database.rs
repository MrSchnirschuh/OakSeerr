use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{SqlitePool, Row};
use crate::models::{User, Media, MediaRequest, Integration, Settings};

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?;
        Ok(Database { pool })
    }

    pub async fn run_migrations(&self) -> anyhow::Result<()> {
        sqlx::migrate!("src/db/migrations")
            .run(&self.pool)
            .await?;
        tracing::info!("Database migrations applied successfully");

        // Ensure demo user exists
        sqlx::query(
            "INSERT OR IGNORE INTO users (id, username, display_name, permissions, created_at, updated_at) VALUES ('demo-user', 'demo', 'Demo User', 100, datetime('now'), datetime('now'))"
        ).execute(&self.pool).await?;

        Ok(())
    }

    // === Users ===
    pub async fn get_user(&self, id: &str) -> anyhow::Result<Option<User>> {
        let row = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }

    // === Integrations ===
    pub async fn list_integrations(&self) -> anyhow::Result<Vec<Integration>> {
        let rows = sqlx::query_as::<_, Integration>("SELECT * FROM integrations ORDER BY name")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows)
    }

    pub async fn get_integration(&self, id: &str) -> anyhow::Result<Option<Integration>> {
        let row = sqlx::query_as::<_, Integration>("SELECT * FROM integrations WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }

    pub async fn create_integration(&self, integration: &Integration) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO integrations (id, name, integration_type, base_url, api_key, enabled, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
            .bind(&integration.id)
            .bind(&integration.name)
            .bind(&integration.integration_type)
            .bind(&integration.base_url)
            .bind(&integration.api_key)
            .bind(integration.enabled as i32)
            .bind(&integration.created_at)
            .bind(&integration.updated_at)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_integration(&self, integration: &Integration) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE integrations SET name = ?, base_url = ?, api_key = ?, enabled = ?, updated_at = ? WHERE id = ?"
        )
            .bind(&integration.name)
            .bind(&integration.base_url)
            .bind(&integration.api_key)
            .bind(integration.enabled as i32)
            .bind(&integration.updated_at)
            .bind(&integration.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_integration(&self, id: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM integrations WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // === Media ===
    pub async fn get_media(&self, id: &str) -> anyhow::Result<Option<Media>> {
        let row = sqlx::query_as::<_, Media>("SELECT * FROM media WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }

    pub async fn create_media(&self, media: &Media) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO media (id, tmdb_id, tvdb_id, musicbrainz_id, isbn, media_type, title, overview, poster_url, backdrop_url, release_date, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
            .bind(&media.id)
            .bind(media.tmdb_id)
            .bind(media.tvdb_id)
            .bind(&media.musicbrainz_id)
            .bind(&media.isbn)
            .bind(&media.media_type)
            .bind(&media.title)
            .bind(&media.overview)
            .bind(&media.poster_url)
            .bind(&media.backdrop_url)
            .bind(&media.release_date)
            .bind(&media.status)
            .bind(&media.created_at)
            .bind(&media.updated_at)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // === Requests ===
    pub async fn list_requests(&self) -> anyhow::Result<Vec<MediaRequest>> {
        let rows = sqlx::query_as::<_, MediaRequest>("SELECT * FROM media_requests ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows)
    }

    pub async fn get_request(&self, id: &str) -> anyhow::Result<Option<MediaRequest>> {
        let row = sqlx::query_as::<_, MediaRequest>("SELECT * FROM media_requests WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }

    pub async fn create_request(&self, request: &MediaRequest) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO media_requests (id, user_id, media_type, media_id, title, status, external_service_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
            .bind(&request.id)
            .bind(&request.user_id)
            .bind(&request.media_type)
            .bind(&request.media_id)
            .bind(&request.title)
            .bind(&request.status)
            .bind(&request.external_service_id)
            .bind(&request.created_at)
            .bind(&request.updated_at)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_request(&self, request: &MediaRequest) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE media_requests SET status = ?, external_service_id = ?, updated_at = ? WHERE id = ?"
        )
            .bind(&request.status)
            .bind(&request.external_service_id)
            .bind(&request.updated_at)
            .bind(&request.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // === Settings ===
    pub async fn get_setting(&self, key: &str) -> anyhow::Result<Option<String>> {
        let row = sqlx::query("SELECT value FROM settings WHERE key = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| r.get::<String, _>(0)))
    }

    pub async fn set_setting(&self, key: &str, value: &str) -> anyhow::Result<()> {
        sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)")
            .bind(key)
            .bind(value)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn list_settings(&self) -> anyhow::Result<Vec<Settings>> {
        let rows = sqlx::query_as::<_, Settings>("SELECT * FROM settings ORDER BY key")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows)
    }
}
