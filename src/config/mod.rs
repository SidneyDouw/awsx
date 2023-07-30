mod getters;
mod init;
#[cfg(test)]
mod tests;
mod tomlfile;
mod verify_path;

use self::tomlfile::ConfigFile;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error while trying to resolve an expression value: {0}")]
    Expression(#[from] crate::cmd::Error),
}

#[derive(Debug)]
pub struct Config {
    configs: HashMap<PathBuf, ConfigFile>,
}
