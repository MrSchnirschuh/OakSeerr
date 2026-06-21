use crate::models::MediaRequest;
use crate::db::Database;
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
            external_service_id: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        db.create_request(&request).await?;
        Ok(request)
    }

    pub async fn approve(db: &Database, id: &str) -> anyhow::Result<MediaRequest> {
        let mut request = db.get_request(id).await?
            .ok_or_else(|| anyhow::anyhow!("Request not found"))?;
        request.status = "approved".to_string();
        request.updated_at = chrono::Utc::now().to_rfc3339();
        db.update_request(&request).await?;
        Ok(request)
    }

    pub async fn decline(db: &Database, id: &str) -> anyhow::Result<MediaRequest> {
        let mut request = db.get_request(id).await?
            .ok_or_else(|| anyhow::anyhow!("Request not found"))?;
        request.status = "declined".to_string();
        request.updated_at = chrono::Utc::now().to_rfc3339();
        db.update_request(&request).await?;
        Ok(request)
    }
}
