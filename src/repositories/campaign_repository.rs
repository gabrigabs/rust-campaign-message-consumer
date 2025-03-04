use std::sync::Arc;

use crate::error::Result;
use tokio_postgres::Client;
use tracing::{info, error};

pub struct CampaignRepository {
    client: Arc<Client>,
}

impl CampaignRepository {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn update_campaign_status(&self, campaign_id: &str, status: &str) -> Result<u64> {
        let query = "UPDATE \"Campaign\" SET status = $1, updated_at = NOW() WHERE id = $2";
        
        info!("Updating campaign {} with status: {}", campaign_id, status);
        let result = self.client.execute(query, &[&status, &campaign_id]).await?;
        
        if result == 0 {
            error!("No campaign found with ID: {}", campaign_id);
        } else {
            info!("Updated campaign {} status: {} rows affected", campaign_id, result);
        }
        
        Ok(result)
    }
}