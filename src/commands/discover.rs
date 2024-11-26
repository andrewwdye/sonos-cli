use clap::Parser;
use sonos_cli::sonos;

#[derive(Parser)]
pub struct Discover {
    #[clap(short, long, value_parser = humantime::parse_duration, default_value = "2s")]
    timeout: std::time::Duration,
}

impl Discover {
    pub async fn discover(&self) {
        match sonos::discover().await {
            Ok(devices) => {
                if devices.len() > 0 {
                    devices.iter().for_each(|device| {
                        println!("{:16}: {}", device.host().unwrap(), device.name());
                        // dbg!(device);
                    });
                } else {
                    println!("No devices found");
                }
            }
            Err(e) => {
                println!("Error discovering devices: {}", e);
            }
        }
    }
}
