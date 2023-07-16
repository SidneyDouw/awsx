use super::{Config, Error, Options};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use toml::Value;

impl Default for Config {
    fn default() -> Config {
        Config::new()
    }
}

/// Initialize
impl Config {
    pub fn new() -> Config {
        Config {
            file_map: HashMap::new(),
        }
    }

    pub fn from_path(config_path: impl AsRef<Path>, options: Options) -> Result<Config, Error> {
        let config_path = config_path
            .as_ref()
            .canonicalize()
            .map_err(|_| Error::load_error(config_path, "could not make an absolute path"))?;

        let filename = config_path.file_name().ok_or(Error::load_error(
            &config_path,
            "could not get filename from path",
        ))?;

        let mut files: HashMap<PathBuf, Value> = HashMap::new();

        if options.nested {
            let project_root = options.get_project_root().map_err(|e| {
                Error::load_error(&config_path, &format!("could not find project root: {}", e))
            })?;

            let mut config_path = config_path.clone();
            while config_path.pop() {
                let new_path = &config_path.join(filename);
                if let Ok(m) = std::fs::metadata(new_path) {
                    if m.is_file() {
                        let data = Config::load_one(new_path)?;
                        files.insert(new_path.clone(), data);
                    }
                }

                if config_path.ends_with(&project_root) {
                    break;
                }
            }
        } else {
            let data = Config::load_one(&config_path)?;
            files.insert(config_path, data);
        }

        Ok(Config { file_map: files })
    }

    fn load_one(config_path: impl AsRef<Path>) -> Result<Value, Error> {
        match std::fs::read_to_string(config_path.as_ref()) {
            Ok(bytes) => bytes
                .parse::<Value>()
                .map_err(|e| Error::load_error(config_path, &format!("Invalid TOML: {}", e))),
            Err(e) => Err(Error::load_error(config_path, &e.to_string())),
        }
    }

    // TODO: `verify` config after loading to make sure certain expectations are met
    #[allow(dead_code)]
    fn verify(&self) -> bool {
        todo!()
    }
}
