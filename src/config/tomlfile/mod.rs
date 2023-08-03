mod getters;
mod init;
mod parse;
pub(crate) mod traits;

use self::traits::TomlFile;
use toml::{value::Map, Value};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("File cannot reference itself as a secret file")]
    SelfReference,

    #[error("Secret files cannot contain references to other secret files")]
    SecretReference,

    #[error(transparent)]
    Parse(#[from] crate::config::tomlfile::parse::Error),

    #[error(transparent)]
    Verify(#[from] crate::config::verify_path::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug)]
pub(crate) struct ConfigFile {
    parameters: Map<String, Value>,
    envs: Map<String, Value>,
    secrets: Option<SecretsFile>,
}

#[derive(Debug)]
pub(crate) struct SecretsFile {
    parameters: Map<String, Value>,
    envs: Map<String, Value>,
}

impl TomlFile for ConfigFile {}
impl TomlFile for SecretsFile {}
