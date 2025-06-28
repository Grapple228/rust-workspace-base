// region:    --- Modules

use tracing::info;
use tracing_subscriber::EnvFilter;

// -- Modules
mod error;

// -- Flatten
pub use error::{Error, Result};

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
    init()?;

    // App logic

    Ok(())
}

fn init() -> Result<()> {
    // LOGGING INITIALIZATION
    tracing_subscriber::fmt()
        .without_time() // For early development
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Initializing");

    Ok(())
}
