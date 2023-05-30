use std::path::PathBuf;

/// Commands that control Cloud Formation related tasks
#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    /// Creates a Cloud Formation stack
    Create {
        /// The name of the stack
        stack_name: String,

        /// Path to a Cloud Formation template file
        #[clap(long, short = 't')]
        template: PathBuf,
    },

    /// Updates a Cloud Formation stack
    Update {
        /// The name of the stack
        stack_name: String,

        /// Path to a Cloud Formation template file
        #[clap(long, short = 't')]
        template: PathBuf,
    },

    /// Destroys a Cloud Formation stack
    #[clap(visible_alias = "delete")]
    Destroy {
        /// The name of the stack
        stack_name: String,
    },

    /// Get output parameters from a Cloud Formation stack
    Output {
        /// The name of the stack
        stack_name: String,

        /// The name of the output variable to get
        output_name: Option<String>,
        // /// Variable to filter for. If none is given, return all.
        // /// Can be used multiple times: i.e. --var "var1" --var "var2"
        // #[clap(long)]
        // var: Vec<String>,
    },

    /// Validates a Cloud Formation template file
    Validate {
        /// Path to a Cloud Formation template file
        template: PathBuf,
    },
}
