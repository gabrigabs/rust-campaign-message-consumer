

mod error;
mod config;
mod logger;
mod db;

use crate::config::Config;
use crate::error::Result;
use crate::db::{mongodb::MongoDBConnection, postgres::PostgresConnection};
use tracing::{info, error};


#[tokio::main]
async fn main() -> Result<()>{

    let config = Config::from_env()?;
    
    logger::setup(&config.app.log_level)?;
    info!("Starting campaign message consumer");


    info!("Connecting to MongoDB...");
    let mongo_db = MongoDBConnection::new(&config.mongodb.uri, &config.mongodb.db_name).await?;
    info!("MongoDB connected");

    info!("Connecting to PostgreSQL...");
    let postgres_db = PostgresConnection::new(&config.postgres).await?;
    info!("PostgreSQL connected");

    Ok(())
}
