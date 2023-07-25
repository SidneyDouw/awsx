use super::Config;
use std::path::{Path, PathBuf};
use toml::Value;

impl Config {
    /// Goes through all [ConfigFile]s until it finds the first [Value].
    /// The files will be sorted by filepath, innermost to outermost.
    pub fn get(&self, key: &str) -> Option<Value> {
        self.sorted_filepaths()
            .into_iter()
            .find_map(|path| self.get_from_config(key, path))
    }

    /// Goes through all [ConfigFile]s, gets the values of the specified key and merges them,
    /// preferring the vales in the innermost files over the outermost files.
    ///
    /// Key needs to refer to a [Value::Table] or [Value::Array]
    pub fn get_merged(_key: &str) {
        todo!()
    }
}

impl Config {
    fn get_from_config(&self, _key: &str, _path: &Path) -> Option<Value> {
        todo!()
    }

    /// returns a [Vec] of config paths, sorted from innermost to outermost
    pub(crate) fn sorted_filepaths(&self) -> Vec<&PathBuf> {
        let mut filepaths = self
            .configs
            .keys()
            // .map(ToOwned::to_owned)
            // .filter(|e| e.to_string_lossy() != OVERRIDE_FILEPATH)
            .collect::<Vec<_>>();

        filepaths.sort_by(|a, b| b.partial_cmp(a).unwrap());
        // filepaths.insert(0, PathBuf::from(OVERRIDE_FILEPATH));

        filepaths
    }
}
