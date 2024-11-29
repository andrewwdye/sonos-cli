// Generated by sonos-docs, do not edit.

use rupnp::{Device, Service};
use rupnp::http::Uri;
use serde_xml_rs;
use std::net::IpAddr;
use crate::sonos::gen::errors::Error;

/// Sonos AlarmClockService
///
/// Control the sonos alarms and times
#[derive(Debug)]
pub struct AlarmClockService {
    service: Service,
    url: Uri,
}

impl AlarmClockService {
    /// Create a new AlarmClockService instance from an existing UPnP device.
    pub async fn from_device(device: Device) -> Result<Self, Error> {
        let urn = "urn:schemas-upnp-org:service:AlarmClock:1".parse().unwrap();
        let service = device.find_service(&urn)
            .ok_or_else(|| Error::ServiceNotFound("AlarmClockService".to_string()))?;
        Ok(Self{ service: service.clone(), url: device.url().clone() })
    }

    /// Create a new AlarmClockService instance from an IP address.
    pub async fn from_ip(ip: IpAddr) -> Result<Self, Error> {
        let url = format!("http://{ip}:1400/xml/device_description.xml").parse().unwrap();
        let device = Device::from_url(url).await?;
        Self::from_device(device).await
    }

    /// CreateAlarm
    ///
    /// Create a single alarm, all properties are required
    ///
    /// Parameters:
    /// * `start_local_time` : The start time as `hh:mm:ss`
    /// * `duration` : The duration as `hh:mm:ss`
    /// * `recurrence` : Repeat this alarm on
    /// * `enabled` : Alarm enabled after creation
    /// * `room_uuid` : The UUID of the speaker you want this alarm for
    /// * `program_uri` : The sound uri
    /// * `program_meta_data` : The sound metadata, can be empty string
    /// * `play_mode` : Alarm play mode
    /// * `volume` : Volume between 0 and 100
    /// * `include_linked_zones` : Should grouped players also play the alarm?
    ///
    /// Outputs:
    /// * `assigned_id` : The ID of the new alarm
    pub async fn create_alarm(
            &self,
            start_local_time: String,
            duration: String,
            recurrence: String,
            enabled: bool,
            room_uuid: String,
            program_uri: String,
            program_meta_data: String,
            play_mode: String,
            volume: u16,
            include_linked_zones: bool
        ) -> Result<CreateAlarmResult, Error> {
        let payload = [
            serde_xml_rs::to_string(&start_local_time).unwrap(),
            serde_xml_rs::to_string(&duration).unwrap(),
            serde_xml_rs::to_string(&recurrence).unwrap(),
            serde_xml_rs::to_string(&enabled).unwrap(),
            serde_xml_rs::to_string(&room_uuid).unwrap(),
            serde_xml_rs::to_string(&program_uri).unwrap(),
            serde_xml_rs::to_string(&program_meta_data).unwrap(),
            serde_xml_rs::to_string(&play_mode).unwrap(),
            serde_xml_rs::to_string(&volume).unwrap(),
            serde_xml_rs::to_string(&include_linked_zones).unwrap(),
        ].concat();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(CreateAlarmResult {
            assigned_id: response.get("AssignedID").ok_or_else(|| Error::MissingField("AssignedID".to_string()))?
                .parse().map_err(|_| Error::ParseError("AssignedID".to_string()))?,
        })
    }

    /// DestroyAlarm
    ///
    /// Delete an alarm
    ///
    /// Parameters:
    /// * `id` : The Alarm ID from ListAlarms
    pub async fn destroy_alarm(
            &self,
            id: u32
        ) -> Result<(), Error> {
        let payload = [
            serde_xml_rs::to_string(&id).unwrap(),
        ].concat();
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// GetDailyIndexRefreshTime
    ///
    /// Outputs:
    /// * `current_daily_index_refresh_time`
    pub async fn get_daily_index_refresh_time(
            &self,
        ) -> Result<GetDailyIndexRefreshTimeResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetDailyIndexRefreshTimeResult {
            current_daily_index_refresh_time: response.get("CurrentDailyIndexRefreshTime").ok_or_else(|| Error::MissingField("CurrentDailyIndexRefreshTime".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentDailyIndexRefreshTime".to_string()))?,
        })
    }

    /// GetFormat
    ///
    /// Outputs:
    /// * `current_time_format`
    /// * `current_date_format`
    pub async fn get_format(
            &self,
        ) -> Result<GetFormatResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetFormatResult {
            current_time_format: response.get("CurrentTimeFormat").ok_or_else(|| Error::MissingField("CurrentTimeFormat".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentTimeFormat".to_string()))?,
            current_date_format: response.get("CurrentDateFormat").ok_or_else(|| Error::MissingField("CurrentDateFormat".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentDateFormat".to_string()))?,
        })
    }

    /// GetHouseholdTimeAtStamp
    ///
    /// Parameters:
    /// * `time_stamp`
    ///
    /// Outputs:
    /// * `household_utctime`
    pub async fn get_household_time_at_stamp(
            &self,
            time_stamp: String
        ) -> Result<GetHouseholdTimeAtStampResult, Error> {
        let payload = [
            serde_xml_rs::to_string(&time_stamp).unwrap(),
        ].concat();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetHouseholdTimeAtStampResult {
            household_utctime: response.get("HouseholdUTCTime").ok_or_else(|| Error::MissingField("HouseholdUTCTime".to_string()))?
                .parse().map_err(|_| Error::ParseError("HouseholdUTCTime".to_string()))?,
        })
    }

    /// GetTimeNow
    ///
    /// Outputs:
    /// * `current_utctime`
    /// * `current_local_time`
    /// * `current_time_zone`
    /// * `current_time_generation`
    pub async fn get_time_now(
            &self,
        ) -> Result<GetTimeNowResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetTimeNowResult {
            current_utctime: response.get("CurrentUTCTime").ok_or_else(|| Error::MissingField("CurrentUTCTime".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentUTCTime".to_string()))?,
            current_local_time: response.get("CurrentLocalTime").ok_or_else(|| Error::MissingField("CurrentLocalTime".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentLocalTime".to_string()))?,
            current_time_zone: response.get("CurrentTimeZone").ok_or_else(|| Error::MissingField("CurrentTimeZone".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentTimeZone".to_string()))?,
            current_time_generation: response.get("CurrentTimeGeneration").ok_or_else(|| Error::MissingField("CurrentTimeGeneration".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentTimeGeneration".to_string()))?,
        })
    }

    /// GetTimeServer
    ///
    /// Outputs:
    /// * `current_time_server`
    pub async fn get_time_server(
            &self,
        ) -> Result<GetTimeServerResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetTimeServerResult {
            current_time_server: response.get("CurrentTimeServer").ok_or_else(|| Error::MissingField("CurrentTimeServer".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentTimeServer".to_string()))?,
        })
    }

    /// GetTimeZone
    ///
    /// Outputs:
    /// * `index`
    /// * `auto_adjust_dst`
    pub async fn get_time_zone(
            &self,
        ) -> Result<GetTimeZoneResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetTimeZoneResult {
            index: response.get("Index").ok_or_else(|| Error::MissingField("Index".to_string()))?
                .parse().map_err(|_| Error::ParseError("Index".to_string()))?,
            auto_adjust_dst: response.get("AutoAdjustDst").ok_or_else(|| Error::MissingField("AutoAdjustDst".to_string()))?
                .parse().map_err(|_| Error::ParseError("AutoAdjustDst".to_string()))?,
        })
    }

    /// GetTimeZoneAndRule
    ///
    /// Outputs:
    /// * `index`
    /// * `auto_adjust_dst`
    /// * `current_time_zone`
    pub async fn get_time_zone_and_rule(
            &self,
        ) -> Result<GetTimeZoneAndRuleResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetTimeZoneAndRuleResult {
            index: response.get("Index").ok_or_else(|| Error::MissingField("Index".to_string()))?
                .parse().map_err(|_| Error::ParseError("Index".to_string()))?,
            auto_adjust_dst: response.get("AutoAdjustDst").ok_or_else(|| Error::MissingField("AutoAdjustDst".to_string()))?
                .parse().map_err(|_| Error::ParseError("AutoAdjustDst".to_string()))?,
            current_time_zone: response.get("CurrentTimeZone").ok_or_else(|| Error::MissingField("CurrentTimeZone".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentTimeZone".to_string()))?,
        })
    }

    /// GetTimeZoneRule
    ///
    /// Parameters:
    /// * `index`
    ///
    /// Outputs:
    /// * `time_zone`
    pub async fn get_time_zone_rule(
            &self,
            index: i32
        ) -> Result<GetTimeZoneRuleResult, Error> {
        let payload = [
            serde_xml_rs::to_string(&index).unwrap(),
        ].concat();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(GetTimeZoneRuleResult {
            time_zone: response.get("TimeZone").ok_or_else(|| Error::MissingField("TimeZone".to_string()))?
                .parse().map_err(|_| Error::ParseError("TimeZone".to_string()))?,
        })
    }

    /// ListAlarms
    ///
    /// Get the AlarmList as XML
    ///
    /// Note: Some libraries also provide a ListAndParseAlarms where the alarm list xml is parsed
    ///
    /// Outputs:
    /// * `current_alarm_list` : xml string, see remarks
    /// * `current_alarm_list_version`
    pub async fn list_alarms(
            &self,
        ) -> Result<ListAlarmsResult, Error> {
        let payload = String::new();
        let response = self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        // TODO: map parse errors
        Ok(ListAlarmsResult {
            current_alarm_list: response.get("CurrentAlarmList").ok_or_else(|| Error::MissingField("CurrentAlarmList".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentAlarmList".to_string()))?,
            current_alarm_list_version: response.get("CurrentAlarmListVersion").ok_or_else(|| Error::MissingField("CurrentAlarmListVersion".to_string()))?
                .parse().map_err(|_| Error::ParseError("CurrentAlarmListVersion".to_string()))?,
        })
    }

    /// SetDailyIndexRefreshTime
    ///
    /// Parameters:
    /// * `desired_daily_index_refresh_time`
    pub async fn set_daily_index_refresh_time(
            &self,
            desired_daily_index_refresh_time: String
        ) -> Result<(), Error> {
        let payload = [
            serde_xml_rs::to_string(&desired_daily_index_refresh_time).unwrap(),
        ].concat();
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// SetFormat
    ///
    /// Parameters:
    /// * `desired_time_format`
    /// * `desired_date_format`
    pub async fn set_format(
            &self,
            desired_time_format: String,
            desired_date_format: String
        ) -> Result<(), Error> {
        let payload = [
            serde_xml_rs::to_string(&desired_time_format).unwrap(),
            serde_xml_rs::to_string(&desired_date_format).unwrap(),
        ].concat();
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// SetTimeNow
    ///
    /// Parameters:
    /// * `desired_time`
    /// * `time_zone_for_desired_time`
    pub async fn set_time_now(
            &self,
            desired_time: String,
            time_zone_for_desired_time: String
        ) -> Result<(), Error> {
        let payload = [
            serde_xml_rs::to_string(&desired_time).unwrap(),
            serde_xml_rs::to_string(&time_zone_for_desired_time).unwrap(),
        ].concat();
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// SetTimeServer
    ///
    /// Parameters:
    /// * `desired_time_server`
    pub async fn set_time_server(
            &self,
            desired_time_server: String
        ) -> Result<(), Error> {
        let payload = [
            serde_xml_rs::to_string(&desired_time_server).unwrap(),
        ].concat();
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// SetTimeZone
    ///
    /// Parameters:
    /// * `index`
    /// * `auto_adjust_dst`
    pub async fn set_time_zone(
            &self,
            index: i32,
            auto_adjust_dst: bool
        ) -> Result<(), Error> {
        let payload = [
            serde_xml_rs::to_string(&index).unwrap(),
            serde_xml_rs::to_string(&auto_adjust_dst).unwrap(),
        ].concat();
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }

    /// UpdateAlarm
    ///
    /// Update an alarm, all parameters are required.
    ///
    /// Note: Some libraries support PatchAlarm where you can update a single parameter
    ///
    /// Parameters:
    /// * `id` : The ID of the alarm see ListAlarms
    /// * `start_local_time` : The start time as `hh:mm:ss`
    /// * `duration` : The duration as `hh:mm:ss`
    /// * `recurrence` : Repeat this alarm on
    /// * `enabled` : Alarm enabled after creation
    /// * `room_uuid` : The UUID of the speaker you want this alarm for
    /// * `program_uri` : The sound uri
    /// * `program_meta_data` : The sound metadata, can be empty string
    /// * `play_mode` : Alarm play mode
    /// * `volume` : Volume between 0 and 100
    /// * `include_linked_zones` : Should grouped players also play the alarm?
    pub async fn update_alarm(
            &self,
            id: u32,
            start_local_time: String,
            duration: String,
            recurrence: String,
            enabled: bool,
            room_uuid: String,
            program_uri: String,
            program_meta_data: String,
            play_mode: String,
            volume: u16,
            include_linked_zones: bool
        ) -> Result<(), Error> {
        let payload = [
            serde_xml_rs::to_string(&id).unwrap(),
            serde_xml_rs::to_string(&start_local_time).unwrap(),
            serde_xml_rs::to_string(&duration).unwrap(),
            serde_xml_rs::to_string(&recurrence).unwrap(),
            serde_xml_rs::to_string(&enabled).unwrap(),
            serde_xml_rs::to_string(&room_uuid).unwrap(),
            serde_xml_rs::to_string(&program_uri).unwrap(),
            serde_xml_rs::to_string(&program_meta_data).unwrap(),
            serde_xml_rs::to_string(&play_mode).unwrap(),
            serde_xml_rs::to_string(&volume).unwrap(),
            serde_xml_rs::to_string(&include_linked_zones).unwrap(),
        ].concat();
        self.service.action(&self.url, "SetTimeNow", payload.as_str()).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct CreateAlarmResult {
    /// The ID of the new alarm
    pub assigned_id: u32,
}

#[derive(Debug)]
pub struct GetDailyIndexRefreshTimeResult {
    pub current_daily_index_refresh_time: String,
}

#[derive(Debug)]
pub struct GetFormatResult {
    pub current_time_format: String,
    pub current_date_format: String,
}

#[derive(Debug)]
pub struct GetHouseholdTimeAtStampResult {
    pub household_utctime: String,
}

#[derive(Debug)]
pub struct GetTimeNowResult {
    pub current_utctime: String,
    pub current_local_time: String,
    pub current_time_zone: String,
    pub current_time_generation: u32,
}

#[derive(Debug)]
pub struct GetTimeServerResult {
    pub current_time_server: String,
}

#[derive(Debug)]
pub struct GetTimeZoneResult {
    pub index: i32,
    pub auto_adjust_dst: bool,
}

#[derive(Debug)]
pub struct GetTimeZoneAndRuleResult {
    pub index: i32,
    pub auto_adjust_dst: bool,
    pub current_time_zone: String,
}

#[derive(Debug)]
pub struct GetTimeZoneRuleResult {
    pub time_zone: String,
}

#[derive(Debug)]
pub struct ListAlarmsResult {
    /// xml string, see remarks
    pub current_alarm_list: String,
    pub current_alarm_list_version: String,
}
