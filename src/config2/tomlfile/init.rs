use super::{
    parse::{parse_string, ParseResult},
    traits::TomlFileInit,
    ConfigFile, Error, SecretsFile,
};
use crate::config2::verify_path::VerifiedPath;

impl TomlFileInit for ConfigFile {
    fn from_path(path: &VerifiedPath) -> Result<Self, Error> {
        let toml_string = std::fs::read_to_string(&path.0)?;
        let ParseResult {
            parameters,
            envs,
            secrets_path,
        } = parse_string(&toml_string)?;

        let secrets = secrets_path
            .map(|secrets_path| path.0.join("..").join(secrets_path))
            .map(VerifiedPath::try_from)
            .transpose()?
            .map(|secrets_path| {
                if secrets_path == *path {
                    Err(Error::SelfReference)
                } else {
                    SecretsFile::from_path(&secrets_path)
                }
            })
            .transpose()?;

        Ok(Self {
            parameters,
            envs,
            secrets,
        })
    }
}

impl TomlFileInit for SecretsFile {
    fn from_path(path: &VerifiedPath) -> Result<Self, Error> {
        let toml_string = std::fs::read_to_string(&path.0)?;
        let ParseResult {
            parameters,
            envs,
            secrets_path,
        } = parse_string(&toml_string)?;

        if secrets_path.is_some() {
            return Err(Error::SecretReference);
        }

        Ok(Self { parameters, envs })
    }
}
