use super::{Error, Result};

pub fn execute(is_error: bool) -> Result<()> {
    if is_error {
        Err(Error::FileNotFound)?;
    }

    Ok(())
}
