mod getters;
mod init;
#[cfg(test)]
mod tests;
mod tomlfile;
mod verify_path;

use self::tomlfile::ConfigFile;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct Config {
    configs: HashMap<PathBuf, ConfigFile>,
}
