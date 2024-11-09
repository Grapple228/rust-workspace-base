use lib_utils::envs::{get_env_parse, Config};
use std::sync::OnceLock;

pub fn fs_config() -> &'static FsConfig {
    static INSTANCE: OnceLock<FsConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        FsConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct FsConfig {
    pub FS_VAR: bool,
}

impl Config for FsConfig {
    fn load_from_env() -> lib_utils::envs::Result<Self> {
        Ok(FsConfig {
            FS_VAR: get_env_parse("FS_VAR")?,
        })
    }
}
