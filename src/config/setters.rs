use super::Config;
use crate::config::OVERRIDE_FILEPATH;
use std::path::PathBuf;
use toml::{value::Map, Value};

/// Setters
impl Config {
    pub fn set(&mut self, key: impl AsRef<str>, value: Value) {
        let keys = key.as_ref().split('.').collect::<Vec<_>>();
        let last_key = keys.last().expect("at least one entry").to_string();

        if keys.len() == 1 {
            self.get_root_table_mut(PathBuf::from(OVERRIDE_FILEPATH))
                .insert(last_key, value);
        } else {
            match self.find_first_existing_table(&key) {
                (Some(table), prefix) => {
                    let subtract = prefix.split('.').count();
                    let sub_key = Vec::from(&keys[subtract..]).join(".");
                    Self::nested_insert(table, sub_key, value)
                }
                (None, _) => {
                    Self::nested_insert(
                        self.get_root_table_mut(PathBuf::from(OVERRIDE_FILEPATH)),
                        key,
                        value,
                    );
                }
            }
        }
    }

    fn nested_insert(table: &mut Map<String, Value>, key: impl AsRef<str>, value: Value) {
        let mut keys = key.as_ref().split('.').collect::<Vec<_>>();
        let first_key = keys.first().expect("at least one entry").to_string();

        keys.reverse();

        let mut value = value;
        for key in &keys[..keys.len() - 1] {
            let mut map = Map::new();
            map.insert(key.to_string(), value);
            value = Value::Table(map);
        }

        table.insert(first_key, value);
    }

    fn find_first_existing_table(
        &mut self,
        key: impl AsRef<str>,
    ) -> (Option<&mut Map<String, Value>>, String) {
        let keys = key.as_ref().split('.').collect::<Vec<_>>();

        let prefix = if keys.len() > 1 {
            keys.clone()
                .into_iter()
                .take(keys.len() - 1)
                .collect::<Vec<_>>()
                .join(".")
        } else {
            keys.first().unwrap().to_string()
        };

        match self.get_from_file_mut(&prefix, PathBuf::from(OVERRIDE_FILEPATH)) {
            Some(Value::Table(_)) => (
                Some(
                    self.get_from_file_mut(&prefix, PathBuf::from(OVERRIDE_FILEPATH))
                        .unwrap()
                        .as_table_mut()
                        .unwrap(),
                ),
                prefix,
            ),
            Some(v) => {
                todo!(
                    "handle invalid nested key\nfound a value but not a table, continuing...\n{:?}",
                    v
                );
            }
            _ if keys.len() > 2 => self.find_first_existing_table(prefix),
            _ => (None, "".to_string()),
        }
    }
}

/// Specific setters
#[cfg(not(tarpaulin_include))]
impl Config {
    pub fn set_string(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) {
        self.set(key, Value::String(value.as_ref().to_owned()));
    }

    pub fn set_int(&mut self, key: impl AsRef<str>, value: impl Into<i64>) {
        self.set(key, Value::Integer(value.into()));
    }

    pub fn set_float(&mut self, key: impl AsRef<str>, value: impl Into<f64>) {
        self.set(key, Value::Float(value.into()));
    }

    pub fn set_bool(&mut self, key: impl AsRef<str>, value: bool) {
        self.set(key, Value::Boolean(value));
    }

    // pub fn set_date(&mut self, key: impl AsRef<str>, value: impl Into<DateTime>) {
    //     self.set(key, Value::Datetime(value.into()));
    // }

    pub fn set_array(&mut self, key: impl AsRef<str>, value: Vec<Value>) {
        self.set(key, Value::Array(value));
    }

    pub fn set_table(&mut self, key: impl AsRef<str>, value: Map<String, Value>) {
        self.set(key, Value::Table(value));
    }
}
