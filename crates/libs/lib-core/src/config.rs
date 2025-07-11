use crate::{Error, Result};
use grapple_utils::envs::get;
use std::sync::OnceLock;
use tracing::warn;

static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

pub fn core_config() -> &'static CoreConfig {
    INSTANCE.get_or_init(|| {
        CoreConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHOLE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case, unused)]
#[derive(Debug)]
pub struct CoreConfig {
    // -- DB
    pub DB_URL: String,
}

impl CoreConfig {
    fn load_from_env() -> Result<Self> {
        Ok(Self {
            // -- DB
            DB_URL: get("SERVICE_DB_URL")?,
        })
    }

    pub fn init_from(cfg: Self) -> Result<()> {
        INSTANCE
            .set(cfg)
            .map_err(|_| Error::ConfigAlreadyInitialized)
    }
}
