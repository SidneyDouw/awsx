use super::{tomlfile::traits::TomlFileGetters, Config, Error};
use crate::cmd::read_with_dir_and_env;
use convert_case::{Case, Casing};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use toml::Value;

impl Config {
    pub fn get_parameters(
        &self,
        key_mask: Option<&Vec<String>>,
    ) -> Result<HashMap<String, String>, Error> {
        let mut map = HashMap::new();
        let config_envs = Self::get_envs(self)?;

        for path in self.sorted_filepaths().into_iter() {
            let config = self.configs.get(path).expect("config exists");

            for (k, v) in config.parameters().iter() {
                if map.contains_key(k) {
                    continue;
                }

                if let Some(key_mask) = key_mask {
                    if !key_mask.contains(k) {
                        continue;
                    }
                }

                let v = match v {
                    Value::String(s) => s.to_owned(),
                    Value::Table(t) => match t.get("value").and_then(|v| v.as_str()) {
                        Some(v) => v.to_owned(),
                        None => todo!(),
                    },
                    _ => todo!(),
                };

                map.insert(
                    k.to_owned(),
                    Self::resolve_expression(v, path, &config_envs)?,
                );
            }
        }

        Ok(map)
    }

    pub fn get_envs(&self) -> Result<HashMap<String, String>, Error> {
        let mut map = HashMap::new();
        for path in self.sorted_filepaths().into_iter() {
            let config = self.configs.get(path).expect("config exists");

            for (k, v) in config.parameters().iter() {
                if map.contains_key(k) {
                    continue;
                }

                if let Some((k, v)) = v
                    .as_table()
                    .and_then(|t| match (t.get("value"), t.get("expose")) {
                        (Some(Value::String(s)), Some(Value::Boolean(b))) if *b => {
                            let k = format!("AWSX_PARAMETER_{}", k.to_case(Case::UpperSnake));
                            Some((k, s.to_owned()))
                        }
                        _ => None,
                    })
                    .map(|(k, v)| (k, Self::resolve_expression(v, path, &map)))
                {
                    map.insert(k, v?);
                }
            }

            for (k, v) in config.envs().iter() {
                if map.contains_key(k) {
                    continue;
                }

                // TODO: Table values not yet supported
                if let Some((k, v)) = v
                    .as_str()
                    .map(|s| (k.to_owned(), s.to_owned()))
                    .map(|(k, v)| (k, Self::resolve_expression(v, path, &map)))
                {
                    map.insert(k, v?);
                }
            }
        }

        Ok(map)
    }

    fn resolve_expression(
        v: String,
        path: &Path,
        envs: &HashMap<String, String>,
    ) -> Result<String, Error> {
        if v.starts_with("{{") && v.ends_with("}}") {
            let exp = v[2..v.len() - 2].trim();
            let workdir = path.parent().expect("has parent");

            read_with_dir_and_env(exp, workdir, envs).map_err(Error::Expression)
        } else {
            Ok(v)
        }
    }

    /// returns a [Vec] of config paths, sorted from innermost to outermost
    fn sorted_filepaths(&self) -> Vec<&PathBuf> {
        let mut filepaths = self.configs.keys().collect::<Vec<_>>();

        filepaths.sort_by(|a, b| b.partial_cmp(a).unwrap());
        filepaths
    }
}
