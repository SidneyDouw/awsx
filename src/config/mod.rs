pub use self::error::Error;
pub use self::options::Options;
use std::{collections::HashMap, path::PathBuf};
use toml::Value;

mod error;
mod options;

mod getters;
mod init;
mod setters;

const OVERRIDE_FILEPATH: &str = "overrides";

#[derive(Debug, Clone)]
pub struct Config {
    file_map: HashMap<PathBuf, Value>,
}
