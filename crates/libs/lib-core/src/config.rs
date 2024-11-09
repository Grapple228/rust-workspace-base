use lib_utils::envs::{get_env, Config};
use std::sync::OnceLock;

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

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

impl Config for CoreConfig {
    fn load_from_env() -> lib_utils::envs::Result<Self> {
        Ok(CoreConfig {
            // -- DB
            DB_URL: get_env("SERVICE_DB_URL")?,
        })
    }
}
