use crate::db::Database;
use crate::models::User;
use argon2::Argon2;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,       // user id
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

pub struct AuthService {
    jwt_secret: String,
}

impl AuthService {
    pub fn new(jwt_secret: &str) -> Self {
        Self {
            jwt_secret: jwt_secret.to_string(),
        }
    }

    pub fn create_token(&self, user: &User) -> anyhow::Result<String> {
        let now = chrono::Utc::now();
        let claims = Claims {
            sub: user.id.clone(),
            username: user.username.clone(),
            iat: now.timestamp() as usize,
            exp: (now + chrono::Duration::days(30)).timestamp() as usize,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;
        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> anyhow::Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }

    pub async fn jellyfin_auth(
        &self,
        db: &Database,
        jellyfin_url: &str,
        api_key: &str,
        username: &str,
    ) -> anyhow::Result<(User, String)> {
        // Jellyfin API: GET /Users?api_key=... to find user
        let client = reqwest::Client::new();
        let users_url = format!("{}/Users?api_key={}", jellyfin_url.trim_end_matches('/'), api_key);
        let resp = client.get(&users_url).send().await?;
        let users: Vec<serde_json::Value> = resp.json().await?;

        let jellyfin_user = users.iter().find(|u| {
            u.get("Name").and_then(|n| n.as_str()) == Some(username)
        }).ok_or_else(|| anyhow::anyhow!("User not found in Jellyfin"))?;

        let jellyfin_user_id = jellyfin_user.get("Id").and_then(|i| i.as_str()).unwrap_or("");
        let display_name = jellyfin_user.get("Name").and_then(|n| n.as_str()).unwrap_or(username);

        // Find or create local user
        let existing = db.get_user_by_jellyfin_id(jellyfin_user_id).await?;
        let user = if let Some(u) = existing {
            u
        } else {
            let new_user = User {
                id: Uuid::new_v4().to_string(),
                username: username.to_string(),
                display_name: display_name.to_string(),
                email: None,
                avatar_url: Some(format!("{}/Users/{}/Images/Primary", jellyfin_url.trim_end_matches('/'), jellyfin_user_id)),
                jellyfin_user_id: Some(jellyfin_user_id.to_string()),
                permissions: 100, // Admin by default for first user
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            };
            db.create_user(&new_user).await?;
            new_user
        };

        let token = self.create_token(&user)?;
        Ok((user, token))
    }

    pub async fn create_demo_user(&self, db: &Database) -> anyhow::Result<(User, String)> {
        let existing = db.get_user("demo-user").await?;
        let user = if let Some(u) = existing {
            u
        } else {
            let new_user = User {
                id: "demo-user".to_string(),
                username: "demo".to_string(),
                display_name: "Demo User".to_string(),
                email: None,
                avatar_url: None,
                jellyfin_user_id: None,
                permissions: 100,
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            };
            db.create_user(&new_user).await?;
            new_user
        };

        let token = self.create_token(&user)?;
        Ok((user, token))
    }
}
