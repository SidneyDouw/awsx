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
    let env = setup(config)?;
    expression(cmd, &env, config)
        .run()
        .map(|_| ())
        .map_err(Error::IO)
}

pub fn read(cmd: &str, config: &Config) -> Result<String, Error> {
    let env = setup(config)?;
    expression(cmd, &env, config).read().map_err(Error::IO)
}

pub fn read_with_env(
    cmd: &str,
    env: &HashMap<String, String>,
    config: &Config,
) -> Result<String, Error> {
    expression(cmd, env, config).read().map_err(Error::IO)
}

pub fn read_with_dir_and_env(
    cmd: &str,
    workdir: impl AsRef<Path>,
    env: &HashMap<String, String>,
    config: &Config,
) -> Result<String, Error> {
    expression(cmd, env, config)
        .dir(workdir.as_ref())
        .read()
        .map_err(Error::IO)
}

pub fn read_with_dir(
    cmd: &str,
    workdir: impl AsRef<Path>,
    config: &Config,
) -> Result<String, Error> {
    let env = setup(config)?;
    expression(cmd, &env, config)
        .dir(workdir.as_ref())
        .read()
        .map_err(Error::IO)
}

/// Sets up a duct expression from the given `cmd` parameter, sets its environment from
/// the given `env` parameter and configures it with settings found in the given `config` parameter
fn expression(cmd: &str, env: &HashMap<String, String>, config: &Config) -> duct::Expression {
    let mut exp = cmd!("bash", "-c", cmd).full_env(env);

    if let Some(b) = config.get_bool("cmd.silent") {
        if *b {
            exp = exp.stdout_null().stderr_null()
        }
    }

    exp
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
