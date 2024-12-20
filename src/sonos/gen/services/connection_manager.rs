// Generated by sonos-docs, do not edit.

use rupnp::{Device, Service};
use rupnp::http::Uri;
use serde_xml_rs;
use std::net::IpAddr;
use crate::sonos::gen::errors::Error;

/// Sonos ConnectionManagerService
///
/// Services related to connections and protocols
#[derive(Debug)]
pub struct ConnectionManagerService {
    service: Service,
    url: Uri,
}

impl ConnectionManagerService {
    /// Create a new ConnectionManagerService instance from an existing UPnP device.
    pub async fn from_device(device: Device) -> Result<Self, Error> {
        let urn = "urn:schemas-upnp-org:service:ConnectionManager:1".parse().unwrap();
        let service = device.find_service(&urn)
            .ok_or_else(|| Error::ServiceNotFound("ConnectionManagerService".to_string()))?;
        Ok(Self{ service: service.clone(), url: device.url().clone() })
    }

    /// Create a new ConnectionManagerService instance from an IP address.
    pub async fn from_ip(ip: IpAddr) -> Result<Self, Error> {
        let url = format!("http://{ip}:1400/xml/device_description.xml").parse().unwrap();
        let device = Device::from_url(url).await?;
        Self::from_device(device).await
    }

    /// GetCurrentConnectionIDs
    ///
    /// Outputs:
    /// * `connection_ids`
    pub async fn get_current_connection_ids(
            &self,
        ) -> Result<GetCurrentConnectionIDsResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetCurrentConnectionIDsResult {
            connection_ids: response.get("ConnectionIDs").ok_or_else(|| Error::MissingField("ConnectionIDs".to_string()))?
                .parse().map_err(|_| Error::ParseError("ConnectionIDs".to_string()))?,
        })
    }

    /// GetCurrentConnectionInfo
    ///
    /// Parameters:
    /// * `connection_id`
    ///
    /// Outputs:
    /// * `rcs_id`
    /// * `av_transport_id`
    /// * `protocol_info`
    /// * `peer_connection_manager`
    /// * `peer_connection_id`
    /// * `direction`
    /// * `status`
    pub async fn get_current_connection_info(
            &self,
            connection_id: i32
        ) -> Result<GetCurrentConnectionInfoResult, Error> {
        let payload = [
            serde_xml_rs::to_string(&connection_id).unwrap(),
        ].concat();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetCurrentConnectionInfoResult {
            rcs_id: response.get("RcsID").ok_or_else(|| Error::MissingField("RcsID".to_string()))?
                .parse().map_err(|_| Error::ParseError("RcsID".to_string()))?,
            av_transport_id: response.get("AVTransportID").ok_or_else(|| Error::MissingField("AVTransportID".to_string()))?
                .parse().map_err(|_| Error::ParseError("AVTransportID".to_string()))?,
            protocol_info: response.get("ProtocolInfo").ok_or_else(|| Error::MissingField("ProtocolInfo".to_string()))?
                .parse().map_err(|_| Error::ParseError("ProtocolInfo".to_string()))?,
            peer_connection_manager: response.get("PeerConnectionManager").ok_or_else(|| Error::MissingField("PeerConnectionManager".to_string()))?
                .parse().map_err(|_| Error::ParseError("PeerConnectionManager".to_string()))?,
            peer_connection_id: response.get("PeerConnectionID").ok_or_else(|| Error::MissingField("PeerConnectionID".to_string()))?
                .parse().map_err(|_| Error::ParseError("PeerConnectionID".to_string()))?,
            direction: response.get("Direction").ok_or_else(|| Error::MissingField("Direction".to_string()))?
                .parse().map_err(|_| Error::ParseError("Direction".to_string()))?,
            status: response.get("Status").ok_or_else(|| Error::MissingField("Status".to_string()))?
                .parse().map_err(|_| Error::ParseError("Status".to_string()))?,
        })
    }

    /// GetProtocolInfo
    ///
    /// Outputs:
    /// * `source`
    /// * `sink`
    pub async fn get_protocol_info(
            &self,
        ) -> Result<GetProtocolInfoResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetProtocolInfoResult {
            source: response.get("Source").ok_or_else(|| Error::MissingField("Source".to_string()))?
                .parse().map_err(|_| Error::ParseError("Source".to_string()))?,
            sink: response.get("Sink").ok_or_else(|| Error::MissingField("Sink".to_string()))?
                .parse().map_err(|_| Error::ParseError("Sink".to_string()))?,
        })
    }
}

#[derive(Debug)]
pub struct GetCurrentConnectionIDsResult {
    pub connection_ids: String,
}

#[derive(Debug)]
pub struct GetCurrentConnectionInfoResult {
    pub rcs_id: i32,
    pub av_transport_id: i32,
    pub protocol_info: String,
    pub peer_connection_manager: String,
    pub peer_connection_id: i32,
    pub direction: String,
    pub status: String,
}

#[derive(Debug)]
pub struct GetProtocolInfoResult {
    pub source: String,
    pub sink: String,
}
