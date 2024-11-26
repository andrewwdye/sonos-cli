use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use std::process::ExitCode;
use tokio;

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    verbose: Verbosity,
}

#[derive(Subcommand)]
enum Commands {
    /// Discover Sonos devices on the network
    Discover(commands::discover::Discover),
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();
    match cli.command {
        Some(Commands::Discover(c)) => {
            c.discover().await;
        }
        None => {
            println!("No command provided");
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
