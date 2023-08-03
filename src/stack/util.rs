use crate::config::Config;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
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

pub fn parameters_to_cmd_string(parameters: HashMap<String, String>) -> String {
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
) -> Result<HashMap<String, String>, Error> {
    let key_mask = extract_parameter_keys_from_template(template)?;
    let parameters = config
        .get_parameters(Some(&key_mask))
        .map_err(|e| match e {
            crate::config::Error::Expression(e) => e,
        })?;

    for key in key_mask.iter() {
        if !parameters.contains_key(key) {
            return Err(Error::MissingParameter {
                key: key.to_owned(),
            });
        }
    }

    Ok(parameters)
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
