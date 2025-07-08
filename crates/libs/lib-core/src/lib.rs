mod config;
mod error;

pub mod ctx;
pub mod model;

// #[cfg(test)]
pub mod _dev_utls;

pub use config::core_config;
pub use error::{Error, Result};

#[derive(Clone)]
pub(crate) struct Db;
