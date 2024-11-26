// Generated by sonos-docs, do not edit.

use rupnp::{Device, Service};
use rupnp::http::Uri;
use rupnp::ssdp::URN;use crate::sonos::gen::errors::Error;

/// Sonos ZoneGroupTopologyService
///
/// Zone config stuff, eg getting all the configured sonos zones
#[derive(Debug)]
pub struct ZoneGroupTopologyService {
    service: Service,
    url: Uri,
}

impl ZoneGroupTopologyService {
    /// Create a new ZoneGroupTopologyService instance from an existing UPnP device.
    pub async fn from_device(device: Device) -> Option<Self> {
        let urn = "urn:schemas-upnp-org:service:ZoneGroupTopology:1".parse::<URN>().unwrap();
        if let Some(s) = device.find_service(&urn) {
            Some(Self{ service: s.clone(), url: device.url().clone() })
        } else {
            None
        }
    }

    /// BeginSoftwareUpdate
    ///
    /// Parameters:
    /// * `update_url`
    /// * `flags`
    /// * `extra_options`
    pub async fn begin_software_update(
            &self,
            update_url: String,
            flags: u32,
            extra_options: String
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<UpdateURL>{}</UpdateURL>", update_url).as_str());
        payload.push_str(format!("<Flags>{}</Flags>", flags).as_str());
        payload.push_str(format!("<ExtraOptions>{}</ExtraOptions>", extra_options).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// CheckForUpdate
    ///
    /// Parameters:
    /// * `update_type`
    /// * `cached_only`
    /// * `version`
    ///
    /// Outputs:
    /// * `update_item`
    pub async fn check_for_update(
            &self,
            update_type: String,
            cached_only: bool,
            version: String
        ) -> Result<CheckForUpdateResult, Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<UpdateType>{}</UpdateType>", update_type).as_str());
        payload.push_str(format!("<CachedOnly>{}</CachedOnly>", cached_only).as_str());
        payload.push_str(format!("<Version>{}</Version>", version).as_str());
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(CheckForUpdateResult {
            update_item: response.get("UpdateItem").ok_or_else(|| Error::MissingField("UpdateItem".to_string()))?
                .parse().map_err(|_| Error::ParseError("UpdateItem".to_string()))?,
        })
    }

    /// GetZoneGroupAttributes
    ///
    /// Get information about the current Zone
    ///
    /// Outputs:
    /// * `current_zone_group_name`
    /// * `current_zone_group_id`
    /// * `current_zone_player_uuids_in_group`
    /// * `current_muse_household_id`
    pub async fn get_zone_group_attributes(
            &self,
        ) -> Result<GetZoneGroupAttributesResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetZoneGroupAttributesResult {
            current_zone_group_name: response.get("CurrentZoneGroupName").ok_or_else(|| Error::MissingField("CurrentZoneGroupName".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentZoneGroupName".to_string()))?,
            current_zone_group_id: response.get("CurrentZoneGroupID").ok_or_else(|| Error::MissingField("CurrentZoneGroupID".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentZoneGroupID".to_string()))?,
            current_zone_player_uuids_in_group: response.get("CurrentZonePlayerUUIDsInGroup").ok_or_else(|| Error::MissingField("CurrentZonePlayerUUIDsInGroup".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentZonePlayerUUIDsInGroup".to_string()))?,
            current_muse_household_id: response.get("CurrentMuseHouseholdId").ok_or_else(|| Error::MissingField("CurrentMuseHouseholdId".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentMuseHouseholdId".to_string()))?,
        })
    }

    /// GetZoneGroupState
    ///
    /// Get all the Sonos groups, (as XML)
    ///
    /// Note: Some libraries also support GetParsedZoneGroupState that parses the xml for you.
    ///
    /// Outputs:
    /// * `zone_group_state` : xml string, see remarks
    pub async fn get_zone_group_state(
            &self,
        ) -> Result<GetZoneGroupStateResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetZoneGroupStateResult {
            zone_group_state: response.get("ZoneGroupState").ok_or_else(|| Error::MissingField("ZoneGroupState".to_string()))?
                .parse().map_err(|_| Error::ParseError("ZoneGroupState".to_string()))?,
        })
    }

    /// RegisterMobileDevice
    ///
    /// Parameters:
    /// * `mobile_device_name`
    /// * `mobile_device_udn`
    /// * `mobile_ipand_port`
    pub async fn register_mobile_device(
            &self,
            mobile_device_name: String,
            mobile_device_udn: String,
            mobile_ipand_port: String
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<MobileDeviceName>{}</MobileDeviceName>", mobile_device_name).as_str());
        payload.push_str(format!("<MobileDeviceUDN>{}</MobileDeviceUDN>", mobile_device_udn).as_str());
        payload.push_str(format!("<MobileIPAndPort>{}</MobileIPAndPort>", mobile_ipand_port).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// ReportAlarmStartedRunning
    pub async fn report_alarm_started_running(
            &self,
        ) -> Result<(), Error> {
        let payload = String::new();
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// ReportUnresponsiveDevice
    ///
    /// Parameters:
    /// * `device_uuid`
    /// * `desired_action`
    pub async fn report_unresponsive_device(
            &self,
            device_uuid: String,
            desired_action: String
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<DeviceUUID>{}</DeviceUUID>", device_uuid).as_str());
        payload.push_str(format!("<DesiredAction>{}</DesiredAction>", desired_action).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// SubmitDiagnostics
    ///
    /// Parameters:
    /// * `include_controllers`
    /// * `type`
    ///
    /// Outputs:
    /// * `diagnostic_id`
    pub async fn submit_diagnostics(
            &self,
            include_controllers: bool,
            type: String
        ) -> Result<SubmitDiagnosticsResult, Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<IncludeControllers>{}</IncludeControllers>", include_controllers).as_str());
        payload.push_str(format!("<Type>{}</Type>", type).as_str());
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(SubmitDiagnosticsResult {
            diagnostic_id: response.get("DiagnosticID").ok_or_else(|| Error::MissingField("DiagnosticID".to_string()))?
                .parse().map_err(|_| Error::ParseError("DiagnosticID".to_string()))?,
        })
    }
}

#[derive(Debug)]
pub struct CheckForUpdateResult {
    pub update_item: String,
}

#[derive(Debug)]
pub struct GetZoneGroupAttributesResult {
    pub current_zone_group_name: String,
    pub current_zone_group_id: String,
    pub current_zone_player_uuids_in_group: String,
    pub current_muse_household_id: String,
}

#[derive(Debug)]
pub struct GetZoneGroupStateResult {
    /// xml string, see remarks
    pub zone_group_state: String,
}

#[derive(Debug)]
pub struct SubmitDiagnosticsResult {
    pub diagnostic_id: u32,
}
