// Generated by sonos-docs, do not edit.

use rupnp::{Device, Service};
use rupnp::http::Uri;
use rupnp::ssdp::URN;use crate::sonos::gen::errors::Error;

/// Sonos DevicePropertiesService
///
/// Modify device properties, like LED status and stereo pairs
#[derive(Debug)]
pub struct DevicePropertiesService {
    service: Service,
    url: Uri,
}

impl DevicePropertiesService {
    /// Create a new DevicePropertiesService instance from an existing UPnP device.
    pub async fn from_device(device: Device) -> Option<Self> {
        let urn = "urn:schemas-upnp-org:service:DeviceProperties:1".parse::<URN>().unwrap();
        if let Some(s) = device.find_service(&urn) {
            Some(Self{ service: s.clone(), url: device.url().clone() })
        } else {
            None
        }
    }

    /// AddBondedZones
    ///
    /// Parameters:
    /// * `channel_map_set`
    pub async fn add_bonded_zones(
            &self,
            channel_map_set: String
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<ChannelMapSet>{}</ChannelMapSet>", channel_map_set).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// AddHTSatellite
    ///
    /// Adds satellites and/or a sub woofer to a (main) player. The satellites become hidden. The main player RINCON_* is mandatory. RR: right - rear, LF: left - front, SW: subwoofer
    ///
    /// Note: Not all speakers support satellites or sub woofer. Satellites should be of same type (e.g. Play:1)
    ///
    /// Parameters:
    /// * `ht_sat_chan_map_set` : example: `RINCON_000PPP1400:LF,RF;RINCON_000RRR1400:RR;RINCON_000SSS1400:LR;RINCON_000QQQ1400:SW`
    pub async fn add_htsatellite(
            &self,
            ht_sat_chan_map_set: String
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<HTSatChanMapSet>{}</HTSatChanMapSet>", ht_sat_chan_map_set).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// CreateStereoPair
    ///
    /// Create a stereo pair (left, right speakers), right one becomes hidden
    ///
    /// Note: Not all speakers support StereoPairs
    ///
    /// Parameters:
    /// * `channel_map_set` : example: `RINCON_B8E9375831C001400:LF,LF;RINCON_000E58FE3AEA01400:RF,RF`
    pub async fn create_stereo_pair(
            &self,
            channel_map_set: String
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<ChannelMapSet>{}</ChannelMapSet>", channel_map_set).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// EnterConfigMode
    ///
    /// Parameters:
    /// * `mode`
    /// * `options`
    ///
    /// Outputs:
    /// * `state`
    pub async fn enter_config_mode(
            &self,
            mode: String,
            options: String
        ) -> Result<EnterConfigModeResult, Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<Mode>{}</Mode>", mode).as_str());
        payload.push_str(format!("<Options>{}</Options>", options).as_str());
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(EnterConfigModeResult {
            state: response.get("State").ok_or_else(|| Error::MissingField("State".to_string()))?
                .parse().map_err(|_| Error::ParseError("State".to_string()))?,
        })
    }

    /// ExitConfigMode
    ///
    /// Parameters:
    /// * `options`
    pub async fn exit_config_mode(
            &self,
            options: String
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<Options>{}</Options>", options).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// GetAutoplayLinkedZones
    ///
    /// Parameters:
    /// * `source`
    ///
    /// Outputs:
    /// * `include_linked_zones`
    pub async fn get_autoplay_linked_zones(
            &self,
            source: String
        ) -> Result<GetAutoplayLinkedZonesResult, Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<Source>{}</Source>", source).as_str());
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetAutoplayLinkedZonesResult {
            include_linked_zones: response.get("IncludeLinkedZones").ok_or_else(|| Error::MissingField("IncludeLinkedZones".to_string()))?
                .parse().map_err(|_| Error::ParseError("IncludeLinkedZones".to_string()))?,
        })
    }

    /// GetAutoplayRoomUUID
    ///
    /// Parameters:
    /// * `source`
    ///
    /// Outputs:
    /// * `room_uuid`
    pub async fn get_autoplay_room_uuid(
            &self,
            source: String
        ) -> Result<GetAutoplayRoomUUIDResult, Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<Source>{}</Source>", source).as_str());
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetAutoplayRoomUUIDResult {
            room_uuid: response.get("RoomUUID").ok_or_else(|| Error::MissingField("RoomUUID".to_string()))?
                .parse().map_err(|_| Error::ParseError("RoomUUID".to_string()))?,
        })
    }

    /// GetAutoplayVolume
    ///
    /// Parameters:
    /// * `source`
    ///
    /// Outputs:
    /// * `current_volume`
    pub async fn get_autoplay_volume(
            &self,
            source: String
        ) -> Result<GetAutoplayVolumeResult, Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<Source>{}</Source>", source).as_str());
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetAutoplayVolumeResult {
            current_volume: response.get("CurrentVolume").ok_or_else(|| Error::MissingField("CurrentVolume".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentVolume".to_string()))?,
        })
    }

    /// GetButtonLockState
    ///
    /// Get the current button lock state
    ///
    /// Outputs:
    /// * `current_button_lock_state`
    pub async fn get_button_lock_state(
            &self,
        ) -> Result<GetButtonLockStateResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetButtonLockStateResult {
            current_button_lock_state: response.get("CurrentButtonLockState").ok_or_else(|| Error::MissingField("CurrentButtonLockState".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentButtonLockState".to_string()))?,
        })
    }

    /// GetButtonState
    ///
    /// Outputs:
    /// * `state`
    pub async fn get_button_state(
            &self,
        ) -> Result<GetButtonStateResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetButtonStateResult {
            state: response.get("State").ok_or_else(|| Error::MissingField("State".to_string()))?
                .parse().map_err(|_| Error::ParseError("State".to_string()))?,
        })
    }

    /// GetHouseholdID
    ///
    /// Outputs:
    /// * `current_household_id`
    pub async fn get_household_id(
            &self,
        ) -> Result<GetHouseholdIDResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetHouseholdIDResult {
            current_household_id: response.get("CurrentHouseholdID").ok_or_else(|| Error::MissingField("CurrentHouseholdID".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentHouseholdID".to_string()))?,
        })
    }

    /// GetHTForwardState
    ///
    /// Outputs:
    /// * `is_htforward_enabled`
    pub async fn get_htforward_state(
            &self,
        ) -> Result<GetHTForwardStateResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetHTForwardStateResult {
            is_htforward_enabled: response.get("IsHTForwardEnabled").ok_or_else(|| Error::MissingField("IsHTForwardEnabled".to_string()))?
                .parse().map_err(|_| Error::ParseError("IsHTForwardEnabled".to_string()))?,
        })
    }

    /// GetLEDState
    ///
    /// Get the current LED state
    ///
    /// Outputs:
    /// * `current_ledstate`
    pub async fn get_ledstate(
            &self,
        ) -> Result<GetLEDStateResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetLEDStateResult {
            current_ledstate: response.get("CurrentLEDState").ok_or_else(|| Error::MissingField("CurrentLEDState".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentLEDState".to_string()))?,
        })
    }

    /// GetUseAutoplayVolume
    ///
    /// Parameters:
    /// * `source`
    ///
    /// Outputs:
    /// * `use_volume`
    pub async fn get_use_autoplay_volume(
            &self,
            source: String
        ) -> Result<GetUseAutoplayVolumeResult, Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<Source>{}</Source>", source).as_str());
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetUseAutoplayVolumeResult {
            use_volume: response.get("UseVolume").ok_or_else(|| Error::MissingField("UseVolume".to_string()))?
                .parse().map_err(|_| Error::ParseError("UseVolume".to_string()))?,
        })
    }

    /// GetZoneAttributes
    ///
    /// Outputs:
    /// * `current_zone_name`
    /// * `current_icon`
    /// * `current_configuration`
    /// * `current_target_room_name`
    pub async fn get_zone_attributes(
            &self,
        ) -> Result<GetZoneAttributesResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetZoneAttributesResult {
            current_zone_name: response.get("CurrentZoneName").ok_or_else(|| Error::MissingField("CurrentZoneName".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentZoneName".to_string()))?,
            current_icon: response.get("CurrentIcon").ok_or_else(|| Error::MissingField("CurrentIcon".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentIcon".to_string()))?,
            current_configuration: response.get("CurrentConfiguration").ok_or_else(|| Error::MissingField("CurrentConfiguration".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentConfiguration".to_string()))?,
            current_target_room_name: response.get("CurrentTargetRoomName").ok_or_else(|| Error::MissingField("CurrentTargetRoomName".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentTargetRoomName".to_string()))?,
        })
    }

    /// GetZoneInfo
    ///
    /// Get information about this specific speaker
    ///
    /// Outputs:
    /// * `serial_number`
    /// * `software_version`
    /// * `display_software_version`
    /// * `hardware_version`
    /// * `ip_address`
    /// * `mac_address`
    /// * `copyright_info`
    /// * `extra_info`
    /// * `ht_audio_in` : SPDIF input, `0` not connected / `2` stereo / `7` Dolby 2.0 / `18` dolby 5.1 / `21` not listening / `22` silence
    /// * `flags`
    pub async fn get_zone_info(
            &self,
        ) -> Result<GetZoneInfoResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetZoneInfoResult {
            serial_number: response.get("SerialNumber").ok_or_else(|| Error::MissingField("SerialNumber".to_string()))?
                .parse().map_err(|_| Error::ParseError("SerialNumber".to_string()))?,
            software_version: response.get("SoftwareVersion").ok_or_else(|| Error::MissingField("SoftwareVersion".to_string()))?
                .parse().map_err(|_| Error::ParseError("SoftwareVersion".to_string()))?,
            display_software_version: response.get("DisplaySoftwareVersion").ok_or_else(|| Error::MissingField("DisplaySoftwareVersion".to_string()))?
                .parse().map_err(|_| Error::ParseError("DisplaySoftwareVersion".to_string()))?,
            hardware_version: response.get("HardwareVersion").ok_or_else(|| Error::MissingField("HardwareVersion".to_string()))?
                .parse().map_err(|_| Error::ParseError("HardwareVersion".to_string()))?,
            ip_address: response.get("IPAddress").ok_or_else(|| Error::MissingField("IPAddress".to_string()))?
                .parse().map_err(|_| Error::ParseError("IPAddress".to_string()))?,
            mac_address: response.get("MACAddress").ok_or_else(|| Error::MissingField("MACAddress".to_string()))?
                .parse().map_err(|_| Error::ParseError("MACAddress".to_string()))?,
            copyright_info: response.get("CopyrightInfo").ok_or_else(|| Error::MissingField("CopyrightInfo".to_string()))?
                .parse().map_err(|_| Error::ParseError("CopyrightInfo".to_string()))?,
            extra_info: response.get("ExtraInfo").ok_or_else(|| Error::MissingField("ExtraInfo".to_string()))?
                .parse().map_err(|_| Error::ParseError("ExtraInfo".to_string()))?,
            ht_audio_in: response.get("HTAudioIn").ok_or_else(|| Error::MissingField("HTAudioIn".to_string()))?
                .parse().map_err(|_| Error::ParseError("HTAudioIn".to_string()))?,
            flags: response.get("Flags").ok_or_else(|| Error::MissingField("Flags".to_string()))?
                .parse().map_err(|_| Error::ParseError("Flags".to_string()))?,
        })
    }

    /// RemoveBondedZones
    ///
    /// Parameters:
    /// * `channel_map_set`
    /// * `keep_grouped`
    pub async fn remove_bonded_zones(
            &self,
            channel_map_set: String,
            keep_grouped: bool
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<ChannelMapSet>{}</ChannelMapSet>", channel_map_set).as_str());
        payload.push_str(format!("<KeepGrouped>{}</KeepGrouped>", keep_grouped).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// RemoveHTSatellite
    ///
    /// Removes a satellite or a sub woofer from (main) player. The satellite becomes visible.
    ///
    /// Note: Not all speakers support satellites or sub woofer. Multiples RINCON_* are not allowed.
    ///
    /// Parameters:
    /// * `sat_room_uuid` : example: `RINCON_000RRR1400`
    pub async fn remove_htsatellite(
            &self,
            sat_room_uuid: String
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<SatRoomUUID>{}</SatRoomUUID>", sat_room_uuid).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// RoomDetectionStartChirping
    ///
    /// Parameters:
    /// * `channel`
    /// * `duration_milliseconds`
    /// * `chirp_if_playing_swappable_audio`
    ///
    /// Outputs:
    /// * `play_id`
    pub async fn room_detection_start_chirping(
            &self,
            channel: u16,
            duration_milliseconds: u32,
            chirp_if_playing_swappable_audio: bool
        ) -> Result<RoomDetectionStartChirpingResult, Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<Channel>{}</Channel>", channel).as_str());
        payload.push_str(format!("<DurationMilliseconds>{}</DurationMilliseconds>", duration_milliseconds).as_str());
        payload.push_str(format!("<ChirpIfPlayingSwappableAudio>{}</ChirpIfPlayingSwappableAudio>", chirp_if_playing_swappable_audio).as_str());
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(RoomDetectionStartChirpingResult {
            play_id: response.get("PlayId").ok_or_else(|| Error::MissingField("PlayId".to_string()))?
                .parse().map_err(|_| Error::ParseError("PlayId".to_string()))?,
        })
    }

    /// RoomDetectionStopChirping
    ///
    /// Parameters:
    /// * `play_id`
    pub async fn room_detection_stop_chirping(
            &self,
            play_id: u32
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<PlayId>{}</PlayId>", play_id).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// SeparateStereoPair
    ///
    /// Separate a stereo pair
    ///
    /// Note: Not all speakers support StereoPairs
    ///
    /// Parameters:
    /// * `channel_map_set` : example: `RINCON_B8E9375831C001400:LF,LF;RINCON_000E58FE3AEA01400:RF,RF`
    pub async fn separate_stereo_pair(
            &self,
            channel_map_set: String
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<ChannelMapSet>{}</ChannelMapSet>", channel_map_set).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// SetAutoplayLinkedZones
    ///
    /// Parameters:
    /// * `include_linked_zones`
    /// * `source`
    pub async fn set_autoplay_linked_zones(
            &self,
            include_linked_zones: bool,
            source: String
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<IncludeLinkedZones>{}</IncludeLinkedZones>", include_linked_zones).as_str());
        payload.push_str(format!("<Source>{}</Source>", source).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// SetAutoplayRoomUUID
    ///
    /// Parameters:
    /// * `room_uuid`
    /// * `source`
    pub async fn set_autoplay_room_uuid(
            &self,
            room_uuid: String,
            source: String
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<RoomUUID>{}</RoomUUID>", room_uuid).as_str());
        payload.push_str(format!("<Source>{}</Source>", source).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// SetAutoplayVolume
    ///
    /// Parameters:
    /// * `volume`
    /// * `source`
    pub async fn set_autoplay_volume(
            &self,
            volume: u16,
            source: String
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<Volume>{}</Volume>", volume).as_str());
        payload.push_str(format!("<Source>{}</Source>", source).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// SetButtonLockState
    ///
    /// Set the button lock state
    ///
    /// Parameters:
    /// * `desired_button_lock_state`
    pub async fn set_button_lock_state(
            &self,
            desired_button_lock_state: String
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<DesiredButtonLockState>{}</DesiredButtonLockState>", desired_button_lock_state).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// SetLEDState
    ///
    /// Set the LED state
    ///
    /// Parameters:
    /// * `desired_ledstate`
    pub async fn set_ledstate(
            &self,
            desired_ledstate: String
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<DesiredLEDState>{}</DesiredLEDState>", desired_ledstate).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// SetUseAutoplayVolume
    ///
    /// Parameters:
    /// * `use_volume`
    /// * `source`
    pub async fn set_use_autoplay_volume(
            &self,
            use_volume: bool,
            source: String
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<UseVolume>{}</UseVolume>", use_volume).as_str());
        payload.push_str(format!("<Source>{}</Source>", source).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// SetZoneAttributes
    ///
    /// Parameters:
    /// * `desired_zone_name`
    /// * `desired_icon`
    /// * `desired_configuration`
    /// * `desired_target_room_name`
    pub async fn set_zone_attributes(
            &self,
            desired_zone_name: String,
            desired_icon: String,
            desired_configuration: String,
            desired_target_room_name: String
        ) -> Result<(), Error> {
        // TODO: use xml helper
        let mut payload = String::new();
        payload.push_str(format!("<DesiredZoneName>{}</DesiredZoneName>", desired_zone_name).as_str());
        payload.push_str(format!("<DesiredIcon>{}</DesiredIcon>", desired_icon).as_str());
        payload.push_str(format!("<DesiredConfiguration>{}</DesiredConfiguration>", desired_configuration).as_str());
        payload.push_str(format!("<DesiredTargetRoomName>{}</DesiredTargetRoomName>", desired_target_room_name).as_str());
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct EnterConfigModeResult {
    pub state: String,
}

#[derive(Debug)]
pub struct GetAutoplayLinkedZonesResult {
    pub include_linked_zones: bool,
}

#[derive(Debug)]
pub struct GetAutoplayRoomUUIDResult {
    pub room_uuid: String,
}

#[derive(Debug)]
pub struct GetAutoplayVolumeResult {
    pub current_volume: u16,
}

#[derive(Debug)]
pub struct GetButtonLockStateResult {
    pub current_button_lock_state: String,
}

#[derive(Debug)]
pub struct GetButtonStateResult {
    pub state: String,
}

#[derive(Debug)]
pub struct GetHouseholdIDResult {
    pub current_household_id: String,
}

#[derive(Debug)]
pub struct GetHTForwardStateResult {
    pub is_htforward_enabled: bool,
}

#[derive(Debug)]
pub struct GetLEDStateResult {
    pub current_ledstate: String,
}

#[derive(Debug)]
pub struct GetUseAutoplayVolumeResult {
    pub use_volume: bool,
}

#[derive(Debug)]
pub struct GetZoneAttributesResult {
    pub current_zone_name: String,
    pub current_icon: String,
    pub current_configuration: String,
    pub current_target_room_name: String,
}

#[derive(Debug)]
pub struct GetZoneInfoResult {
    pub serial_number: String,
    pub software_version: String,
    pub display_software_version: String,
    pub hardware_version: String,
    pub ip_address: String,
    pub mac_address: String,
    pub copyright_info: String,
    pub extra_info: String,
    /// SPDIF input, `0` not connected / `2` stereo / `7` Dolby 2.0 / `18` dolby 5.1 / `21` not listening / `22` silence
    pub ht_audio_in: u32,
    pub flags: u32,
}

#[derive(Debug)]
pub struct RoomDetectionStartChirpingResult {
    pub play_id: u32,
}
