use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use std::process::ExitCode;
use tokio;

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    verbose: Verbosity,
}

#[derive(Subcommand)]
enum Commands {
    /// Discover Sonos devices on the network
    Discover(commands::discover::Discover),

    /// Get the state of a Sonos speaker
    Speaker(commands::speaker::Speaker),
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();
    match cli.command {
        Commands::Discover(c) => {
            c.discover().await;
            Ok(())
        }
        Commands::Speaker(s) => {
            s.state().await?;
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
