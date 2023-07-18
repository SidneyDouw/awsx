use super::{Config, OVERRIDE_FILEPATH};
use crate::{cmd::read_with_dir_and_env, config::Error, secrets};
use convert_case::{Case, Casing};
use core::panic;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use toml::{
    value::{Datetime, Map},
    Value,
};

// Getters
impl Config {
    pub fn get(&self, key: impl AsRef<str>) -> Option<&Value> {
        self.sorted_filepaths()
            .into_iter()
            .find_map(|filepath| self.get_from_file(&key, filepath))
    }

    pub fn get_with_filepath(&self, key: impl AsRef<str>) -> Option<(&Value, PathBuf)> {
        self.sorted_filepaths()
            .into_iter()
            .find_map(|filepath| self.get_from_file(&key, &filepath).map(|v| (v, filepath)))
    }

    pub fn get_mut(&mut self, key: impl AsRef<str>) -> Option<&mut Value> {
        for filepath in self.sorted_filepaths() {
            match self.get_from_file_mut(&key, &filepath) {
                Some(_) => return self.get_from_file_mut(key.as_ref(), &filepath),
                None => continue,
            }
        }
        None
    }

    pub fn get_mut_with_filepath(&mut self, key: impl AsRef<str>) -> Option<(&mut Value, PathBuf)> {
        for filepath in self.sorted_filepaths() {
            match self.get_from_file_mut(&key, &filepath) {
                Some(_) => {
                    return self
                        .get_from_file_mut(key.as_ref(), &filepath)
                        .map(|x| (x, filepath))
                }
                None => continue,
            }
        }
        None
    }

    /// returns a HashMap of all envs and exposed parameters of a config
    pub fn get_envs(&self) -> HashMap<String, String> {
        let mut envs: HashMap<String, (String, PathBuf)> = HashMap::new();

        envs.extend(
            self.get_merged_tables("env")
                .into_iter()
                .map(|(k, (v, p))| match v {
                    Value::String(s) => (k, (s, p)),
                    x => panic!("value for {k} is not a string, found {:?}", x),
                }),
        );

        envs.extend(
            self.get_merged_tables("parameters")
                .into_iter()
                .filter_map(|(k, (v, p))| match v {
                    Value::Table(t) => match (t.get("value"), t.get("expose")) {
                        (Some(Value::String(s)), Some(Value::Boolean(e))) if e == &true => Some((
                            format!("AWSX_PARAMETER_{}", k.to_case(Case::UpperSnake)),
                            (s.to_owned(), p),
                        )),
                        _ => None,
                    },
                    _ => None,
                }),
        );

        self.resolve_expression_values(&mut envs);

        envs.into_iter().map(|(k, (v, _))| (k, v)).collect()
    }

    pub fn get_secrets(&self, password: impl AsRef<str>) -> HashMap<String, String> {
        let mut secrets: HashMap<String, String> = HashMap::new();

        if let Some((val, mut config_path)) = self.get_with_filepath("secrets.files") {
            config_path.pop();
            let secrets_filepath = config_path.join(val.as_str().expect("filepath to secret"));

            let decrypted_toml =
                secrets::core::decrypt(secrets_filepath, password).expect("correct password");

            let parsed = String::from_utf8(decrypted_toml)
                .expect("valid utf-8")
                .parse::<Value>()
                .map_err(|e| Error::load_error(config_path, &format!("Invalid TOML: {}", e)))
                .expect("valid toml");

            match parsed {
                Value::Table(t) => {
                    t.into_iter().for_each(|(k, v)| match v {
                        Value::String(s) => {
                            secrets.insert(k, s);
                        }
                        x => panic!("value for {k} is not a string, found {:?}", x),
                    });
                }
                _ => panic!("value for secrets is not a table"),
            };

            secrets
        } else {
            secrets
        }
    }

    fn resolve_expression_values(&self, envs: &mut HashMap<String, (String, PathBuf)>) {
        let mut cleaned_envs = envs
            .clone()
            .into_iter()
            .map(|(k, (v, _))| (k, v))
            .collect::<HashMap<_, _>>();

        for (key, (val, config_path)) in envs.clone().into_iter() {
            if val.starts_with("{{") && val.ends_with("}}") {
                let exp = val[2..val.len() - 2].trim();
                let workdir = config_path.parent().expect("has parent");

                let val = read_with_dir_and_env(exp, workdir, &cleaned_envs, self)
                    .expect("valid expression");

                cleaned_envs.insert(key.clone(), val.clone());
                envs.entry(key).and_modify(|(v, _)| *v = val);
            }
        }
    }

    /// Gets a key from all config files, merges them according to priority into one HashMap and
    /// returns that
    pub(crate) fn get_merged_tables(
        &self,
        key: impl AsRef<str>,
    ) -> HashMap<String, (Value, PathBuf)> {
        self.sorted_filepaths()
            .into_iter()
            .rev()
            .map(|filepath| {
                self.get_from_file(&key, &filepath)
                    .map(|v| {
                        (
                            v.as_table()
                                .unwrap_or_else(|| panic!("key {} should be a table", key.as_ref()))
                                .to_owned(),
                            filepath,
                        )
                    })
                    .unwrap_or_default()
            })
            .map(|(table, filepath)| {
                table
                    .into_iter()
                    .map(|(key, val)| (key, (val, filepath.clone())))
                    .collect::<HashMap<_, _>>()
            })
            .reduce(|mut accum, item| {
                item.into_iter().for_each(|(k, v)| {
                    accum.insert(k, v);
                });
                accum
            })
            .unwrap_or_default()
    }

    pub(crate) fn get_from_file(
        &self,
        key: impl AsRef<str>,
        filepath: impl AsRef<Path>,
    ) -> Option<&Value> {
        let (key, sub_keys) = Self::split_key_once(key.as_ref());
        let filepath = if filepath.as_ref().to_string_lossy() == OVERRIDE_FILEPATH {
            filepath.as_ref().to_owned()
        } else {
            filepath.as_ref().canonicalize().ok()?
        };
        let val = self.file_map.get(&filepath)?.get(key)?;
        match (sub_keys.is_empty(), val) {
            (false, Value::Table(t)) => Self::get_from_table(t, sub_keys),
            (true, _) => Some(val),
            _ => None,
        }
    }

    pub(crate) fn get_from_table(
        table: &Map<String, Value>,
        key: impl AsRef<str>,
    ) -> Option<&Value> {
        let (key, sub_keys) = Self::split_key_once(key.as_ref());
        let val = table.get(key)?;
        match (sub_keys.is_empty(), val) {
            (false, Value::Table(t)) => Self::get_from_table(t, sub_keys),
            (true, _) => Some(val),
            _ => None,
        }
    }

    pub(crate) fn get_from_file_mut(
        &mut self,
        key: impl AsRef<str>,
        filepath: impl AsRef<Path>,
    ) -> Option<&mut Value> {
        let (key, sub_keys) = Self::split_key_once(key.as_ref());
        let filepath = if filepath.as_ref().to_string_lossy() == OVERRIDE_FILEPATH {
            filepath.as_ref().to_owned()
        } else {
            filepath.as_ref().canonicalize().ok()?
        };
        let val = self.file_map.get_mut(&filepath)?.get_mut(key)?;
        if sub_keys.is_empty() {
            Some(val)
        } else {
            match val {
                Value::Table(t) => Self::get_from_table_mut(t, sub_keys),
                _ => None,
            }
        }
    }

    pub(crate) fn get_from_table_mut(
        table: &mut Map<String, Value>,
        key: impl AsRef<str>,
    ) -> Option<&mut Value> {
        let (key, sub_keys) = Self::split_key_once(key.as_ref());
        let val = table.get_mut(key)?;
        if sub_keys.is_empty() {
            Some(val)
        } else {
            match val {
                Value::Table(t) => Self::get_from_table_mut(t, sub_keys),
                _ => None,
            }
        }
    }

    pub(crate) fn get_root_table_mut(
        &mut self,
        filepath: impl AsRef<Path>,
    ) -> &mut Map<String, Value> {
        self.file_map
            .entry(filepath.as_ref().to_owned())
            .or_insert_with(|| Value::Table(Map::new()))
            .as_table_mut()
            .expect("there should always be a 'table' at the root")
    }

    pub(crate) fn split_key_once(key: &str) -> (&str, &str) {
        match key.split_once('.') {
            Some((key, sub_keys)) => (key, sub_keys),
            None => (key, ""),
        }
    }

    /// returns a sorted Vec of the filepaths from lowest to highest priority, where the inner files have higher
    /// priority than the outer files
    pub(crate) fn sorted_filepaths(&self) -> Vec<PathBuf> {
        let mut filepaths = self
            .file_map
            .keys()
            .map(ToOwned::to_owned)
            .filter(|e| e.to_string_lossy() != OVERRIDE_FILEPATH)
            .collect::<Vec<_>>();

        filepaths.sort_by(|a, b| b.partial_cmp(a).unwrap());
        filepaths.insert(0, PathBuf::from(OVERRIDE_FILEPATH));

        filepaths
    }
}

/// Specific getters
#[cfg(not(tarpaulin_include))]
impl Config {
    pub fn get_string(&self, key: impl AsRef<str>) -> Option<&String> {
        match self.get(key) {
            Some(Value::String(v)) => Some(v),
            _ => None,
        }
    }

    pub fn get_int(&self, key: impl AsRef<str>) -> Option<&i64> {
        match self.get(key) {
            Some(Value::Integer(v)) => Some(v),
            _ => None,
        }
    }

    pub fn get_float(&self, key: impl AsRef<str>) -> Option<&f64> {
        match self.get(key) {
            Some(Value::Float(v)) => Some(v),
            _ => None,
        }
    }

    pub fn get_bool(&self, key: impl AsRef<str>) -> Option<&bool> {
        match self.get(key) {
            Some(Value::Boolean(v)) => Some(v),
            _ => None,
        }
    }

    pub fn get_datetime(&self, key: impl AsRef<str>) -> Option<&Datetime> {
        match self.get(key) {
            Some(Value::Datetime(v)) => Some(v),
            _ => None,
        }
    }

    pub fn get_array(&self, key: impl AsRef<str>) -> Option<&Vec<Value>> {
        match self.get(key) {
            Some(Value::Array(v)) => Some(v),
            _ => None,
        }
    }

    pub fn get_table(&self, key: impl AsRef<str>) -> Option<&Map<String, Value>> {
        match self.get(key) {
            Some(Value::Table(v)) => Some(v),
            _ => None,
        }
    }

    pub fn get_string_mut(&mut self, key: impl AsRef<str>) -> Option<&mut String> {
        match self.get_mut(key) {
            Some(Value::String(v)) => Some(v),
            _ => None,
        }
    }

    pub fn get_int_mut(&mut self, key: impl AsRef<str>) -> Option<&mut i64> {
        match self.get_mut(key) {
            Some(Value::Integer(v)) => Some(v),
            _ => None,
        }
    }

    pub fn get_float_mut(&mut self, key: impl AsRef<str>) -> Option<&mut f64> {
        match self.get_mut(key) {
            Some(Value::Float(v)) => Some(v),
            _ => None,
        }
    }

    pub fn get_bool_mut(&mut self, key: impl AsRef<str>) -> Option<&mut bool> {
        match self.get_mut(key) {
            Some(Value::Boolean(v)) => Some(v),
            _ => None,
        }
    }

    pub fn get_datetime_mut(&mut self, key: impl AsRef<str>) -> Option<&mut Datetime> {
        match self.get_mut(key) {
            Some(Value::Datetime(v)) => Some(v),
            _ => None,
        }
    }

    pub fn get_array_mut(&mut self, key: impl AsRef<str>) -> Option<&mut Vec<Value>> {
        match self.get_mut(key) {
            Some(Value::Array(v)) => Some(v),
            _ => None,
        }
    }

    pub fn get_table_mut(&mut self, key: impl AsRef<str>) -> Option<&mut Map<String, Value>> {
        match self.get_mut(key) {
            Some(Value::Table(v)) => Some(v),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_get_from_file() {
        let path = PathBuf::from("tests/fixtures/nested_configs/config.toml");
        let config = Config::from_path(&path, Default::default()).unwrap();

        let v = config.get_from_file("non_existent", &path);
        assert_eq!(v, None);

        let v = config.get_from_file("var_a", &path.join("invalid"));
        assert_eq!(v, None);

        let v = config
            .get_from_file("var_a", &path)
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(v, "abc");

        let v = config.get_from_file("env", &path).unwrap();
        assert!(matches!(v, toml::Value::Table(_)));

        let v = config
            .get_from_file("env.AWS_PROFILE", &path)
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(v, "default");

        let v = config.get_from_file("sub.a", &path).unwrap();
        assert!(matches!(v, toml::Value::Table(_)));

        let v = config.get_from_file("sub.a.", &path).unwrap();
        assert!(matches!(v, toml::Value::Table(_)));

        let v = config
            .get_from_file("sub.b.var_c", &path)
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(v, "rst");
    }
}
