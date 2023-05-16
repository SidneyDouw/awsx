use crate::config::Config;
use anyhow::Result;
use std::{collections::HashMap, path::PathBuf};

pub fn substitute_env_vars(file: PathBuf, _output: Option<PathBuf>, config: &Config) -> Result<()> {
    let mut filestring = std::fs::read_to_string(file)?;
    let env_vars = std::env::vars()
        .chain(config.get_envs())
        .collect::<HashMap<_, _>>();

    filestring.clone().rmatch_indices("{{").for_each(|(i, _)| {
        let offset = filestring[i..].find("}}").expect("braces not closed") + 2;

        let trim_chars: &[_] = &['{', '}', '$', ' '];
        let env_var = (&filestring[i..i + offset]).trim_matches(trim_chars);

        let env_var_value = env_vars
            .get(env_var)
            .expect(&format!("environment variable not set: {}", env_var));

        filestring.replace_range(i..i + offset, env_var_value)
    });

    println!("{}", filestring);

    Ok(())
}

pub fn print_env_vars(config: &crate::config::Config) -> Result<()> {
    let envs = config.get_envs();
    let keys: Vec<_> = envs.clone().into_iter().map(|(k, _)| k).collect();

    std::env::vars()
        .chain(envs)
        .filter(|(k, _)| keys.contains(k))
        .for_each(|(k, v)| println!("{}\t{}", k, v));

    Ok(())
}
