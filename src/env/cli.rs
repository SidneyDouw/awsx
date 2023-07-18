use crate::config::Config;
use anyhow::Result;
use std::{collections::HashMap, path::PathBuf};

pub fn substitute_env_vars(file: PathBuf, _output: Option<PathBuf>, config: &Config) -> Result<()> {
    let mut filestring = std::fs::read_to_string(file)?;
    let env_vars = config
        .get_envs()
        .into_iter()
        .chain(std::env::vars())
        .collect::<HashMap<_, _>>();

    filestring.clone().rmatch_indices("{{").for_each(|(i, _)| {
        let offset = filestring[i..].find("}}").expect("braces not closed") + 2;

        let trim_chars: &[_] = &['{', '}', '$', ' '];
        let env_var = filestring[i..i + offset].trim_matches(trim_chars);

        let env_var_value = env_vars
            .get(env_var)
            .unwrap_or_else(|| panic!("environment variable not set: {}", env_var));

        filestring.replace_range(i..i + offset, env_var_value)
    });

    println!("{}", filestring);

    Ok(())
}

pub fn print_env_vars(config: &crate::config::Config, password: Option<impl AsRef<str>>) -> Result<()> {
    let config_envs = config.get_envs();
    let keys: Vec<_> = config_envs.clone().into_keys().collect();

    println!("[env]");
    config_envs
        .into_iter()
        .chain(std::env::vars())
        .filter(|(k, _)| keys.contains(k))
        .collect::<HashMap<_, _>>()
        .iter()
        .for_each(|(k, v)| println!("{}\t{}", k, v));

    if let Some(password) = password {
        println!("[secrets]");
        let secrets = config.get_secrets(&password);
        secrets.iter().for_each(|(k, v)| println!("{}\t{}", k, v));
    }

    Ok(())
}
