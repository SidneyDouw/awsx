use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error while loading the config file at {:?}\n\t{:?}", path, msg)]
    Load { path: String, msg: String },
}

#[cfg(not(tarpaulin_include))]
impl Error {
    pub fn load_error(path: impl AsRef<Path>, msg: &str) -> Error {
        Error::Load {
            path: path
                .as_ref()
                .to_str()
                .expect("invalid utf-8 in path")
                .to_string(),
            msg: msg.to_string(),
        }
    }
}
