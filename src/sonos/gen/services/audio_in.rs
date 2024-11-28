// Generated by sonos-docs, do not edit.

use rupnp::{Device, Service};
use rupnp::http::Uri;
use rupnp::ssdp::URN;use crate::sonos::gen::errors::Error;
use serde_xml_rs;

/// Sonos AudioInService
///
/// Control line in
#[derive(Debug)]
pub struct AudioInService {
    service: Service,
    url: Uri,
}

impl AudioInService {
    /// Create a new AudioInService instance from an existing UPnP device.
    pub async fn from_device(device: Device) -> Option<Self> {
        let urn = "urn:schemas-upnp-org:service:AudioIn:1".parse::<URN>().unwrap();
        if let Some(s) = device.find_service(&urn) {
            Some(Self{ service: s.clone(), url: device.url().clone() })
        } else {
            None
        }
    }

    /// GetAudioInputAttributes
    ///
    /// Outputs:
    /// * `current_name`
    /// * `current_icon`
    pub async fn get_audio_input_attributes(
            &self,
        ) -> Result<GetAudioInputAttributesResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetAudioInputAttributesResult {
            current_name: response.get("CurrentName").ok_or_else(|| Error::MissingField("CurrentName".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentName".to_string()))?,
            current_icon: response.get("CurrentIcon").ok_or_else(|| Error::MissingField("CurrentIcon".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentIcon".to_string()))?,
        })
    }

    /// GetLineInLevel
    ///
    /// Outputs:
    /// * `current_left_line_in_level`
    /// * `current_right_line_in_level`
    pub async fn get_line_in_level(
            &self,
        ) -> Result<GetLineInLevelResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetLineInLevelResult {
            current_left_line_in_level: response.get("CurrentLeftLineInLevel").ok_or_else(|| Error::MissingField("CurrentLeftLineInLevel".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentLeftLineInLevel".to_string()))?,
            current_right_line_in_level: response.get("CurrentRightLineInLevel").ok_or_else(|| Error::MissingField("CurrentRightLineInLevel".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentRightLineInLevel".to_string()))?,
        })
    }

    /// SelectAudio
    ///
    /// Parameters:
    /// * `object_id`
    pub async fn select_audio(
            &self,
            object_id: String
        ) -> Result<(), Error> {
        let payload = [
            serde_xml_rs::to_string(&object_id).unwrap(),
        ].concat();
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// SetAudioInputAttributes
    ///
    /// Parameters:
    /// * `desired_name`
    /// * `desired_icon`
    pub async fn set_audio_input_attributes(
            &self,
            desired_name: String,
            desired_icon: String
        ) -> Result<(), Error> {
        let payload = [
            serde_xml_rs::to_string(&desired_name).unwrap(),
            serde_xml_rs::to_string(&desired_icon).unwrap(),
        ].concat();
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// SetLineInLevel
    ///
    /// Parameters:
    /// * `desired_left_line_in_level`
    /// * `desired_right_line_in_level`
    pub async fn set_line_in_level(
            &self,
            desired_left_line_in_level: i32,
            desired_right_line_in_level: i32
        ) -> Result<(), Error> {
        let payload = [
            serde_xml_rs::to_string(&desired_left_line_in_level).unwrap(),
            serde_xml_rs::to_string(&desired_right_line_in_level).unwrap(),
        ].concat();
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// StartTransmissionToGroup
    ///
    /// Parameters:
    /// * `coordinator_id`
    ///
    /// Outputs:
    /// * `current_transport_settings`
    pub async fn start_transmission_to_group(
            &self,
            coordinator_id: String
        ) -> Result<StartTransmissionToGroupResult, Error> {
        let payload = [
            serde_xml_rs::to_string(&coordinator_id).unwrap(),
        ].concat();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(StartTransmissionToGroupResult {
            current_transport_settings: response.get("CurrentTransportSettings").ok_or_else(|| Error::MissingField("CurrentTransportSettings".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentTransportSettings".to_string()))?,
        })
    }

    /// StopTransmissionToGroup
    ///
    /// Parameters:
    /// * `coordinator_id`
    pub async fn stop_transmission_to_group(
            &self,
            coordinator_id: String
        ) -> Result<(), Error> {
        let payload = [
            serde_xml_rs::to_string(&coordinator_id).unwrap(),
        ].concat();
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct GetAudioInputAttributesResult {
    pub current_name: String,
    pub current_icon: String,
}

#[derive(Debug)]
pub struct GetLineInLevelResult {
    pub current_left_line_in_level: i32,
    pub current_right_line_in_level: i32,
}

#[derive(Debug)]
pub struct StartTransmissionToGroupResult {
    pub current_transport_settings: String,
}

