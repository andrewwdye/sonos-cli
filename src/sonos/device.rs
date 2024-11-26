use super::upnp;

#[derive(Debug)]
pub struct Device {
    upnp_device: rupnp::Device,
}

impl Device {
    pub fn new(upnp_device: rupnp::Device) -> Self {
        Self { upnp_device }
    }

    pub fn name(&self) -> &str {
        if let Some(device) = self.upnp_device.find_device(&upnp::MEDIA_RENDERER_URN) {
            device.friendly_name()
        } else {
            self.upnp_device.friendly_name()
        }
    }

    pub fn host(&self) -> Option<&str> {
        self.upnp_device.url().host()
    }
}
