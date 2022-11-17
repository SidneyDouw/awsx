use crate::{cmd::read_with_dir, config::Config};
use std::path::{Path, PathBuf};
use toml::Value;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid yaml syntax")]
    InvalidYaml(#[from] yaml_rust::ScanError),

    #[error("Missing parameter in config: {:?}", key)]
    MissingParameter { key: String },

    #[error("Problem with path: {:?}", path)]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error(transparent)]
    Cmd(#[from] crate::cmd::Error),
}

pub fn parameters_to_string(parameters: Vec<(String, Value)>) -> String {
    [String::from("--parameters")]
        .into_iter()
        .chain(
            parameters
                .into_iter()
                .map(|(k, v)| format!("ParameterKey={},ParameterValue={}", k, v)),
        )
        .collect::<Vec<String>>()
        .join(" ")
}

/// Only gets the necessary parameters that it finds in the template file
pub fn get_parameter_values_from_config(
    template: impl AsRef<Path>,
    config: &Config,
) -> Result<Vec<(String, Value)>, Error> {
    extract_parameter_keys_from_template(template)?
        .into_iter()
        .map(|key| {
            config
                .get_with_filepath(&format!("parameters.{}", key))
                .map(|(val, filepath)| (key.clone(), val.clone(), filepath))
                .ok_or(Error::MissingParameter { key })
        })
        // .map(|r| r.map(|(key, val, _filepath)| (key, val)))
        .flat_map(|r| {
            r.map({
                |(key, val, filepath)| {
                    if let Some(s) = val.as_str() {
                        if s.starts_with("{{") && s.ends_with("}}") {
                            let exp = s[2..s.len() - 2].trim();
                            return read_with_dir(
                                exp,
                                config,
                                filepath.parent().expect("has a parent"),
                            )
                            .map(|val| (key, Value::String(val)))
                            .map_err(Error::Cmd);
                        }
                    }

                    Ok((key, val))
                }
            })
        })
        .collect::<Result<Vec<_>, Error>>()
}

pub fn extract_parameter_keys_from_template(
    template: impl AsRef<Path>,
) -> Result<Vec<String>, Error> {
    let yaml_str = std::fs::read_to_string(&template).map_err(|e| Error::Io {
        path: template.as_ref().into(),
        source: e,
    })?;
    let mut roots = YamlLoader::load_from_str(&yaml_str)?;

    assert_eq!(
        roots.len(),
        1,
        "multiple entrypoints found in template file"
    );

    Ok(roots
        .remove(0)
        .into_hash()
        .expect("tree structure at root")
        .remove(&Yaml::String("Parameters".to_string()))
        .map_or(Vec::new(), |x| {
            x.into_hash()
                .expect("Expected \"Parameters\" entry to have sub entries")
                .into_iter()
                .map(|(k, _v)| match k {
                    Yaml::String(k) => k,
                    _ => unreachable!("expected ”String” as keys"),
                })
                .collect()
        }))
}
