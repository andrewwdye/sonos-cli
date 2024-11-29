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
    pub async fn state(&self) -> Result<(), Box<dyn std::error::Error>> {
        let service = AVTransportService::from_ip(self.ip_address).await?;

        println!("Device capabilities: {:?}", service.get_device_capabilities(0).await?);
        println!("Transport info: {:?}", service.get_transport_info(0).await?);
        println!("Media info: {:?}", service.get_media_info(0).await?);

        Ok(())
    }
}
