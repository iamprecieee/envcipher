pub mod edit;
pub mod init;
pub mod key;
pub mod lock;
pub mod run;
pub mod status;
pub mod unlock;

use crate::error::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "envcipher")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize project.
    Init,

    /// Encrypt .env.
    Lock,

    /// Decrypt .env.
    Unlock,

    /// Show status.
    Status,

    /// Edit encrypted .env.
    Edit,

    /// Run command with decrypted env vars.
    Run {
        /// Command to run.
        #[arg(last = true, required = true)]
        args: Vec<String>,
    },

    /// Export key for sharing.
    ExportKey,

    /// Import shared key.
    ImportKey {
        /// Base64 encoded key.
        #[arg(required = true)]
        key: String,
    },
}

pub fn execute<I, T>(args: I) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Init => init::run(),
        Commands::Lock => lock::run(),
        Commands::Unlock => unlock::run(),
        Commands::Status => status::run(),
        Commands::Edit => edit::run(),
        Commands::Run { args } => run::run(args),
        Commands::ExportKey => key::export(),
        Commands::ImportKey { key } => key::import(&key),
    }
}
