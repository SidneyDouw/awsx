use std::path::PathBuf;

#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    /// Replace all occurences of existing environment variables in a file.
    /// By default the result will be printed to stdout.
    Substitute {
        /// Path to file in which all occurences of existing env vars will be replaced.
        file: PathBuf,

        /// Optionally write the output to a file instead of printing to stdout.
        #[clap(long, short = 'o')]
        output: Option<PathBuf>,
    },

    Print {},
}
