use crate::error::{Error, Result};
use mongodb::{Client, Database};

#[derive(Clone)]
pub struct MongoDBConnection {
    pub client: Client,
    pub database: Database,
}

impl MongoDBConnection {
    pub async fn new(uri: &str, db_name: &str) -> Result<Self> {
        let client = Client::with_uri_str(uri)
            .await
            .map_err(|e| Error::Connection(format!("Failed to connect to MongoDB: {}", e)))?;
            
        let database = client.database(db_name);
        
        Ok(Self {
            client,
            database,
        })
    }

}