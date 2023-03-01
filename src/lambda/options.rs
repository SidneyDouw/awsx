use std::path::PathBuf;

/// Commands that control EC2 related tasks
#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    UpdateFunction {
        function_name: String,
        zip_file: PathBuf,
    },
}
