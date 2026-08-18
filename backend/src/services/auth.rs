use crate::db::Database;
use crate::models::User;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
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

    /// Jellyfin SSO: authenticate with username/password against Jellyfin API
    pub async fn jellyfin_auth(
        &self,
        db: &Database,
        jellyfin_url: &str,
        username: &str,
        password: &str,
    ) -> anyhow::Result<(User, String)> {
        let client = reqwest::Client::new();
        let auth_url = format!(
            "{}/Users/AuthenticateByName",
            jellyfin_url.trim_end_matches('/')
        );

        let auth_payload = serde_json::json!({
            "Username": username,
            "Pw": password,
        });

        let resp = client.post(&auth_url)
            .header("Content-Type", "application/json")
            .header("X-Emby-Authorization", "MediaBrowser Client=\"OakSeerr\", Device=\"Server\", DeviceId=\"OakSeerr\", Version=\"0.1.0\"")
            .json(&auth_payload)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(anyhow::anyhow!(
                "Jellyfin authentication failed: HTTP {}",
                resp.status()
            ));
        }

        let auth_data: serde_json::Value = resp.json().await?;
        let jellyfin_user_id = auth_data
            .get("User")
            .and_then(|u| u.get("Id"))
            .and_then(|i| i.as_str())
            .ok_or_else(|| anyhow::anyhow!("Could not parse Jellyfin user ID"))?;

        let display_name = auth_data
            .get("User")
            .and_then(|u| u.get("Name"))
            .and_then(|n| n.as_str())
            .unwrap_or(username);

        // Find or create local user
        let existing = db.get_user_by_jellyfin_id(jellyfin_user_id).await?;
        let user = if let Some(u) = existing {
            u
        } else {
            let is_first_user = db.count_users().await? == 0;
            let permissions = if is_first_user { 100 } else { 0 };
            let new_user = User {
                id: Uuid::new_v4().to_string(),
                username: username.to_string(),
                display_name: display_name.to_string(),
                email: None,
                avatar_url: Some(format!(
                    "{}/Users/{}/Images/Primary",
                    jellyfin_url.trim_end_matches('/'),
                    jellyfin_user_id
                )),
                jellyfin_user_id: Some(jellyfin_user_id.to_string()),
                permissions,
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


#[cfg(test)]
mod tests {
    use super::*;

    fn sample_user() -> User {
        User {
            id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            username: "test".to_string(),
            display_name: "Test User".to_string(),
            email: None,
            avatar_url: None,
            jellyfin_user_id: None,
            permissions: 0,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    #[test]
    fn test_create_and_verify_token() {
        let service = AuthService::new("test-secret");
        let user = sample_user();
        let token = service.create_token(&user).unwrap();
        let claims = service.verify_token(&token).unwrap();
        assert_eq!(claims.sub, user.id);
        assert_eq!(claims.username, user.username);
    }

    #[test]
    fn test_verify_wrong_secret_fails() {
        let signer = AuthService::new("secret-a");
        let verifier = AuthService::new("secret-b");
        let user = sample_user();
        let token = signer.create_token(&user).unwrap();
        assert!(verifier.verify_token(&token).is_err());
    }
}
