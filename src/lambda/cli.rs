use crate::{cmd::run, config::Config};
use anyhow::Result;
use std::path::Path;

pub fn update_function(
    function_name: String,
    zip_file: impl AsRef<Path>,
    config: &Config,
) -> Result<()> {
    let cmd = format!(
        "aws lambda update-function-code --function-name {} --zip-file fileb://{}",
        function_name,
        zip_file.as_ref().to_string_lossy()
    );

    run(&cmd, config)?;
    Ok(())
}
