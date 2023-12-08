use crate::{cmd::read, config::Config};
use anyhow::{anyhow, Result};

pub fn get(name: impl AsRef<str>, key: impl AsRef<str>, config: &Config) -> Result<()> {
    let res = read(
        &format!(
            "aws secretsmanager get-secret-value --secret-id {} --output text --query 'SecretString'",
            name.as_ref(),
        ),
        config,
    )?;

    // parse the json
    let json = serde_json::from_str::<serde_json::Value>(&res)?;
    let json_obj = json.as_object().ok_or(anyhow!("not an object"))?;
    let value = json_obj.get(key.as_ref()).ok_or(anyhow!("key not found"))?;

    if let Some(value) = value.as_str() {
        println!("{}", value);
    } else {
        println!("{}", value);
    }

    Ok(())
}
