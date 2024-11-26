use futures::{pin_mut, TryStreamExt};
use log::trace;
use rupnp::ssdp::URN;
use std::time::Duration;
use crate::sonos::device::Device;
use crate::sonos::errors::Error;

const SONOS_URN: URN = URN::device("schemas-upnp-org", "ZonePlayer", 1);

pub async fn discover() -> Result<Vec<Device>, Error> {
    let devices = rupnp::discover(&SONOS_URN.into(), Duration::from_secs(3))
        .await?;
    pin_mut!(devices);
    let mut sonos_devices = Vec::new();
    while let Some(device) = devices.try_next().await? {
        trace!("{:?}", device);
        sonos_devices.push(Device::new(device));
    }
    Ok(sonos_devices)
}
