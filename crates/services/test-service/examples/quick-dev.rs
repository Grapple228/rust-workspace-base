#![allow(unused)] // For beginning only.

use anyhow::Result;
use lib_fs::fs_config;

#[tokio::main]
async fn main() -> Result<()> {
    let config = fs_config();

    lib_fs::execute(config.FS_VAR)?;

    Ok(())
}
