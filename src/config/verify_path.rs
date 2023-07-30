use std::{
    fs::metadata,
    path::{Path, PathBuf},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Path contains invalid unicode: \"{path}\"")]
    Unicode { path: PathBuf },

    #[error("Could not canonicalize path: \"{path}\"\n{source}")]
    Canonicalize {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Could not get metadata for path: \"{path}\"\n  {source}")]
    Metadata {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Config path does not point to a file: \"{path}\"")]
    NotAFile { path: PathBuf },
}

/// Absolute path to an existing config file that does not containt any non-unicode characters
#[derive(PartialEq)]
pub(crate) struct VerifiedPath(pub(crate) PathBuf);

impl VerifiedPath {
    /// Gets the filename component of a path and returns it as a [String]
    pub(crate) fn get_file_name(&self) -> String {
        self.0
            .file_name()
            .expect("verified path does not end in ..")
            .to_str()
            .expect("verified path has valid unicode")
            .to_owned()
    }

    fn verify_path(path: impl AsRef<Path>) -> Result<VerifiedPath, Error> {
        let path = path.as_ref();
        if path.to_str().is_none() {
            return Err(Error::Unicode {
                path: path.to_owned(),
            });
        }

        let path = path.canonicalize().map_err(|err| Error::Canonicalize {
            path: path.to_owned(),
            source: err,
        })?;

        let m = metadata(&path).map_err(|err| Error::Metadata {
            path: path.to_owned(),
            source: err,
        })?;

        if !m.is_file() {
            return Err(Error::NotAFile { path });
        }

        Ok(VerifiedPath(path))
    }
}

impl TryFrom<PathBuf> for VerifiedPath {
    type Error = Error;
    /// Checks that the file at the given path exists and gives back a canonicalized version of it.
    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        Self::verify_path(path)
    }
}

impl TryFrom<&Path> for VerifiedPath {
    type Error = Error;
    /// Checks that the file at the given path exists and gives back a canonicalized version of it.
    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        Self::verify_path(path)
    }
}
