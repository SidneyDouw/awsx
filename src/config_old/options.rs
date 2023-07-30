use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Options {
    /// configures if additional configs in the parent directories should be loaded as well
    pub nested: bool,
    /// manually sets the project root for nested configs
    /// when scanning for additional configs in parent directories, it will not look beyond this
    /// directory
    /// if not specified, it will look for a `.git` folder to determine the project root
    pub project_root: Option<PathBuf>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            nested: true,
            project_root: None,
        }
    }
}

impl Options {
    /// If `self.project_root` is `Some(path)` it will be returned as an absolute path.
    /// If it is `None` it will find the first parent folder containing a `.git`
    /// directory and return that as an absolute path.
    pub fn get_project_root(&self) -> Result<PathBuf, std::io::Error> {
        if let Some(project_root) = self.project_root.clone() {
            std::fs::canonicalize(project_root)
        } else {
            let mut path = std::env::current_dir()?;

            let project_root = loop {
                if let Ok(m) = std::fs::metadata(path.join(".git")) {
                    if m.is_dir() {
                        break path;
                    }
                }
                if path.parent().is_none() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Could not find a .git diretory in any parent folder",
                    ));
                }
                path.pop();
            };

            Ok(project_root)
        }
    }
}
