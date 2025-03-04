

mod error;
mod config;
mod logger;

use crate::config::Config;
use crate::error::Result;
use tracing::{info, error};

fn main() -> Result<()>{

    let config = Config::from_env()?;
    
    logger::setup(&config.app.log_level)?;
    info!("Starting campaign message consumer");
    Ok(())
}
