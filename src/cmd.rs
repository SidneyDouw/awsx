use crate::config::Config;
use duct::cmd;
use std::{collections::HashMap, path::Path};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not find environment variable: {:?} in config files", key)]
    EnvVarMissing { key: String },

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

pub fn run(cmd: &str, config: &Config) -> Result<(), Error> {
    expression(cmd, config)?
        .run()
        .map(|_| ())
        .map_err(Error::IO)
}

pub fn read(cmd: &str, config: &Config) -> Result<String, Error> {
    expression(cmd, config)?.read().map_err(Error::IO)
}

pub fn read_with_env(
    cmd: &str,
    env: &HashMap<String, String>,
    config: &Config,
) -> Result<String, Error> {
    expression_with_env(cmd, env, config)?
        .read()
        .map_err(Error::IO)
}

pub fn read_with_dir(
    cmd: &str,
    config: &Config,
    workdir: impl AsRef<Path>,
) -> Result<String, Error> {
    expression(cmd, config)?
        .dir(workdir.as_ref())
        .read()
        .map_err(Error::IO)
}

fn expression(cmd: &str, config: &Config) -> Result<duct::Expression, Error> {
    let env = setup(config)?;
    expression_with_env(cmd, &env, config)
}

fn expression_with_env(
    cmd: &str,
    env: &HashMap<String, String>,
    config: &Config,
) -> Result<duct::Expression, Error> {
    let mut exp = cmd!("bash", "-c", cmd).full_env(env);

    if let Some(b) = config.get_bool("cmd.silent") {
        if *b {
            exp = exp.stdout_null().stderr_null()
        }
    }

    Ok(exp)
}

fn setup(config: &Config) -> Result<HashMap<String, String>, Error> {
    let mut envs = config.get_envs_from_tables();

    ensure_env_var(&envs, "AWS_PROFILE")?;
    ensure_env_var(&envs, "AWS_DEFAULT_REGION")?;
    ensure_env_var_or_default(&mut envs, "AWS_PAGER", "");

    Ok(envs)
}

fn ensure_env_var(envs: &HashMap<String, String>, key: impl AsRef<str>) -> Result<(), Error> {
    envs.contains_key(key.as_ref())
        .then_some(())
        .ok_or(Error::EnvVarMissing {
            key: key.as_ref().to_string(),
        })
}

fn ensure_env_var_or_default(
    envs: &mut HashMap<String, String>,
    key: impl AsRef<str>,
    default: impl AsRef<str>,
) {
    ensure_env_var(envs, key.as_ref())
        .or_else(|_| {
            envs.insert(key.as_ref().to_string(), default.as_ref().to_string());
            Ok::<(), Error>(())
        })
        .unwrap()
}
