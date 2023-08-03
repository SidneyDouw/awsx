use super::{traits::TomlFileGetters, ConfigFile, SecretsFile};
use toml::{value::Map, Value};

impl TomlFileGetters for ConfigFile {
    fn parameters(&self) -> &Map<String, Value> {
        &self.parameters
    }

    fn envs(&self) -> &Map<String, Value> {
        &self.envs
    }

    fn secrets(&self) -> Option<&SecretsFile> {
        self.secrets.as_ref()
    }
}

impl TomlFileGetters for SecretsFile {
    fn parameters(&self) -> &Map<String, Value> {
        &self.parameters
    }

    fn envs(&self) -> &Map<String, Value> {
        &self.envs
    }

    fn secrets(&self) -> Option<&SecretsFile> {
        None
    }
}
