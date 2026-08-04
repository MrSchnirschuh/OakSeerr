use crate::db::Database;
use crate::models::MediaRequest;
use uuid::Uuid;

pub struct RequestService;

impl RequestService {
    pub async fn create(
        db: &Database,
        user_id: &str,
        media_type: &str,
        media_id: &str,
        title: &str,
    ) -> anyhow::Result<MediaRequest> {
        let request = MediaRequest {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            media_type: media_type.to_string(),
            media_id: media_id.to_string(),
            title: title.to_string(),
            status: "pending".to_string(),
            download_status: "none".to_string(),
            external_service_id: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        db.create_request(&request).await?;
        Ok(request)
    }

    pub async fn approve(db: &Database, id: &str) -> anyhow::Result<MediaRequest> {
        let mut request = db
            .get_request(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Request not found"))?;
        request.status = "approved".to_string();
        request.updated_at = chrono::Utc::now().to_rfc3339();
        db.update_request(&request).await?;
        Ok(request)
    }

    pub async fn decline(db: &Database, id: &str) -> anyhow::Result<MediaRequest> {
        let mut request = db
            .get_request(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Request not found"))?;
        request.status = "declined".to_string();
        request.updated_at = chrono::Utc::now().to_rfc3339();
        db.update_request(&request).await?;
        Ok(request)
    }

    /// Get all requests with their download status
    pub async fn get_all_with_status(db: &Database) -> anyhow::Result<Vec<MediaRequest>> {
        db.list_requests().await
    }

    /// Update download status for a request
    pub async fn update_download_status(
        db: &Database,
        id: &str,
        download_status: &str,
    ) -> anyhow::Result<MediaRequest> {
        let mut request = db
            .get_request(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Request not found"))?;
        request.download_status = download_status.to_string();
        request.updated_at = chrono::Utc::now().to_rfc3339();
        db.update_request(&request).await?;
        Ok(request)
    }
}
