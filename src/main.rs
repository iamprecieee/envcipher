use colored::Colorize;
use envcipher::cli;

fn main() {
    if let Err(e) = cli::execute(std::env::args()) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
