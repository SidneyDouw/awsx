use super::{tomlfile::traits::TomlFileGetters, Config, Error};
use crate::cmd::read_with_dir_and_env;
use convert_case::{Case, Casing};
use std::{
    cmp::Ordering,
    collections::HashMap,
    path::{Path, PathBuf},
};
use toml::Value;

impl Config {
    pub fn get_parameters(
        &self,
        key_mask: Option<&Vec<String>>,
    ) -> Result<HashMap<String, String>, Error> {
        let config_envs = Self::get_envs(self)?;

        let mut map = HashMap::new();
        for path in self.sorted_filepaths().into_iter() {
            let config = self.configs.get(path).expect("config exists");

            let mut parameters = config
                .parameters()
                .iter()
                .map(|(k, v)| (k.to_owned(), Self::extract_value(v)))
                .collect::<Vec<_>>();

            Self::sort_by_value_with_expression(&mut parameters);

            for (k, v) in parameters.into_iter() {
                if map.contains_key(&k) {
                    continue;
                }

                if let Some(key_mask) = key_mask {
                    if !key_mask.contains(&k) {
                        continue;
                    }
                }

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

            // TODO: potential sorting issue when parameter expressions depend on envs or the other
            // way around

            let mut parameters = config
                .parameters()
                .iter()
                .filter_map(|(k, v)| Self::extract_exposed_value(v).map(|v| (k.to_owned(), v)))
                .collect::<Vec<_>>();

            Self::sort_by_value_with_expression(&mut parameters);

            for (k, v) in parameters.into_iter() {
                if map.contains_key(&k) {
                    continue;
                }

                let k = format!("AWSX_PARAMETER_{}", k.to_case(Case::UpperSnake));
                map.insert(k, Self::resolve_expression(v, path, &map)?);
            }

            let mut envs = config
                .envs()
                .iter()
                .map(|(k, v)| (k.to_owned(), Self::extract_value(v)))
                .collect::<Vec<_>>();

            Self::sort_by_value_with_expression(&mut envs);

            for (k, v) in envs.into_iter() {
                if map.contains_key(&k) {
                    continue;
                }

                map.insert(k.to_owned(), Self::resolve_expression(v, path, &map)?);
            }
        }

        Ok(map)
    }

    /// Checks if a given value is surrounded by `{{ ... }}`
    /// If it is, it will run the inner expression via `bash -c ...` and return the value
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

    fn extract_exposed_value(v: &Value) -> Option<String> {
        match v {
            Value::String(_) => None,
            Value::Table(t) => match (t.get("value"), t.get("expose")) {
                (Some(Value::String(s)), Some(Value::Boolean(b))) if *b == true => {
                    Some(s.to_owned())
                }
                (Some(Value::String(_)), _) => None,
                (Some(_), _) => panic!("expected string value"),
                (None, _) => panic!("no \"value\" entry found"),
            },
            _ => panic!("expected \"string\" or \"table\" value"),
        }
    }

    fn extract_value(v: &Value) -> String {
        match v {
            Value::String(s) => s.to_owned(),
            Value::Table(t) => match t.get("value") {
                Some(Value::String(s)) => s.to_owned(),
                Some(_) => panic!("expected string value"),
                None => panic!("no \"value\" entry found"),
            },
            _ => panic!("expected \"string\" or \"table\" value"),
        }
    }

    fn sort_by_value_with_expression(vec: &mut Vec<(String, String)>) {
        vec.sort_by(
            |(_, v_a), (_, v_b)| match (v_a.contains('$'), v_b.contains('$')) {
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                _ => Ordering::Equal,
            },
        );
    }

    /// returns a [Vec] of config paths, sorted from innermost to outermost
    fn sorted_filepaths(&self) -> Vec<&PathBuf> {
        let mut filepaths = self.configs.keys().collect::<Vec<_>>();

        filepaths.sort_by(|a, b| b.partial_cmp(a).unwrap());
        filepaths
    }
}
