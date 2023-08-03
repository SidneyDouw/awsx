use anyhow::Result;
use std::io::Write;
use std::path::PathBuf;

pub fn encrypt(file: PathBuf, password: impl AsRef<str>) -> Result<()> {
    let encrypted = crate::secrets::core::encrypt(file, password)?;

    std::io::stdout().write_all(&encrypted)?;
    std::io::stdout().flush()?;

    Ok(())
}

pub fn decrypt(file: PathBuf, password: impl AsRef<str>) -> Result<()> {
    let decrypted = crate::secrets::core::decrypt(file, password)?;

    std::io::stdout().write_all(&decrypted)?;
    std::io::stdout().flush()?;

    Ok(())
}
