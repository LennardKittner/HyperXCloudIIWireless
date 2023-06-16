use std::time::Duration;

use hidapi::{HidApi, HidDevice, HidError};
use thiserror::Error;

// Possible vendor IDs [hyperx , HP]
const VENDOR_IDS: [u16; 2] = [0x0951, 0x03F0];
// Possible Cloud II Wireless product IDs
const PRODUCT_IDS: [u16; 2] = [0x1718, 0x018B];

const BATTERY_LEVEL_INDEX: usize = 7;
const CHARGING_PREAMBLE: [u8; 4] = [11, 0 , 187, 2];
const NOW_CHARGING: [u8; 5] = [6, 255, 187, 3, 1];
const STOPPED_CHARGING: [u8; 5] = [6, 255, 187, 3, 0];
const NOW_MUTED: [u8; 5] = [6, 255, 187, 32, 1];
const STOPPED_MUTED: [u8; 5] = [6, 255, 187, 32, 0];
const NOW_MIC_DISCONNECTED: [u8; 5] = [6, 255, 187, 8, 0];
const NOW_MIC_CONNECTED: [u8; 5] = [6, 255, 187, 8, 1];

const BATTERY_PACKET: [u8; 62] = [6, 0, 2, 0, 154, 0, 0, 104, 74, 142, 10, 0, 0, 0, 187, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const BATTERY_PACKET_2: [u8; 20] = [6, 0, 2, 0, 154, 0, 0, 104, 74, 142, 10, 0, 0, 0, 187, 2, 0, 0, 0, 0];

pub enum DeviceEvent {
    BatterLevel(u8),
    NowCharging,
    StoppedCharging,
    NowMuted,
    StoppedMuted,
    NowMicDisconnected,
    NowMicConnected,
}

impl DeviceEvent {
    pub fn get_event_from_buf(buf: &[u8; 8], len: usize) -> Result<Self, DeviceError> {
        if len == 0 {
            return Err(DeviceError::NoResponse());
        }
        if len != 8 {
            return Err(DeviceError::UnknownResponse(buf.clone(), len));
        }
        match buf {
            buf if buf.starts_with(&NOW_CHARGING)      => Ok(Self::NowCharging),
            buf if buf.starts_with(&STOPPED_CHARGING)  => Ok(Self::StoppedCharging),
            buf if buf.starts_with(&CHARGING_PREAMBLE) => Ok(Self::BatterLevel(buf[BATTERY_LEVEL_INDEX])),
            buf if buf.starts_with(&NOW_MUTED)         => Ok(Self::NowMuted),
            buf if buf.starts_with(&STOPPED_MUTED)     => Ok(Self::StoppedMuted),
            buf if buf.starts_with(&NOW_MIC_CONNECTED) => Ok(Self::NowMicConnected),
            buf if buf.starts_with(&NOW_MIC_DISCONNECTED) => Ok(Self::NowMicDisconnected),
            _ => Err(DeviceError::UnknownResponse(buf.clone(), len)),
        }
    }
}

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("{0}")]
    HidError(#[from] HidError),
    #[error("No device found.")]
    NoDeviceFound(),
    #[error("No response. Is the headset turned on?")]
    HeadSetOff(),
    #[error("No response.")]
    NoResponse(),
    #[error("Unknown response: {0:?} with length: {1}")]
    UnknownResponse([u8; 8], usize),
}
   
pub struct Device {
    hid_device: HidDevice,
    pub battery_level: u8,
    pub charging: Option<bool>,
    pub muted: Option<bool>,
    pub mic_connected: Option<bool>,
}

impl Device {
    pub fn new() -> Result<Self, DeviceError> {
        let hid_api = HidApi::new().unwrap();
        let hid_device = hid_api.device_list().find_map(|info| {
            if PRODUCT_IDS.contains(&info.product_id()) && VENDOR_IDS.contains(&info.vendor_id()) {
                Some(hid_api.open(info.vendor_id(), info.product_id()))
            } else {
                None
            }
        }).ok_or(DeviceError::NoDeviceFound()).unwrap().unwrap();
        Ok(Device { 
            hid_device,
            charging: None,
            battery_level: 0,
            muted: None,
            mic_connected: None,
         })
    }

    fn update_self_with_event(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::BatterLevel(level) => self.battery_level = level.clone(),
            DeviceEvent::NowCharging => self.charging = Some(true),
            DeviceEvent::StoppedCharging => self.charging = Some(false),
            DeviceEvent::NowMuted => self.muted = Some(true),
            DeviceEvent::StoppedMuted => self.muted = Some(false),
            DeviceEvent::NowMicDisconnected => self.mic_connected = Some(false),
            DeviceEvent::NowMicConnected => self.mic_connected = Some(true),
        };
    }

    pub fn wait_for_updates(&mut self, duration: Duration) -> Result<DeviceEvent, DeviceError> {
        let mut buf = [0u8; 8];
        let res = self.hid_device.read_timeout(&mut buf[..], duration.as_millis() as i32).unwrap();
       
        match DeviceEvent::get_event_from_buf(&buf, res) {
            Ok(event) => {
                self.update_self_with_event(&event);
                Ok(event)
            },
            Err(error) => Err(error),
        }
    }

    pub fn update_battery_level(&mut self) -> Result<u8, DeviceError> {
        for _ in 0..10 { // loop if other events are currently happening.
            self.hid_device.write(&BATTERY_PACKET).unwrap();
            let mut buf = [0u8; 8];
            let res = self.hid_device.read_timeout(&mut buf[..], 1000).unwrap();
            print!("{:?}", &buf[..res]);
            match DeviceEvent::get_event_from_buf(&buf, res) {
                Ok(DeviceEvent::BatterLevel(level)) => {
                    self.update_self_with_event(&DeviceEvent::BatterLevel(level));
                    return Ok(self.battery_level);
                }
                Ok(event) => self.update_self_with_event(&event),
                Err(DeviceError::NoResponse()) => return Err(DeviceError::HeadSetOff()),
                Err(error) => return Err(error),
            };
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        Err(DeviceError::NoResponse())
    }

    pub fn clear_state(&mut self) {
        self.charging = None;
        self.battery_level = 0;
        self.muted = None;
        self.mic_connected = None;
    }
}