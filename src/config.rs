use crate::error::Result;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub mongodb: MongoDBConfig,
    pub rabbitmq: RabbitMQConfig,
    pub postgres: PostgresConfig,
    pub app: AppConfig,
}

#[derive(Debug, Clone)]
pub struct MongoDBConfig {
    pub uri: String,
    pub db_name: String,
}

#[derive(Debug, Clone)]
pub struct RabbitMQConfig {
    pub url: String,
    pub queue: String,
}

#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();
        
        Ok(Config {
            mongodb: MongoDBConfig {
                uri: env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017/campaigns".to_string()),
                db_name: env::var("MONGODB_DB_NAME").unwrap_or_else(|_| "campaigns".to_string()),
            },
            rabbitmq: RabbitMQConfig {
                url: env::var("RABBITMQ_URL").unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string()),
                queue: env::var("RABBITMQ_QUEUE").unwrap_or_else(|_| "campaign_messages".to_string()),
            },
            postgres: PostgresConfig {
                host: env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string()),
                port: env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string()).parse().unwrap_or(5432),
                database: env::var("POSTGRES_DB").unwrap_or_else(|_| "campaigns".to_string()),
                user: env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string()),
                password: env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".to_string()),
            },
            app: AppConfig {
                log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            },
        })
    }
}