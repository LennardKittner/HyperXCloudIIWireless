use hyper_x_cloud_ii_wireless::{Device, DeviceError};
use ksni::TrayService;
mod battery_tray;
use battery_tray::BatteryTray;

fn pair_device() -> Device {
    loop {
        match Device::new() {
            Ok(device) => break device,
            Err(error) => {
                eprintln!("{error}");
            }
        };
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn main() {
    let service = TrayService::new(BatteryTray::new());
    let handle = service.handle();
    service.spawn();

    let mut device = pair_device();

    // Run loop
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let (battery_level, charging) = match device.get_battery_level() {
            Ok(t) => t,
            Err(DeviceError::HidError(hidapi::HidError::HidApiError { message })) => {
                eprintln!("Error: {message}");
                if message == "No such device" {
                    handle.update(|tray: &mut BatteryTray| { tray.no_device_found(); });
                    device = pair_device();
                }
                continue;
            }
            Err(DeviceError::NoDeviceFound()) => {
                eprintln!("{}", DeviceError::NoDeviceFound());
                handle.update(|tray: &mut BatteryTray| { tray.no_device_found(); });
                continue;
            }
            Err(DeviceError::HeadSetOff()) => {
                eprintln!("{}", DeviceError::HeadSetOff());
                handle.update(|tray: &mut BatteryTray| { tray.no_device_found(); });
                continue;
            }
            Err(error) => {
                eprintln!("{error}");
                continue;
            }
        };
        handle.update(|tray: &mut BatteryTray| { tray.update(battery_level, charging); });
    }
}
