

mod error;
mod config;
mod logger;
mod db;
mod models;
mod repositories;

use crate::config::Config;
use crate::error::Result;
use crate::db::{mongodb::MongoDBConnection, postgres::PostgresConnection};
use crate::models::message::Message;

use models::message;
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

    let message_repository = repositories::message_repository::MessageRepository::new(mongo_db.database);
    let campaign_repository = repositories::campaign_repository::CampaignRepository::new(postgres_db.client);



    Ok(())
}
