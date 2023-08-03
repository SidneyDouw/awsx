use crate::config::tomlfile::traits::TomlFileInit;
use crate::config::{verify_path::VerifiedPath, Config, ConfigFile};
use std::{
    collections::HashMap,
    fs::metadata,
    path::{Path, PathBuf},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error while verifying path:\n  {0}")]
    VerifyPath(#[from] crate::config::verify_path::Error),

    #[error("No .git file / folder found while looking for root folder")]
    RootFolder,

    #[error("Error while parsing TOML file at:\n  \"{path}\"\n{source}")]
    Toml {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("Error while loading file:\n  {0}")]
    ConfigFile(#[from] crate::config::tomlfile::Error),
}

impl Error {
    pub fn from_toml(path: &Path, err: toml::de::Error) -> Error {
        Error::Toml {
            path: path.to_owned(),
            source: err,
        }
    }
}

impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = VerifiedPath::try_from(path.as_ref())?;
        let filename = path.get_file_name();
        let root_folder = Self::find_root_folder(&path)?;

        let mut configs: HashMap<PathBuf, ConfigFile> = HashMap::new();
        let mut path = path.0;

        while path.pop() {
            if let Ok(new_path) = VerifiedPath::try_from(path.join(&filename)) {
                let config = ConfigFile::from_path(&new_path)?;
                configs.insert(new_path.0, config);
            }

            if path.ends_with(&root_folder) {
                break;
            }
        }

        Ok(Self { configs })
    }
}

impl Config {
    // TODO: find a better way of finding a root folder than the presence of a git file / folder

    /// Starting from the given path, iterate through each parent folder until we find a `.git`
    /// directory, aka our `root_folder`.
    ///
    /// Any config file within a parent folder up to and including the root folder will be loaded
    /// into the [Config].
    fn find_root_folder(path: &VerifiedPath) -> Result<PathBuf, Error> {
        let mut path = path.0.to_owned();
        while path.pop() {
            if metadata(path.join(".git")).is_ok() {
                break;
            };

            if path.parent().is_none() {
                return Err(Error::RootFolder);
            }
        }

        Ok(path)
    }
}
