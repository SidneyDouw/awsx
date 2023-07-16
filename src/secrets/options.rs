use std::path::PathBuf;

#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    /// Encrypts a file with the given password
    Encrypt {
        /// Path to file to encrpyt
        file: PathBuf,

        /// Password to encrypt the file with
        #[clap(long, short = 'p')]
        password: String,
    },

    /// Decrypts a file using the given password
    Decrypt {
        /// Path to file to decrpyt
        file: PathBuf,

        /// Password to decrypt the file with
        #[clap(long, short = 'p')]
        password: String,
    },
}
