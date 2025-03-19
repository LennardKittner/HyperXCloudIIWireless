pub mod cloud_ii_wireless_dts;

use hidapi::{HidApi, HidDevice, HidError};
use std::{fmt::Display, time::Duration};
use thistermination::TerminationFull;

//TODO: connect to rest of code base

#[derive(Debug)]
pub struct DeviceState {
    hid_device: HidDevice,
    pub battery_level: Option<u8>,
    pub charging: Option<ChargingStatus>,
    pub muted: Option<bool>,
    pub mic_connected: Option<bool>,
    pub automatic_shutdown_after: Option<Duration>,
    pub pairing_info: Option<u8>,
    pub product_color: Option<Color>,
    pub side_tone_on: Option<bool>,
    pub side_tone_volume: Option<u8>,
    pub voice_prompt_on: Option<bool>,
    pub connected: Option<bool>,
}

impl Display for DeviceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unknown = "Unknown".to_string();
        write!(
            f,
            "
            Battery level:            {}%
            Carging status:           {}
            Muted:                    {}
            Mic connected:            {}
            Automatic shutdown after: {}min
            Pairing info:             {}
            Product color:            {}
            Side tone on:             {}
            Side tone volume:         {}
            Voice prompt on:          {}
            Connected:                {}
        ",
            self.battery_level
                .map_or(unknown.clone(), |l| l.to_string()),
            self.charging.map_or(unknown.clone(), |c| c.to_string()),
            self.muted.map_or(unknown.clone(), |m| m.to_string()),
            self.mic_connected
                .map_or(unknown.clone(), |m| m.to_string()),
            self.automatic_shutdown_after
                .map_or(unknown.clone(), |a| (a.as_secs() / 60).to_string()),
            self.pairing_info.map_or(unknown.clone(), |p| p.to_string()),
            self.product_color
                .map_or(unknown.clone(), |c| c.to_string()),
            self.side_tone_on.map_or(unknown.clone(), |s| s.to_string()),
            self.side_tone_volume
                .map_or(unknown.clone(), |s| s.to_string()),
            self.voice_prompt_on
                .map_or(unknown.clone(), |v| v.to_string()),
            self.connected.map_or(unknown.clone(), |c| c.to_string()),
        )
    }
}

impl DeviceState {
    pub fn new(product_ids: &[u16], vendor_ids: &[u16]) -> Result<Self, DeviceError> {
        let hid_api = HidApi::new()?;
        let hid_device = hid_api
            .device_list()
            .find_map(|info| {
                if product_ids.contains(&info.product_id())
                    && vendor_ids.contains(&info.vendor_id())
                {
                    Some(hid_api.open(info.vendor_id(), info.product_id()))
                } else {
                    None
                }
            })
            .ok_or(DeviceError::NoDeviceFound())??;
        Ok(DeviceState {
            hid_device,
            charging: None,
            battery_level: None,
            muted: None,
            mic_connected: None,
            automatic_shutdown_after: None,
            pairing_info: None,
            product_color: None,
            side_tone_on: None,
            side_tone_volume: None,
            voice_prompt_on: None,
            connected: None,
        })
    }

    fn update_self_with_event(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::BatterLevel(level) => self.battery_level = Some(*level),
            DeviceEvent::Charging(status) => self.charging = Some(*status),
            DeviceEvent::Muted(status) => self.muted = Some(*status),
            DeviceEvent::MicConnected(status) => self.mic_connected = Some(*status),
            DeviceEvent::AutomaticShutdownAfter(duration) => {
                self.automatic_shutdown_after = Some(*duration)
            }
            DeviceEvent::PairingInfo(info) => self.pairing_info = Some(*info),
            DeviceEvent::ProductColor(color) => self.product_color = Some(*color),
            DeviceEvent::SideToneOn(side) => self.side_tone_on = Some(*side),
            DeviceEvent::SideToneVolume(volume) => self.side_tone_volume = Some(*volume),
            DeviceEvent::VoicePrompt(on) => self.voice_prompt_on = Some(*on),
            DeviceEvent::WirelessConnected(connected) => self.connected = Some(*connected),
        };
    }

    pub fn clear_state(&mut self) {
        self.charging = None;
        self.battery_level = None;
        self.muted = None;
        self.mic_connected = None;
    }
}

#[derive(TerminationFull)]
pub enum DeviceError {
    #[termination(msg("{0:?}"))]
    HidError(#[from] HidError),
    #[termination(msg("No device found."))]
    NoDeviceFound(),
    #[termination(msg("No response. Is the headset turned on?"))]
    HeadSetOff(),
    #[termination(msg("No response."))]
    NoResponse(),
    #[termination(msg("Unknown response: {0:?} with length: {1:?}"))]
    UnknownResponse([u8; 8], usize),
}

#[derive(Debug, Copy, Clone)]
pub enum DeviceEvent {
    BatterLevel(u8),
    Muted(bool),
    MicConnected(bool),
    Charging(ChargingStatus),
    AutomaticShutdownAfter(Duration),
    PairingInfo(u8),
    ProductColor(Color),
    SideToneOn(bool),
    SideToneVolume(u8),
    VoicePrompt(bool),
    WirelessConnected(bool),
}

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Red,
    UnknownColor(u8),
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::Red => "Red".to_string(),
                Color::UnknownColor(n) => format!("Unknown color {}", n),
            }
        )
    }
}

impl From<u8> for Color {
    fn from(color: u8) -> Self {
        match color {
            0 => Color::Red,
            _ => Color::UnknownColor(color),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ChargingStatus {
    NotCharging,
    Charging,
    FullyCharged,
    ChargeError,
}

impl Display for ChargingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ChargingStatus::NotCharging => "Not charging",
                ChargingStatus::Charging => "Charging",
                ChargingStatus::FullyCharged => "Fully charged",
                ChargingStatus::ChargeError => "Charging error!",
            }
        )
    }
}

impl From<u8> for ChargingStatus {
    fn from(value: u8) -> ChargingStatus {
        match value {
            0 => ChargingStatus::NotCharging,
            1 => ChargingStatus::Charging,
            2 => ChargingStatus::FullyCharged,
            _ => ChargingStatus::ChargeError,
        }
    }
}

pub trait Device {
    fn get_charging_packet(&self) -> Vec<u8>;
    fn get_battery_packet(&self) -> Vec<u8>;
    fn set_automatic_shut_down_packet(&self, shutdown_after: Duration) -> Vec<u8>;
    fn get_automatic_shut_down_packet(&self) -> Vec<u8>;
    fn get_mute_packet(&self) -> Vec<u8>;
    fn set_mute_packet(&self, mute: bool) -> Vec<u8>;
    fn get_mic_connected_packet(&self) -> Vec<u8>;
    fn get_pairing_info_packet(&self) -> Vec<u8>;
    fn get_product_color_packet(&self) -> Vec<u8>;
    fn get_side_tone_packet(&self) -> Vec<u8>;
    fn set_side_tone_packet(&self, side_tone_on: bool) -> Vec<u8>;
    fn get_side_tone_volume_packet(&self) -> Vec<u8>;
    fn set_side_tone_volume_packet(&self) -> Vec<u8>;
    fn get_voice_prompt_packet(&self) -> Vec<u8>;
    fn set_voice_prompt_packet(&self, enable: bool) -> Vec<u8>;
    fn get_wireless_connected_status_packet(&self) -> Vec<u8>;
    fn get_event_from_device_response(&self, response: &[u8]) -> Option<DeviceEvent>;
    fn get_device_state(&mut self) -> &mut DeviceState;

    fn wait_for_updates(&mut self, duration: Duration) -> Option<DeviceEvent> {
        let mut buf = [0u8; 8];
        let res = self
            .get_device_state()
            .hid_device
            .read_timeout(&mut buf[..], duration.as_millis() as i32)
            .ok()?;

        if res == 0 {
            return None;
        }

        self.get_event_from_device_response(&buf[0..res])
    }

    fn refresh_state(&mut self) -> Result<(), DeviceError> {
        let packets = vec![
            self.get_wireless_connected_status_packet(),
            self.get_charging_packet(),
            self.get_battery_packet(),
            self.get_automatic_shut_down_packet(),
            self.get_mute_packet(),
            self.get_mic_connected_packet(),
            self.get_pairing_info_packet(),
            // self.get_product_color_packet(),
            self.get_side_tone_packet(),
            self.get_side_tone_volume_packet(),
            // self.get_voice_prompt_packet(),
        ];

        let mut responded = false;
        for packet in packets {
            self.get_device_state().connected = None;
            self.get_device_state().hid_device.write(&packet)?;
            if let Some(event) = self.wait_for_updates(Duration::from_secs(1)) {
                self.get_device_state().update_self_with_event(&event);
                responded = true;
            }
            if !self.get_device_state().connected.map_or(true, |c| c) {
                break;
            }
        }

        if responded {
            Ok(())
        } else {
            Err(DeviceError::NoResponse())
        }
    }
}
