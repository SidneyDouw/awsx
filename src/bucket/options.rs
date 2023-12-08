/// Commands that control S3 related tasks
#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    /// Copies files from a source to a destination
    Cp {
        from: String,
        to: String,
        #[clap(long, short = 'r')]
        recursive: bool,
    },

    /// Removes the given path
    Rm {
        path: String,
        #[clap(long, short = 'r')]
        recursive: bool,
    },

    /// Checks if the given bucket exists
    Exists { bucket_name: String },

    /// Adds a bucket policy to the specified bucket
    PutBucketPolicy { bucket_name: String, policy: String },

    /// Upload and replace
    Upload { path: String, to: String },
}
