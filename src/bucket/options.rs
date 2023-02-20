/// Commands that control S3 related tasks
#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    /// Checks if the given bucket exists
    Exists { bucket_name: String },
}
