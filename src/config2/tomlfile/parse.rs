use toml::{macros::Deserialize, value::Map, Value};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error parsing TOML string: {0}")]
    Parse(#[source] toml::de::Error),

    #[error("Error converting TOML Values: {0}")]
    Convert(#[source] toml::de::Error),
}

pub(crate) struct ParseResult {
    pub(crate) parameters: Map<String, Value>,
    pub(crate) envs: Map<String, Value>,
    pub(crate) secrets_path: Option<String>,
}

pub(crate) fn parse_string(input: &str) -> Result<ParseResult, Error> {
    let root_table = input.parse::<Value>().map_err(Error::Parse)?;
    let mut map = root_table.try_into::<Map<_, _>>().map_err(Error::Convert)?;

    Ok(ParseResult {
        parameters: extract_or_default::<Map<_, _>>(&mut map, "parameters")?,
        envs: extract_or_default::<Map<_, _>>(&mut map, "envs")?,
        secrets_path: extract::<String>(&mut map, "secrets")?,
    })
}

fn extract_or_default<'a, T: Default + Deserialize<'a>>(
    map: &mut Map<String, Value>,
    key: &str,
) -> Result<T, Error> {
    Ok(extract(map, key)?.unwrap_or_default())
}

fn extract<'a, T: Default + Deserialize<'a>>(
    map: &mut Map<String, Value>,
    key: &str,
) -> Result<Option<T>, Error> {
    map.remove(key)
        .map(|p| p.try_into::<T>())
        .transpose()
        .map_err(Error::Convert)
}
