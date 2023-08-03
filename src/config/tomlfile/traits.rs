use toml::{value::Map, Value};

use super::{Error, SecretsFile};
use crate::config::verify_path::VerifiedPath;

pub(crate) trait TomlFileInit {
    fn from_path(path: &VerifiedPath) -> Result<Self, Error>
    where
        Self: Sized;
}

pub(crate) trait TomlFileGetters {
    fn parameters(&self) -> &Map<String, Value>;
    fn envs(&self) -> &Map<String, Value>;
    fn secrets(&self) -> Option<&SecretsFile>;
}

pub(crate) trait TomlFile: TomlFileInit + TomlFileGetters {
    fn get_parameter(&self, key: &str) -> Option<&Value> {
        self.parameters().get(key)
    }

    fn get_env(&self, key: &str) -> Option<&Value> {
        self.envs().get(key)
    }

    fn get_secret_parameter(&self, key: &str) -> Option<&Value> {
        self.secrets()
            .and_then(|secrets| secrets.get_parameter(key))
    }

    fn get_secret_env(&self, key: &str) -> Option<&Value> {
        self.secrets().and_then(|secrets| secrets.get_env(key))
    }
}
