use std::path::PathBuf;

// pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn fixture_path(fixture_name: &str) -> PathBuf {
    PathBuf::from_iter(["tests", "fixtures", fixture_name])
}
