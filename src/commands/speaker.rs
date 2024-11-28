use clap::{Parser, Subcommand};
use std::net::IpAddr;
use sonos_cli::sonos::gen::services::av_transport::AVTransportService;

#[derive(Parser)]
pub struct Speaker {
    /// IP address of the speaker
    ip_address: IpAddr,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get the current state of the speaker
    State,
}

impl Speaker {
    pub async fn state(&self) {
        println!("Speaker state for {}", self.ip_address);
        let s = AVTransportService::from_ip(self.ip_address).await;
    }
}
