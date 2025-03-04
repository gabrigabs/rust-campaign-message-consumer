use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Environment variable not found: {0}")]
    MissingEnv(&'static str),
    
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Logging error: {0}")]
    Logging(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Processing error: {0}")]
    Processing(String),
    
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("MongoDB error: {0}")]
    MongoDB(#[from] mongodb::error::Error),

    #[error("PostgreSQL error: {0}")]
    Postgres(#[from] tokio_postgres::Error),
}