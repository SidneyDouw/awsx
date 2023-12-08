/// Commands that control S3 related tasks
#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    /// Lists hosted zones by name
    Get { name: String, key: String },
}
