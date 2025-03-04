use std::sync::Arc;
use crate::config::PostgresConfig;
use crate::error::{Error, Result};
use tokio_postgres::{Client, NoTls};

#[derive(Clone)]
pub struct PostgresConnection {
    pub client: Arc<Client>,
}

impl PostgresConnection {
    pub async fn new(config: &PostgresConfig) -> Result<Self> {
        let connection_string = format!(
            "host={} port={} dbname={} user={} password={}",
            config.host, config.port, config.database, config.user, config.password
        );
        

        let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
            .await
            .map_err(|e| Error::Connection(format!("Failed to connect to PostgreSQL: {}", e)))?;
            
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("PostgreSQL connection error: {}", e);
            }
        });
        
        Ok(Self { client: Arc::new(client)})
    }
}