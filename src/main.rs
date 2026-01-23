use clap::{Parser, Subcommand};
use colored::Colorize;
use envcipher::{cli, error::Result};

#[derive(Parser)]
#[command(name = "envcipher")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

fn main() {
    let cli = Cli::parse();

    let result: Result<()> = match cli.command {
        Commands::Init => cli::init::run(),
        Commands::Lock => cli::lock::run(),
        Commands::Unlock => cli::unlock::run(),
        Commands::Status => cli::status::run(),
        Commands::Edit => cli::edit::run(),
        Commands::Run { args } => cli::run::run(args),
        Commands::ExportKey => cli::key::export(),
        Commands::ImportKey { key } => cli::key::import(&key),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
