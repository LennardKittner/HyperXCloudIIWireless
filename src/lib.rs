use hidapi::{HidApi, HidDevice, HidError};
use thiserror::Error;

// Possible vendor IDs [hyperx , HP]
const VENDOR_IDS: [u16; 2] = [0x0951, 0x03F0];
// Possible Cloud II Wireless product IDs
const PRODUCT_IDS: [u16; 2] = [0x1718, 0x018B];

const BATTERY_PACKET: [u8; 20] = {
    let mut packet = [0; 20];
    (packet[0], packet[1], packet[2], packet[3]) = (0x06, 0xff, 0xbb, 0x02);
    packet
};

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Error: {0}")]
    HidError(#[from] HidError),
    #[error("Error: No device found.")]
    NoDeviceFound(),
    #[error("Error: No response. Is the headset turned on?")]
    HeadSetOff(),
}
   
pub struct Device {
    hid_device: HidDevice,
}

impl Device {
    pub fn new() -> Result<Self, DeviceError> {
        let hid_api = HidApi::new()?;
        let hid_device = hid_api.device_list().find_map(|info| {
            if PRODUCT_IDS.contains(&info.product_id()) && VENDOR_IDS.contains(&info.vendor_id()) {
                Some(hid_api.open(info.vendor_id(), info.product_id()))
            } else {
                None
            }
        }).ok_or(DeviceError::NoDeviceFound())??;
        Ok(Device { hid_device })
    }

    //TODO: implement charging
    pub fn get_battery_level(&self) -> Result<(u8, bool), DeviceError> {
        self.hid_device.write(&BATTERY_PACKET)?;
        let mut buf = [0u8; 8];
        let res = self.hid_device.read_timeout(&mut buf[..], 1000)?;
        if res == 0 {
            return Err(DeviceError::HeadSetOff());
        }
        println!("Read: {:?}", &buf[..res]);
        Ok((buf[7], false))
    }
}