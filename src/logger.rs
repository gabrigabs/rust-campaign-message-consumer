use crate::error::{Error, Result};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn setup(log_level: &str) -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(log_level))
        .map_err(|e| Error::Logging(format!("Invalid log level: {}", e)))?;

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(env_filter)
        .try_init()
        .map_err(|e| Error::Logging(format!("Failed to initialize logger: {}", e)))?;

    Ok(())
}