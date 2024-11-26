#[derive(Debug)]
pub struct Device {
    _upnp_device: rupnp::Device,
}

impl Device {
    pub fn new(upnp_device: rupnp::Device) -> Self {
        Self { _upnp_device: upnp_device }
    }

    pub fn name(&self) -> &str {
        self._upnp_device.friendly_name()
    }
}
