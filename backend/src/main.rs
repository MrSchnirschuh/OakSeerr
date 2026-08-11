use axum::{Json, Router, response::IntoResponse, routing::get};
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use oakseerr::{AppState, download_status_poller};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "oakseerr=info,tower_http=info".into()),
        )
        .init();

    // Load config
    let config = oakseerr::config::Config::from_env()?;
    tracing::info!("OakSeerr v{} starting...", env!("CARGO_PKG_VERSION"));

    // Initialize database
    let db = oakseerr::db::Database::new(&config.database_url).await?;
    db.run_migrations().await?;

    let state = Arc::new(AppState {
        db,
        config: config.clone(),
        auth_service: oakseerr::services::auth::AuthService::new(&config.jwt_secret),
    });

    // Start background download status polling
    let poll_state = state.clone();
    tokio::spawn(async move {
        download_status_poller(poll_state).await;
    });

    let addr = state.config.listen_addr.clone();
    let cors_origin = state.config.cors_origin.clone();

    // Build router
    let frontend_path: PathBuf = std::env::var("OAKSEERR_FRONTEND_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            [env!("CARGO_MANIFEST_DIR"), "..", "frontend", "out"]
                .iter()
                .collect()
        });

    let app = Router::new()
        .route("/api/health", get(health_check))
        .nest("/api/v1/auth", oakseerr::api::auth::router())
        .nest("/api/v1/requests", oakseerr::api::requests::router())
        .nest("/api/v1/media", oakseerr::api::media::router())
        .nest("/api/v1/settings", oakseerr::api::settings::router())
        .nest(
            "/api/v1/integrations",
            oakseerr::api::integrations::router(),
        )
        .nest("/api/v1/users", oakseerr::api::users::router())
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(
                    move |origin: &axum::http::HeaderValue,
                          _request_parts: &axum::http::request::Parts| {
                        origin.as_bytes() == cors_origin.as_bytes()
                    },
                ))
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::PUT,
                    axum::http::Method::DELETE,
                    axum::http::Method::OPTIONS,
                ])
                .allow_headers([
                    axum::http::header::AUTHORIZATION,
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::ACCEPT,
                ]),
        )
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
