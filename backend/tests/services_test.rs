use oakseerr::{
    config::Config,
    db::Database,
    models::{Integration, MediaRequest, User},
    services::{
        auth::AuthService,
        integrations::IntegrationService,
        requests::RequestService,
        settings::SettingsService,
    },
};
use uuid::Uuid;

/// Create an in-memory SQLite database for testing
async fn test_db() -> Database {
    let db = Database::new("sqlite::memory:").await.unwrap();
    // Run migrations manually (sqlx::migrate! needs a path, so we inline the schema)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            display_name TEXT NOT NULL,
            email TEXT,
            avatar_url TEXT,
            jellyfin_user_id TEXT,
            permissions INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )"
    ).execute(&db.pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS media_requests (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            media_type TEXT NOT NULL,
            media_id TEXT NOT NULL,
            title TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            download_status TEXT NOT NULL DEFAULT 'none',
            external_service_id TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )"
    ).execute(&db.pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS media (
            id TEXT PRIMARY KEY,
            tmdb_id INTEGER,
            tvdb_id INTEGER,
            musicbrainz_id TEXT,
            isbn TEXT,
            media_type TEXT NOT NULL,
            title TEXT NOT NULL,
            overview TEXT,
            poster_url TEXT,
            backdrop_url TEXT,
            release_date TEXT,
            status TEXT NOT NULL DEFAULT 'unknown',
            rating REAL,
            genres TEXT,
            season_count INTEGER,
            episode_count INTEGER,
            artist_name TEXT,
            author_name TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )"
    ).execute(&db.pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS integrations (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            integration_type TEXT NOT NULL,
            base_url TEXT NOT NULL,
            api_key TEXT NOT NULL,
            enabled INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )"
    ).execute(&db.pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )"
    ).execute(&db.pool).await.unwrap();

    db
}

fn test_user() -> User {
    User {
        id: "test-user-1".to_string(),
        username: "testuser".to_string(),
        display_name: "Test User".to_string(),
        email: Some("test@example.com".to_string()),
        avatar_url: None,
        jellyfin_user_id: None,
        permissions: 100,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    }
}

// ===== AuthService Tests =====

#[tokio::test]
async fn test_auth_create_token() {
    let auth = AuthService::new("test-secret");
    let user = test_user();
    let token = auth.create_token(&user).unwrap();
    assert!(!token.is_empty(), "Token should not be empty");
    // JWT has 3 parts separated by dots
    assert_eq!(token.matches('.').count(), 2, "JWT should have 3 parts");
}

#[tokio::test]
async fn test_auth_verify_valid_token() {
    let auth = AuthService::new("test-secret");
    let user = test_user();
    let token = auth.create_token(&user).unwrap();
    let claims = auth.verify_token(&token).unwrap();
    assert_eq!(claims.sub, "test-user-1");
    assert_eq!(claims.username, "testuser");
}

#[tokio::test]
async fn test_auth_verify_invalid_token() {
    let auth = AuthService::new("test-secret");
    let result = auth.verify_token("invalid.token.here");
    assert!(result.is_err(), "Invalid token should fail verification");
}

#[tokio::test]
async fn test_auth_verify_wrong_secret() {
    let auth1 = AuthService::new("secret-1");
    let auth2 = AuthService::new("secret-2");
    let user = test_user();
    let token = auth1.create_token(&user).unwrap();
    let result = auth2.verify_token(&token);
    assert!(result.is_err(), "Token signed with different secret should fail");
}

#[tokio::test]
async fn test_auth_create_demo_user() {
    let db = test_db().await;
    let auth = AuthService::new("test-secret");
    let (user, token) = auth.create_demo_user(&db).await.unwrap();
    assert_eq!(user.id, "demo-user");
    assert_eq!(user.username, "demo");
    assert!(!token.is_empty());

    // Second call should return existing user, not create a new one
    let (user2, _) = auth.create_demo_user(&db).await.unwrap();
    assert_eq!(user2.id, "demo-user");
}

// ===== RequestService Tests =====

#[tokio::test]
async fn test_request_create() {
    let db = test_db().await;
    let request = RequestService::create(
        &db,
        "user-1",
        "movie",
        "tmdb-550",
        "Fight Club",
    ).await.unwrap();

    assert_eq!(request.user_id, "user-1");
    assert_eq!(request.media_type, "movie");
    assert_eq!(request.media_id, "tmdb-550");
    assert_eq!(request.title, "Fight Club");
    assert_eq!(request.status, "pending");
    assert_eq!(request.download_status, "none");
    assert!(request.external_service_id.is_none());
}

#[tokio::test]
async fn test_request_approve() {
    let db = test_db().await;
    let request = RequestService::create(
        &db, "user-1", "movie", "tmdb-550", "Fight Club",
    ).await.unwrap();

    let approved = RequestService::approve(&db, &request.id).await.unwrap();
    assert_eq!(approved.status, "approved");
}

#[tokio::test]
async fn test_request_approve_not_found() {
    let db = test_db().await;
    let result = RequestService::approve(&db, "nonexistent-id").await;
    assert!(result.is_err(), "Approving nonexistent request should fail");
}

#[tokio::test]
async fn test_request_decline() {
    let db = test_db().await;
    let request = RequestService::create(
        &db, "user-1", "movie", "tmdb-550", "Fight Club",
    ).await.unwrap();

    let declined = RequestService::decline(&db, &request.id).await.unwrap();
    assert_eq!(declined.status, "declined");
}

#[tokio::test]
async fn test_request_update_download_status() {
    let db = test_db().await;
    let request = RequestService::create(
        &db, "user-1", "movie", "tmdb-550", "Fight Club",
    ).await.unwrap();

    let updated = RequestService::update_download_status(
        &db, &request.id, "imported",
    ).await.unwrap();
    assert_eq!(updated.download_status, "imported");
}

#[tokio::test]
async fn test_request_get_all_with_status() {
    let db = test_db().await;
    // Create a few requests
    RequestService::create(&db, "user-1", "movie", "tmdb-550", "Fight Club").await.unwrap();
    RequestService::create(&db, "user-1", "tv", "tmdb-1668", "Breaking Bad").await.unwrap();
    RequestService::create(&db, "user-2", "movie", "tmdb-680", "Pulp Fiction").await.unwrap();

    let all = RequestService::get_all_with_status(&db).await.unwrap();
    assert_eq!(all.len(), 3, "Should return all 3 requests");
}

// ===== Database Tests =====

#[tokio::test]
async fn test_db_create_and_get_user() {
    let db = test_db().await;
    let user = test_user();
    db.create_user(&user).await.unwrap();

    let fetched = db.get_user("test-user-1").await.unwrap().unwrap();
    assert_eq!(fetched.username, "testuser");
    assert_eq!(fetched.display_name, "Test User");
    assert_eq!(fetched.email, Some("test@example.com".to_string()));
}

#[tokio::test]
async fn test_db_get_user_not_found() {
    let db = test_db().await;
    let result = db.get_user("nonexistent").await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_db_list_users() {
    let db = test_db().await;
    db.create_user(&test_user()).await.unwrap();

    let mut user2 = test_user();
    user2.id = "test-user-2".to_string();
    user2.username = "testuser2".to_string();
    db.create_user(&user2).await.unwrap();

    let users = db.list_users().await.unwrap();
    assert_eq!(users.len(), 2);
}

#[tokio::test]
async fn test_db_update_user() {
    let db = test_db().await;
    db.create_user(&test_user()).await.unwrap();

    let mut user = db.get_user("test-user-1").await.unwrap().unwrap();
    user.display_name = "Updated Name".to_string();
    db.update_user(&user).await.unwrap();

    let updated = db.get_user("test-user-1").await.unwrap().unwrap();
    assert_eq!(updated.display_name, "Updated Name");
}

#[tokio::test]
async fn test_db_delete_user() {
    let db = test_db().await;
    db.create_user(&test_user()).await.unwrap();
    db.delete_user("test-user-1").await.unwrap();

    let result = db.get_user("test-user-1").await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_db_create_and_get_request() {
    let db = test_db().await;
    let request = MediaRequest {
        id: Uuid::new_v4().to_string(),
        user_id: "user-1".to_string(),
        media_type: "movie".to_string(),
        media_id: "tmdb-550".to_string(),
        title: "Fight Club".to_string(),
        status: "pending".to_string(),
        download_status: "none".to_string(),
        external_service_id: None,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    db.create_request(&request).await.unwrap();

    let fetched = db.get_request(&request.id).await.unwrap().unwrap();
    assert_eq!(fetched.title, "Fight Club");
    assert_eq!(fetched.status, "pending");
}

#[tokio::test]
async fn test_db_list_requests() {
    let db = test_db().await;
    let r1 = MediaRequest {
        id: Uuid::new_v4().to_string(), user_id: "u1".to_string(),
        media_type: "movie".to_string(), media_id: "tmdb-1".to_string(),
        title: "Movie 1".to_string(), status: "pending".to_string(),
        download_status: "none".to_string(), external_service_id: None,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    let r2 = MediaRequest {
        id: Uuid::new_v4().to_string(), user_id: "u2".to_string(),
        media_type: "tv".to_string(), media_id: "tmdb-2".to_string(),
        title: "Show 1".to_string(), status: "approved".to_string(),
        download_status: "none".to_string(), external_service_id: None,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    db.create_request(&r1).await.unwrap();
    db.create_request(&r2).await.unwrap();

    let all = db.list_requests().await.unwrap();
    assert_eq!(all.len(), 2);

    let pending = db.list_requests_by_status("pending").await.unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].title, "Movie 1");
}

#[tokio::test]
async fn test_db_settings() {
    let db = test_db().await;
    db.set_setting("TMDB_API_KEY", "test-key-123").await.unwrap();

    let val = db.get_setting("TMDB_API_KEY").await.unwrap();
    assert_eq!(val, Some("test-key-123".to_string()));

    let missing = db.get_setting("NONEXISTENT").await.unwrap();
    assert_eq!(missing, None);

    // Overwrite
    db.set_setting("TMDB_API_KEY", "new-key").await.unwrap();
    let val = db.get_setting("TMDB_API_KEY").await.unwrap();
    assert_eq!(val, Some("new-key".to_string()));
}

// ===== Config Tests =====

#[tokio::test]
async fn test_config_defaults() {
    // Without env vars, should use defaults
    let config = Config {
        database_url: "sqlite://:memory:".to_string(),
        jwt_secret: "test".to_string(),
        listen_addr: "0.0.0.0:5055".to_string(),
        port: 5055,
        jellyfin_url: None,
        jellyfin_api_key: None,
        log_level: "info".to_string(),
        demo_mode: false,
        cors_origin: "http://localhost:5055".to_string(),
    };
    assert_eq!(config.port, 5055);
    assert!(!config.demo_mode);
    assert_eq!(config.cors_origin, "http://localhost:5055");
}

// ===== SettingsService Tests =====

#[tokio::test]
async fn test_settings_get_set() {
    let db = test_db().await;
    SettingsService::set(&db, "test_key", "test_value").await.unwrap();
    let val = SettingsService::get(&db, "test_key").await.unwrap();
    assert_eq!(val, Some("test_value".to_string()));
}

#[tokio::test]
async fn test_settings_get_missing() {
    let db = test_db().await;
    let val = SettingsService::get(&db, "nonexistent").await.unwrap();
    assert_eq!(val, None);
}

#[tokio::test]
async fn test_settings_get_all() {
    let db = test_db().await;
    SettingsService::set(&db, "key1", "val1").await.unwrap();
    SettingsService::set(&db, "key2", "val2").await.unwrap();
    let all = SettingsService::get_all(&db).await.unwrap();
    assert_eq!(all.get("key1"), Some(&"val1".to_string()));
    assert_eq!(all.get("key2"), Some(&"val2".to_string()));
}

#[tokio::test]
async fn test_settings_get_api_key() {
    let db = test_db().await;
    // No key set — should return default
    let val = SettingsService::get_api_key(&db, "TMDB_API_KEY", "default-key").await.unwrap();
    assert_eq!(val, "default-key");

    // Set key — should return it
    SettingsService::set(&db, "TMDB_API_KEY", "real-key").await.unwrap();
    let val = SettingsService::get_api_key(&db, "TMDB_API_KEY", "default-key").await.unwrap();
    assert_eq!(val, "real-key");
}

// ===== IntegrationService Tests =====

#[tokio::test]
async fn test_integration_test_connection_fails_for_unreachable() {
    let integration = Integration {
        id: "test-int-1".to_string(),
        name: "Test Radarr".to_string(),
        integration_type: "radarr".to_string(),
        base_url: "http://127.0.0.1:1".to_string(), // port 1 = unreachable
        api_key: "test-key".to_string(),
        enabled: true,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    let result = IntegrationService::test(&integration).await;
    assert!(result.is_err(), "Connection to unreachable host should fail");
}

#[tokio::test]
async fn test_integration_test_connection_fails_for_bad_url() {
    let integration = Integration {
        id: "test-int-2".to_string(),
        name: "Test Bad URL".to_string(),
        integration_type: "sonarr".to_string(),
        base_url: "http://invalid-host-that-does-not-exist.local".to_string(),
        api_key: "test-key".to_string(),
        enabled: true,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    let result = IntegrationService::test(&integration).await;
    assert!(result.is_err(), "Connection to invalid host should fail");
}

// ===== urlencoding Tests =====

#[test]
fn test_urlencoding_basic() {
    assert_eq!(oakseerr::urlencoding("hello world"), "hello%20world");
    assert_eq!(oakseerr::urlencoding("test"), "test");
    assert_eq!(oakseerr::urlencoding("a/b"), "a%2Fb");
}

#[test]
fn test_urlencoding_special_chars() {
    assert_eq!(oakseerr::urlencoding("foo bar baz"), "foo%20bar%20baz");
    assert_eq!(oakseerr::urlencoding("a&b=c"), "a%26b%3Dc");
}

#[test]
fn test_urlencoding_unicode() {
    let encoded = oakseerr::urlencoding("über cool");
    assert!(encoded.contains("%C3%BC"), "ü should be percent-encoded: {}", encoded);
}

// ===== Config Tests (continued) =====

#[test]
fn test_config_port_parsing() {
    let config = Config {
        database_url: "sqlite://:memory:".to_string(),
        jwt_secret: "test".to_string(),
        listen_addr: "0.0.0.0:9090".to_string(),
        port: 9090,
        jellyfin_url: None,
        jellyfin_api_key: None,
        log_level: "info".to_string(),
        demo_mode: false,
        cors_origin: "http://localhost:9090".to_string(),
    };
    assert_eq!(config.port, 9090);
    assert_eq!(config.listen_addr, "0.0.0.0:9090");
}

#[test]
fn test_config_demo_mode() {
    let config = Config {
        database_url: "sqlite://:memory:".to_string(),
        jwt_secret: "test".to_string(),
        listen_addr: "0.0.0.0:5055".to_string(),
        port: 5055,
        jellyfin_url: None,
        jellyfin_api_key: None,
        log_level: "info".to_string(),
        demo_mode: true,
        cors_origin: "http://localhost:5055".to_string(),
    };
    assert!(config.demo_mode);
}
