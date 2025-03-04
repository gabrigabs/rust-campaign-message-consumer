

mod error;
mod config;
mod logger;
mod db;
mod models;
mod repositories;
mod rabbitmq;

use crate::config::Config;
use crate::error::Result;
use crate::db::{mongodb::MongoDBConnection, postgres::PostgresConnection};
use crate::rabbitmq::consumer::RabbitMQConsumer;

use tracing::info;


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

    info!("Connecting to RabbitMQ at {}", config.rabbitmq.url);

    let consumer = RabbitMQConsumer::new(
        &config.rabbitmq.url,
        message_repository,
        campaign_repository,
    ).await?;

    info!("Starting to consume messages from queue: {}", config.rabbitmq.queue);
    consumer.consume_messages(&config.rabbitmq.queue).await?;



    Ok(())
}
