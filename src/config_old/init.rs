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
            config_file_map: HashMap::new(),
            _secrets_file_map: HashMap::new(),
        }
    }

    pub fn from_path(config_path: impl AsRef<Path>, options: Options) -> Result<Config, Error> {
        let (config_path, config_filename) = verify_config_path(&config_path)?;
        let mut config_file_map: HashMap<PathBuf, Value> = HashMap::new();
        let mut secrets_file_map: HashMap<PathBuf, Value> = HashMap::new();

        if options.nested {
            let project_root = options.get_project_root()?;

            let mut parent_path = config_path;
            while parent_path.pop() {
                if let Ok((new_path, _)) = verify_config_path(parent_path.join(&config_filename)) {
                    Self::load_and_insert(&mut config_file_map, &mut secrets_file_map, &new_path)?;
                }

                if parent_path.ends_with(&project_root) {
                    break;
                }
            }
        } else {
            Self::load_and_insert(&mut config_file_map, &mut secrets_file_map, &config_path)?;
        }

        let config = Config {
            config_file_map,
            _secrets_file_map: secrets_file_map,
        };

        Ok(config)
    }

    fn load_and_insert(
        config_file_map: &mut HashMap<PathBuf, Value>,
        secrets_file_map: &mut HashMap<PathBuf, Value>,
        path: impl AsRef<Path>,
    ) -> Result<(), Error> {
        let (config_data, config_secrets_file_map) = Config::load_one(&path)?;
        config_file_map.insert(path.as_ref().to_owned(), config_data);
        if let Some(config_secrets_file_map) = config_secrets_file_map {
            secrets_file_map.extend(config_secrets_file_map.into_iter());
        }
        Ok(())
    }

    fn load_one(
        config_path: impl AsRef<Path>,
    ) -> Result<(Value, Option<HashMap<PathBuf, Value>>), Error> {
        let config_data = Self::parse_toml_file(&config_path)
            .map_err(|e| Error::load_error(&config_path, &format!("Invalid TOML: {}", e)))?;

        let secrets_file_map = config_data
            .get("secrets")
            .map(|secrets_entry| Self::load_secrets(secrets_entry, &config_path))
            .transpose()?;

        Ok((config_data, secrets_file_map))
    }

    fn load_secrets(
        secrets_entry: &Value,
        config_path: impl AsRef<Path>,
    ) -> Result<HashMap<PathBuf, Value>, Error> {
        match secrets_entry {
            Value::String(path) => Ok(HashMap::from([(
                PathBuf::from(path),
                Self::parse_toml_file(path)?,
            )])),
            Value::Array(_) => todo!("array of secret files not supported yet"),
            x => Err(Error::load_error(
                config_path,
                &format!("unexpected value for secrets: {:?}", x),
            )),
        }
    }

    fn parse_toml_file(path: impl AsRef<Path>) -> Result<Value, std::io::Error> {
        let data = std::fs::read_to_string(path.as_ref())?;
        let parsed_toml = data.parse::<Value>()?;
        Ok(parsed_toml)
    }
}

fn verify_config_path(config_path: impl AsRef<Path>) -> Result<(PathBuf, String), std::io::Error> {
    let config_path = config_path.as_ref().canonicalize()?;

    if !std::fs::metadata(&config_path)?.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "given config path does not point to a file",
        ));
    }

    let filename = config_path.file_name().ok_or(std::io::Error::new(
        std::io::ErrorKind::Other,
        "could not get file component from path",
    ))?;
    let filename = filename
        .to_os_string()
        .to_str()
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "file path component contains invalid characters",
        ))?
        .to_owned();

    Ok((config_path, filename))
}
