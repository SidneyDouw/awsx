use crate::config::Config;
use duct::cmd;
use std::{collections::HashMap, path::Path};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(
        "Could not find required environment variable: {:?} in config files",
        key
    )]
    EnvVarMissing { key: String },

    // TODO: why not cmd::Error?
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

pub fn run(cmd: &str, config: &Config) -> Result<(), Error> {
    let env = get_envs_with_config_envs(config)?;
    expression(cmd, &env).run().map(|_| ()).map_err(Error::IO)
}

pub fn read(cmd: &str, config: &Config) -> Result<String, Error> {
    let env = get_envs_with_config_envs(config)?;
    expression(cmd, &env).read().map_err(Error::IO)
}

pub fn read_with_dir_and_env(
    cmd: &str,
    workdir: impl AsRef<Path>,
    env: &HashMap<String, String>,
) -> Result<String, Error> {
    expression(cmd, env)
        .dir(workdir.as_ref())
        .read()
        .map_err(Error::IO)
}

/// Sets up a duct expression from the given `cmd` parameter, sets its environment from
/// the given `env` parameter
// and configures it with settings found in the given `config` parameter
fn expression(cmd: &str, env: &HashMap<String, String>) -> duct::Expression {
    cmd!("bash", "-c", cmd).full_env(env)

    // if let Some(b) = config.get_bool("cmd.silent") {
    //     if *b {
    //         exp = exp.stdout_null().stderr_null()
    //     }
    // }
}

fn get_envs_with_config_envs(config: &Config) -> Result<HashMap<String, String>, Error> {
    let mut config_envs = config.get_envs().map_err(|e| match e {
        crate::config::Error::Expression(e) => e,
    })?;

    ensure_env_var(&config_envs, "AWS_PROFILE")?;
    ensure_env_var(&config_envs, "AWS_DEFAULT_REGION")?;
    ensure_env_var_or_default(&mut config_envs, "AWS_PAGER", "");

    let all_envs = config_envs
        .into_iter()
        .chain(std::env::vars())
        .collect::<HashMap<_, _>>();

    Ok(all_envs)
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
