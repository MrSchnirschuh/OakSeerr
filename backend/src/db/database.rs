use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

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
        Ok(())
    }
}
