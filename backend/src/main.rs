use axum::{
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use std::path::PathBuf;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

mod api;
mod config;
mod db;
mod integrations;
mod models;
mod services;

pub struct AppState {
    pub db: db::Database,
    pub config: config::Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "oakseerr=info,tower_http=info".into()))
        .init();

    // Load config
    let config = config::Config::from_env()?;
    tracing::info!("OakSeerr v{} starting...", env!("CARGO_PKG_VERSION"));

    // Initialize database
    let db = db::Database::new(&config.database_url).await?;
    db.run_migrations().await?;

    let state = Arc::new(AppState { db, config });

    let addr = state.config.listen_addr.clone();

    // Build router
    let frontend_path: PathBuf = std::env::var("OAKSEERR_FRONTEND_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            [env!("CARGO_MANIFEST_DIR"), "..", "frontend", "out"].iter().collect()
        });
    let app = Router::new()
        .route("/api/health", get(health_check))
        .nest("/api/v1/auth", api::auth::router())
        .nest("/api/v1/requests", api::requests::router())
        .nest("/api/v1/media", api::media::router())
        .nest("/api/v1/settings", api::settings::router())
        .nest("/api/v1/integrations", api::integrations::router())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
        .fallback_service(ServeDir::new(&frontend_path).append_index_html_on_directories(true));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
